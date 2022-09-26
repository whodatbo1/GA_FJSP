use crate::decode_simple;
use crate::encode_decode::{decode_optimal, encode_schedule};
use crate::instance::Instance;
use rand::seq::SliceRandom;
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Schedule {
    pub instance_num: i32,
    pub v1: Vec<i32>,
    pub v2: Vec<i32>,
    pub objective_values: HashMap<String, i32>,
}

impl Schedule {
    pub fn new(instance_num: i32, v1: Vec<i32>, v2: Vec<i32>) -> Schedule {
        Schedule {
            instance_num,
            v1,
            v2,
            objective_values: HashMap::new(),
        }
    }

    pub fn calculate_makespan_fast(&mut self, instance: &Instance) -> i32 {
        let makespan = decode_optimal(self, instance).calculate_makespan();
        self.objective_values
            .insert(String::from("makespan"), makespan);
        makespan
    }

    pub fn calculate_makespan_optimal(&mut self, instance: &Instance) -> i32 {
        let makespan = decode_optimal(self, instance).calculate_makespan();
        self.objective_values
            .insert(String::from("makespan"), makespan);
        makespan
    }

    pub fn generate_random_schedule(instance: &Instance) -> Schedule {
        let (v1, v2) = generate_random_schedule_encoding(instance);
        let init_schedule = Schedule::new(instance.instance_num, v1, v2);
        let mut decoded_schedule = decode_optimal(&init_schedule, instance);
        encode_schedule(instance, &mut decoded_schedule)
    }

    pub fn order_by_makespan(&self, other: &Schedule) -> Ordering {
        let self_makespan = self
            .objective_values
            .get("makespan")
            .expect("Makespan of Schedule self missing.");
        let other_makespan = other
            .objective_values
            .get("makespan")
            .expect("Makespan of Schedule other missing.");

        if self_makespan < other_makespan {
            return Less;
        } else if self_makespan > other_makespan {
            return Greater;
        }
        Equal
    }

    pub fn is_valid(&self, instance: &Instance) -> bool {
        // instance.instance_constants.
        true
    }
}

pub fn generate_random_schedule_encoding(instance: &Instance) -> (Vec<i32>, Vec<i32>) {
    let mut v1: Vec<i32> = Vec::new();
    let mut rng = rand::thread_rng();

    for job in 0..instance.nr_jobs {
        for i in 0..(instance.operations[&job].len() as i32) {
            let rand_num: i32 = *instance
                .machine_alternatives
                .get(&(job, i))
                .expect("")
                .choose(&mut rng)
                .expect("");
            v1.push(rand_num);
        }
    }
    let mut v2: Vec<i32> = Vec::new();
    for (job, ops) in &instance.operations {
        for _ in ops {
            v2.push(*job);
        }
    }
    v2.shuffle(&mut rng);
    (v1, v2)
}
