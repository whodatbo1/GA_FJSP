use std::collections::HashMap;
use crate::instance::Instance;
use crate::instance::Order;
use rand::Rng;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ChangeOverTime {
    index: i32,
    machine: i32,
    product1: String,
    product2: String,
    changeOver: i32
}

pub fn loadChangeOvers() -> HashMap<(i32, String, String), i32> {
    let mut changeOvers = csv::Reader::from_path("change_overs.csv")
        .expect("Cannot read csv.");

    let mut changeOversHM: HashMap<(i32, String, String), i32> = HashMap::new();
    for result in changeOvers.deserialize() {
        let record: ChangeOverTime = result.expect("");
        changeOversHM.insert((record.machine, record.product1, record.product2), record.changeOver);
    }
    changeOversHM
}

pub fn generateAllInstances<'a>(nr_instances: i32,
                                products: &'a Vec<String>,
                                recipes: &[Vec<i32>; 6],
                                nr_machines: i32,
                                unit_machines: &HashMap<i32, Vec<i32>>,
                                processing_times: &[Vec<i32>; 3]
) -> HashMap<i32, Instance<'a>> {
    let nr_products: i32 = products.len() as i32;

    // let change_overs: HashMap<(i32, String, String), i32> = loadChangeOvers();

    let mut instances: HashMap<i32, Instance> = HashMap::new();
    for instanceNum in 0..nr_instances {
        let mut operations = HashMap::new();
        let mut machine_alternatives: HashMap<(i32, i32), Vec<i32>> = HashMap::new();
        let mut processingTimes: HashMap<(i32, i32, i32), i32> = HashMap::new();
        let mut orders = HashMap::new();

        let mut rng = rand::thread_rng();

        let mut job_id: i32 = 0;

        let jobs = (0..(nr_products * (instanceNum + 1))).collect::<Vec<i32>>();

        for product in 0..nr_products {
            let job: &i32 = &jobs[job_id as usize];
            for _ in 0..(instanceNum + 1) {
                orders.insert(*job, Order {
                    product: &products[product as usize],
                    due: 10 + rng.gen_range(0..(10 * (instanceNum) + 1))
                });

                let ops: Vec<i32> = (0..(recipes[product as usize].len() as i32)).collect::<Vec<i32>>();
                operations.insert(*job, ops);
                for j in operations.get(&job).expect("") {
                    let unit: i32 = recipes[product as usize][*j as usize];
                    let alternatives: Vec<i32> = unit_machines.get(&unit).expect("").to_vec();
                    for k in &alternatives {
                        processingTimes.insert((*job, *j, *k), *processing_times[unit as usize].get(product as usize).expect(""));
                    }
                    machine_alternatives.insert((*job, *j), alternatives);

                }
            }
            job_id += 1;
        }

        let mut job_vector: Vec<i32> = Vec::new();
        for (job, ops) in &operations {
            for _ in ops {
                job_vector.push(*job);
            }
        }

        let curr_instance = Instance {
            nr_jobs: jobs.len() as i32,
            orders,
            jobs,
            operations,
            machine_alternatives,
            job_vector
        };
        instances.insert(instanceNum, curr_instance);
    }

    instances
}