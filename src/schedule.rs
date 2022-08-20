use crate::instance::Instance;
use rand::seq::SliceRandom;
use crate::decode_simple;

pub struct Schedule<'a> {
    pub instance: &'a Instance<'a>,
    pub v1: Vec<i32>,
    pub v2: Vec<i32>
}

impl Schedule<'_> {
    pub fn new<'a>(instance: &'a Instance, v1: Vec<i32>, v2: Vec<i32>) -> Schedule<'a> {
        Schedule {
            instance,
            v1,
            v2
        }
    }

    pub fn calculate_makespan(&self) -> i32{
        decode_simple(&self).calculate_makespan()
    }
}

pub fn generate_random_schedule_encoding(instance: &Instance) -> (Vec<i32>, Vec<i32>)  {
    let mut v1: Vec<i32> = Vec::new();
    let mut rng = rand::thread_rng();

    for job in 0..instance.nr_jobs {

        for i in 0..(instance.operations[&job].len() as i32) {
            let rand_num: i32 = *instance.machine_alternatives.get(&(job, i)).expect("").choose(&mut rng).expect("");
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
    return (v1, v2);
}