use std::cmp::{max, Ordering};
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::HashMap;
use crate::instance::Instance;
use crate::schedule::Schedule;

#[derive(Clone)]
pub struct DecodedOperation {
    machine: i32,
    job: i32,
    product: String,
    operation: i32,
    start: i32,
    duration: i32,
    completion: i32
}

impl DecodedOperation {
    pub fn new(machine: i32,
               job: i32,
               product: String,
               operation: i32,
               start: i32,
               duration: i32,
               completion: i32) -> DecodedOperation {
        DecodedOperation {
            machine,
            job,
            product,
            operation,
            start,
            duration,
            completion
        }
    }

    pub fn order_by_job_operation(&self, other: &DecodedOperation) -> Ordering{
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

    pub fn order_by_start(&self, other: &DecodedOperation) -> Ordering{
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
    operations: Vec<DecodedOperation>
}

impl DecodedSchedule {
    pub fn new(decoded_operations: Vec<DecodedOperation>) -> DecodedSchedule{
        DecodedSchedule {
            operations: decoded_operations
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

pub fn encode_schedule<'a>(instance: &'static Instance, schedule: &mut DecodedSchedule) -> Schedule<'a> {

    schedule.operations.sort_by(|a, b| a.order_by_start(b));

    let v2 = schedule.operations.clone().into_iter().map(|dec_op| dec_op.job).collect();

    schedule.operations.sort_by(|a, b| a.order_by_job_operation(b));

    let v1 = schedule.operations.clone().into_iter().map(|dec_op| dec_op.machine).collect();

    return Schedule::new(instance, v1, v2);
}

pub fn decode_simple(schedule: &Schedule) -> DecodedSchedule {
    let mut curr_machine_times = create_zeros_vec(schedule.instance.instance_constants.nr_machines);
    let mut curr_job_times = create_zeros_vec(schedule.instance.nr_jobs);

    let mut previous_enzyme_per_machine: HashMap<i32, String> = HashMap::new();

    let mut results: Vec<DecodedOperation> = Vec::new();

    let mut op_index = create_zeros_vec(schedule.instance.nr_jobs);

    for job in schedule.v2.clone().into_iter() {
        let operation: i32 = schedule.instance.operations[&job][op_index[job as usize] as usize];
        let machine = schedule.v1[(&schedule.instance.initial_operation_index_in_job_vector_per_job[job as usize] + &op_index[job as usize]) as usize];
        op_index[job as usize] += 1;

        let duration = schedule.instance.processing_times[&(job, operation, machine)].clone();

        let mut change_over_time = 0;

        let curr_enzyme = schedule.instance.orders[&job].product.clone();

        if previous_enzyme_per_machine.contains_key(&machine) {
            change_over_time = schedule.instance.instance_constants.change_overs[&(machine, previous_enzyme_per_machine[&machine].clone(), curr_enzyme.clone())];
        }

        curr_machine_times[machine as usize] += change_over_time;

        let start = max(curr_job_times[job as usize], curr_machine_times[machine as usize]);

        let end = start + duration;

        previous_enzyme_per_machine.insert(machine, curr_enzyme.clone());

        curr_job_times[job as usize] = max(curr_job_times[job as usize], end);

        curr_machine_times[machine as usize] = max(curr_machine_times[job as usize], end);

        results.push(DecodedOperation::new(
            machine,
            job,
            curr_enzyme.clone(),
            operation,
            start,
            duration,
            end));
    }

    return DecodedSchedule::new(results);
}

fn create_zeros_vec(length: i32) -> Vec<i32> {
    let mut zeros = Vec::with_capacity(length as usize);
    zeros.resize(length as usize, 0);
    zeros
}