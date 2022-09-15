use rand::Rng;
use rand::seq::SliceRandom;
use crate::encode_decode::create_vec_with_length_and_value;
use crate::{Instance, Schedule};

pub fn mutate_schedule(instance: &Instance, schedule: &mut Schedule, mutation_coefficient: f64) {
    let mut rng = rand::thread_rng();

    let mut op_index = create_vec_with_length_and_value(instance.nr_jobs, 0);

    for i in 0..instance.job_vector.len() {

        let job = instance.job_vector[i];
        let operation = instance.operations[&job][op_index[job as usize] as usize];
        op_index[job as usize] += 1;

        if rng.gen_bool(mutation_coefficient) {
            let new_machine = instance.machine_alternatives[&(job, operation)].choose(&mut rng).expect("Couldn't generate random.");
            schedule.v1[i] =  *new_machine;
        }

        if rng.gen_bool(mutation_coefficient) {
            //TODO
            //Second mutation type
        }
    }
}

pub fn cross_over_schedules(instance: &Instance, male_schedule: &Schedule, female_schedule: &Schedule) -> Schedule {

    let binary_job_vector_for_male = generate_bool_vector(instance.nr_jobs);
    let binary_machine_vector_for_male = generate_bool_vector(male_schedule.v1.len() as i32);

    let mut child_v1 = create_vec_with_length_and_value(male_schedule.v1.len() as i32, -1);
    let mut child_v2 = create_vec_with_length_and_value(male_schedule.v1.len() as i32, -1);

    let mut jobs_to_add_from_female: Vec<i32> = Vec::new();

    for i in 0..child_v1.len() {
        if binary_machine_vector_for_male[i] {
            child_v1[i] = male_schedule.v1[i].clone();
        } else {
            child_v1[i] = female_schedule.v1[i].clone();
        }

        if binary_job_vector_for_male[male_schedule.v2[i] as usize] {
            child_v2[i] = male_schedule.v2[i].clone();
        } else {
            jobs_to_add_from_female.push(female_schedule.v2[i].clone());
        }
    }
    let mut index: i32 = 0;
    for i in 0..child_v2.len() {
        if child_v2[i] == -1 {
            child_v2[i] = jobs_to_add_from_female[index as usize];
            index += 1;
        }
    }

    Schedule::new(male_schedule.instance_num, child_v1, child_v2)

}

fn generate_bool_vector(length: i32) -> Vec<bool>{
    let mut rng = rand::thread_rng();
    let mut return_vec = Vec::with_capacity(length as usize);

    for _ in 0..length {
        return_vec.push(rng.gen_bool(0.5));
    }

    return_vec
}