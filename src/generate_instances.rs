use std::collections::HashMap;
use crate::instance::{Instance, InstanceConstants};
use crate::instance::Order;
use rand::Rng;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ChangeOverTime {
    index: i32,
    machine: i32,
    product1: String,
    product2: String,
    change_over: i32
}

pub fn load_change_overs() -> HashMap<(i32, String, String), i32> {
    let mut change_overs = csv::Reader::from_path("D:\\Code\\Rust\\GA\\src\\change_overs.csv")
        .expect("Cannot read csv.");

    let mut change_overs_map: HashMap<(i32, String, String), i32> = HashMap::new();
    for result in change_overs.deserialize() {
        let record: ChangeOverTime = result.expect("Unable to parse change over.");
        change_overs_map.insert((record.machine, record.product1, record.product2), record.change_over);
    }
    change_overs_map
}

pub fn generate_all_instances(instance_constants: &InstanceConstants,
                              nr_instances: i32
) -> HashMap<i32, Instance> {

    let nr_products: i32 = instance_constants.products.len() as i32;

    let mut instances: HashMap<i32, Instance> = HashMap::new();
    for instance_num in 0..nr_instances {
        let mut operations = HashMap::new();
        let mut machine_alternatives: HashMap<(i32, i32), Vec<i32>> = HashMap::new();
        let mut processing_times: HashMap<(i32, i32, i32), i32> = HashMap::new();
        let mut orders = HashMap::new();

        let mut rng = rand::thread_rng();

        let mut job_id: i32 = 0;

        let jobs = (0..(nr_products * (instance_num + 1))).collect::<Vec<i32>>();

        for product in 0..nr_products {
            let job: &i32 = &jobs[job_id as usize];
            for _ in 0..(instance_num + 1) {
                orders.insert(*job, Order {
                    product: instance_constants.products[product as usize].clone(),
                    due: 10 + rng.gen_range(0..(10 * (instance_num) + 1))
                });

                let ops: Vec<i32> = (0..(instance_constants.recipes[product as usize].len() as i32)).collect::<Vec<i32>>();
                operations.insert(*job, ops);
                for j in operations.get(&job).expect("") {
                    let unit: i32 = instance_constants.recipes[product as usize][*j as usize];
                    let alternatives: Vec<i32> = instance_constants.unit_machines.get(&unit).expect("").to_vec();
                    for k in &alternatives {
                        processing_times.insert((*job, *j, *k), instance_constants.processing_times[unit as usize][product as usize]);
                    }
                    machine_alternatives.insert((*job, *j), alternatives);

                }
            }
            job_id += 1;
        }

        let mut job_vector: Vec<i32> = Vec::new();
        let mut initial_operation_index_in_job_vector_per_job = Vec::new();
        let mut index: i32 = 0;
        for job in 0..operations.len() {
            initial_operation_index_in_job_vector_per_job.push(index);
            for _ in &operations[&(job as i32)] {
                job_vector.push(job as i32);
                index += 1;
            }
        }

        let curr_instance = Instance {
            instance_constants,
            nr_jobs: jobs.len() as i32,
            orders,
            jobs,
            operations,
            machine_alternatives,
            processing_times,
            job_vector,
            initial_operation_index_in_job_vector_per_job
        };
        instances.insert(instance_num, curr_instance);
    }

    instances
}