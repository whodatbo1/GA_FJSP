use crate::instance::Instance;
use crate::schedule::Schedule;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::cmp::{max, Ordering};
use std::collections::btree_map::BTreeMap;
use std::collections::HashMap;

#[derive(Clone)]
pub struct DecodedOperation {
    machine: i32,
    job: i32,
    product: String,
    operation: i32,
    start: i32,
    duration: i32,
    completion: i32,
}

impl DecodedOperation {
    pub fn new(
        machine: i32,
        job: i32,
        product: String,
        operation: i32,
        start: i32,
        duration: i32,
        completion: i32,
    ) -> DecodedOperation {
        DecodedOperation {
            machine,
            job,
            product,
            operation,
            start,
            duration,
            completion,
        }
    }

    pub fn order_by_job_operation(&self, other: &DecodedOperation) -> Ordering {
        if self.job > other.job {
            return Greater;
        } else if self.job < other.job {
            return Less;
        } else if self.operation > other.operation {
            return Greater;
        } else if self.operation < other.operation {
            return Less;
        }
        return Equal;
    }

    pub fn order_by_start(&self, other: &DecodedOperation) -> Ordering {
        if self.start > other.start {
            return Greater;
        } else if self.start < other.start {
            return Less;
        }
        return Equal;
    }
}

#[derive(Clone)]
pub struct DecodedSchedule {
    operations: Vec<DecodedOperation>,
}

impl DecodedSchedule {
    pub fn new(decoded_operations: Vec<DecodedOperation>) -> DecodedSchedule {
        DecodedSchedule {
            operations: decoded_operations,
        }
    }

    pub fn calculate_makespan(&self) -> i32 {
        let mut max_end = 0;
        for operation in &self.operations {
            if operation.completion > max_end {
                max_end = operation.completion;
            }
        }
        max_end
    }
}

pub fn encode_schedule(instance: &Instance, schedule: &mut DecodedSchedule) -> Schedule {
    schedule.operations.sort_by(|a, b| a.order_by_start(b));

    let v2 = schedule
        .operations
        .clone()
        .into_iter()
        .map(|dec_op| dec_op.job)
        .collect();

    schedule
        .operations
        .sort_by(|a, b| a.order_by_job_operation(b));

    let v1 = schedule
        .operations
        .clone()
        .into_iter()
        .map(|dec_op| dec_op.machine)
        .collect();
    // println!("v1: {:?}, v2: {:?}", v1, v2);
    Schedule::new(instance.instance_num, v1, v2)
}

