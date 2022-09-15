use std::collections::HashMap;
use rand::Rng;
use crate::encode_decode::decode_simple;
use crate::generate_instances::load_change_overs;
use crate::genetic_operations::cross_over_schedules;
use crate::instance::{Instance, InstanceConstants};
use crate::population::Population;
use crate::schedule::Schedule;

pub mod instance;
pub mod utils;
mod generate_instances;
mod encode_decode;
mod schedule;
mod population;
mod genetic_operations;

fn main() {
    let nr_instances = 20;
    // let destination = String::from("str_instance_");
    let products: Vec<String> = Vec::from([
        String::from("enzyme0"),
        String::from("enzyme1"),
        String::from("enzyme2"),
        String::from("enzyme3"),
        String::from("enzyme4"),
        String::from("enzyme5")]
    );

    // let nr_products: i32 = products.len() as i32;

    let recipes: [Vec<i32>; 6] = [
        Vec::from([0, 1, 2]),
        Vec::from([0, 1]),
        Vec::from([1, 2]),
        Vec::from([0, 1, 2]),
        Vec::from([0, 1, 2]),
        Vec::from([1, 2])
    ];

    const NR_MACHINES: i32 = 9;

    let mut unit_machines: HashMap<i32, Vec<i32>> = HashMap::new();
    unit_machines.insert(0, Vec::from([0, 1, 2]));
    unit_machines.insert(1, Vec::from([3, 4, 5, 6]));
    unit_machines.insert(2, Vec::from([7, 8]));

    let processing_times: [Vec<i32>; 3] = [
        Vec::from([8, 3, 0, 4, 5, 0]),
        Vec::from([4, 2, 3, 6, 4, 8]),
        Vec::from([4, 0, 3, 6, 7, 3])
    ];

    let change_overs = load_change_overs();

    let instance_constants: InstanceConstants = InstanceConstants::new(NR_MACHINES, products, unit_machines, processing_times, recipes, change_overs);

    let instances = generate_instances::generate_all_instances(&instance_constants, nr_instances);

    // for _ in 0..100000 {
    //     let (v10, v20) = schedule::generate_random_schedule_encoding(&instances.get(&0).expect(""));
    //     let (v11, v21) = schedule::generate_random_schedule_encoding(&instances.get(&0).expect(""));
    //
    //     let s: Schedule = Schedule::new(0, v10, v20);
    //     let s1: Schedule = Schedule::new(0, v11, v21);
    //     // let decoded_schedule = decode_simple(&s);
    //     let x = cross_over_schedules(&s, &s1);
    //     // println!("{}", s.calculate_makespan());
    // }
    run_ga(0, &instances, 100, 100, 0.1);
    println!("Done");
}

fn run_ga(instance_num: i32, instances: &HashMap<i32, Instance>, population_size: i32, generation_count: i32, mutation_coeffictient: f64) {
    let instance = instances.get(&instance_num).expect("Instance num not found");

    println!("Beginning pipeline...");

    let mut rng = rand::thread_rng();

    println!("Generating starting population...");
    let mut population = Population::generate_starting_population(instance, population_size);

    for generation in 0..generation_count {
        println!("Generation {}...", generation);

        let a = population.calculate_objective_values_and_sort(&instance);

        let distribution = population.generate_probability_distribution();
        let cumulative_distribution = utils::construct_cdf(distribution);

        for child in 0..population_size {
            let parent_male_index = utils::get_rand_element_from_cumulative_probability_distribution(&cumulative_distribution);
            let parent_female_index = utils::get_rand_element_from_cumulative_probability_distribution(&cumulative_distribution);

            let parent_male = &population.members[parent_male_index as usize];
            let parent_female = &population.members[parent_female_index as usize];

            let child = genetic_operations::cross_over_schedules(&instance, parent_male, parent_female);
            population.members.push(child);
        }

        population.calculate_objective_values_and_sort(&instance);
        population.members.truncate(population_size as usize);

        println!("Min makespan: {}", population.get_members()[0].objective_values.get("makespan").expect(""))
    }
}