pub fn decode_simple(schedule: &Schedule, instance: &Instance) -> DecodedSchedule {
    let mut curr_machine_times =
        create_vec_with_length_and_value(instance.instance_constants.nr_machines, 0);
    let mut curr_job_times = create_vec_with_length_and_value(instance.nr_jobs, 0);

    let mut previous_enzyme_per_machine: HashMap<i32, String> = HashMap::new();

    let mut results: Vec<DecodedOperation> = Vec::new();

    let mut op_index = create_vec_with_length_and_value(instance.nr_jobs, 0);

    for job in schedule.v2.clone().into_iter() {
        let operation: i32 = instance.operations[&job][op_index[job as usize] as usize];
        let machine = schedule.v1[(&instance.initial_operation_index_in_job_vector_per_job
            [job as usize]
            + &op_index[job as usize]) as usize];
        op_index[job as usize] += 1;

        let duration = instance.processing_times[&(job, operation, machine)].clone();

        let mut change_over_time = 0;

        let curr_enzyme = instance.orders[&job].product.clone();

        if previous_enzyme_per_machine.contains_key(&machine) {
            let prev_enzyme = previous_enzyme_per_machine[&machine].clone();
            change_over_time = instance.instance_constants.change_overs
                [&(machine, prev_enzyme, curr_enzyme.clone())];
        }

        curr_machine_times[machine as usize] += change_over_time;

        let start = max(
            curr_job_times[job as usize],
            curr_machine_times[machine as usize],
        );

        let end = start + duration;

        previous_enzyme_per_machine.insert(machine, curr_enzyme.clone());

        curr_job_times[job as usize] = max(curr_job_times[job as usize], end);

        curr_machine_times[machine as usize] = max(curr_machine_times[machine as usize], end);

        results.push(DecodedOperation::new(
            machine,
            job,
            curr_enzyme.clone(),
            operation,
            start,
            duration,
            end,
        ));
    }

    DecodedSchedule::new(results)
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct MachineTask {
    start: i32,
    end: i32,
    enzyme: String,
}

impl MachineTask {
    pub fn new(start: i32, end: i32, enzyme: String) -> MachineTask {
        MachineTask { start, end, enzyme }
    }
}

pub fn decode_optimal(schedule: &Schedule, instance: &Instance) -> DecodedSchedule {
    let mut curr_machine_times =
        create_vec_with_length_and_value(instance.instance_constants.nr_machines, 0);
    let mut curr_job_times = create_vec_with_length_and_value(instance.nr_jobs, 0);

    let mut previous_enzyme_per_machine: HashMap<i32, String> = HashMap::new();

    let mut results: Vec<DecodedOperation> = Vec::new();

    let mut op_index = create_vec_with_length_and_value(instance.nr_jobs, 0);

    // A map which stores a sorted set of operations for each machine
    // The set contains tuples (start, end, enzyme)
    let mut machine_tasks: HashMap<i32, BTreeMap<(i32, i32), MachineTask>> = HashMap::new();

    const PLACEHOLDER_ENZYME: &str = "placeholder";
    for m in 0..instance.instance_constants.nr_machines {
        let mut map: BTreeMap<(i32, i32), MachineTask> = BTreeMap::new();
        map.insert(
            (0, 0),
            MachineTask::new(0, 0, PLACEHOLDER_ENZYME.to_owned()),
        );
        machine_tasks.insert(m, map);
    }

    for job in schedule.v2.clone().into_iter() {
        let operation: i32 = instance.operations[&job][op_index[job as usize] as usize];
        let machine = schedule.v1[(instance.initial_operation_index_in_job_vector_per_job
            [job as usize]
            + op_index[job as usize]) as usize];
        op_index[job as usize] += 1;

        let duration = instance.processing_times[&(job, operation, machine)];

        let mut change_over_time = 0;

        let curr_enzyme = instance.orders[&job].product.clone();

        if previous_enzyme_per_machine.contains_key(&machine) {
            let prev_enzyme = previous_enzyme_per_machine[&machine].clone();
            change_over_time = instance.instance_constants.change_overs
                [&(machine, prev_enzyme, curr_enzyme.clone())];
        }

        let mut found = false;
        let mut start = -1;
        let mut end = -1;

        let machine_tasks_sorted = machine_tasks[&machine]
            .values()
            .cloned()
            .filter(|machine_task| machine_task.end > curr_job_times[job as usize])
            .collect::<Vec<MachineTask>>();
        if machine_tasks_sorted.len() >= 2 {
            for index in 1..machine_tasks_sorted.len() {
                let previous_operation: &MachineTask = &machine_tasks_sorted[index - 1];
                let curr_operation: &MachineTask = &machine_tasks_sorted[index];

                let mut first_change_over_time = 0;
                if !previous_operation.enzyme.eq(PLACEHOLDER_ENZYME) {
                    first_change_over_time = instance.instance_constants.change_overs[&(
                        machine,
                        previous_operation.enzyme.clone(),
                        curr_enzyme.clone(),
                    )];
                }

                let second_change_over_time = instance.instance_constants.change_overs
                    [&(machine, curr_enzyme.clone(), curr_operation.enzyme.clone())];

                let interval_start = previous_operation.end;
                let interval_end = curr_operation.start;

                let gap = interval_end
                    - interval_start
                    - first_change_over_time
                    - second_change_over_time;

                if gap < duration {
                    continue;
                }

                let potential_start =
                    max(curr_job_times[job as usize], interval_start) + first_change_over_time;
                let potential_end = potential_start + duration + second_change_over_time;

                if potential_end <= interval_end {
                    found = true;
                    start = potential_start;
                    end = start + duration;
                    continue;
                }
            }
        }

        if !found {
            curr_machine_times[machine as usize] += change_over_time;
            start = max(
                curr_job_times[job as usize],
                curr_machine_times[machine as usize],
            );
            end = start + duration;
            previous_enzyme_per_machine.insert(machine, curr_enzyme.clone());
        }

        curr_job_times[job as usize] = max(curr_job_times[job as usize], end);
        curr_machine_times[machine as usize] = max(curr_machine_times[machine as usize], end);
        machine_tasks.get_mut(&machine).unwrap().insert(
            (start, end),
            MachineTask::new(start, end, curr_enzyme.to_owned()),
        );

        results.push(DecodedOperation::new(
            machine,
            job,
            curr_enzyme.clone(),
            operation,
            start,
            duration,
            end,
        ));
    }

    DecodedSchedule::new(results)
}

pub fn create_vec_with_length_and_value(length: i32, value: i32) -> Vec<i32> {
    let mut zeros = Vec::with_capacity(length as usize);
    zeros.resize(length as usize, value);
    zeros
}
