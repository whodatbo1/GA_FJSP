use crate::encode_decode::{decode_optimal, decode_simple, encode_schedule};
use crate::generate_instances::load_change_overs;
use crate::genetic_operations::cross_over_schedules;
use crate::instance::{Instance, InstanceConstants};
use crate::population::Population;
use crate::python_instance_decoder::decode_python_instance;
use crate::schedule::Schedule;
use rand::Rng;
use std::collections::HashMap;
use std::time::{Duration, Instant};

mod encode_decode;
mod generate_instances;
mod genetic_operations;
pub mod instance;
mod population;
mod python_instance_decoder;
mod schedule;
pub mod utils;

fn main() {
    let nr_instances = 20;

    let products: Vec<String> = Vec::from([
        String::from("enzyme0"),
        String::from("enzyme1"),
        String::from("enzyme2"),
        String::from("enzyme3"),
        String::from("enzyme4"),
        String::from("enzyme5"),
    ]);

    let recipes: [Vec<i32>; 6] = [
        Vec::from([0, 1, 2]),
        Vec::from([0, 1]),
        Vec::from([1, 2]),
        Vec::from([0, 1, 2]),
        Vec::from([0, 1, 2]),
        Vec::from([1, 2]),
    ];

    const NR_MACHINES: i32 = 9;

    let mut unit_machines: HashMap<i32, Vec<i32>> = HashMap::new();
    unit_machines.insert(0, Vec::from([0, 1, 2]));
    unit_machines.insert(1, Vec::from([3, 4, 5, 6]));
    unit_machines.insert(2, Vec::from([7, 8]));

    let processing_times: [Vec<i32>; 3] = [
        Vec::from([8, 3, 0, 4, 5, 0]),
        Vec::from([4, 2, 3, 6, 4, 8]),
        Vec::from([4, 0, 3, 6, 7, 3]),
    ];

    let change_overs = load_change_overs();

    let instance_constants: InstanceConstants = InstanceConstants::new(
        NR_MACHINES,
        products,
        unit_machines,
        processing_times,
        recipes,
        change_overs,
    );

    let python_instance_num = 50;
    let python_instance =
        decode_python_instance("FJSP_0.py", instance_constants.clone(), python_instance_num);

    let mut instances = HashMap::new();

    for i in 0..13 {
        let instance = decode_python_instance(
            &("FJSP_".to_owned() + &i.to_string() + ".py"),
            instance_constants.clone(),
            i,
        );
        instances.insert(i, instance);
    }

    run_ga(12, &instances, 100, 10000, 0.5);
    println!("Done");
}

fn run_ga(
    instance_num: i32,
    instances: &HashMap<i32, Instance>,
    population_size: i32,
    generation_count: i32,
    mutation_coeffictient: f64,
) {
    let instance = instances
        .get(&instance_num)
        .expect("Instance num not found");

    println!("Beginning pipeline...");

    println!("Generating starting population...");
    let mut population = Population::generate_starting_population(instance, population_size);
    population.calculate_objective_values_and_sort_optimal(instance);

    let mut min_makespan = 2000;
    let mut min_makespan_schedule: Schedule = population.members[0].clone();

    for generation in 0..generation_count {
        println!("Generation {}...", generation);
        let generation_start = Instant::now();

        let distribution = population.generate_probability_distribution();
        let cumulative_distribution = utils::construct_cdf(distribution);

        for _ in 0..population_size {
            let parent_male_index =
                utils::get_rand_element_from_cumulative_probability_distribution(
                    &cumulative_distribution,
                );
            let parent_female_index =
                utils::get_rand_element_from_cumulative_probability_distribution(
                    &cumulative_distribution,
                );

            let parent_male = &population.members[parent_male_index as usize];
            let parent_female = &population.members[parent_female_index as usize];

            let child = cross_over_schedules(instance, parent_male, parent_female);

            population.members.push(child);
        }
        population.mutate_schedules(instance, mutation_coeffictient);
        if generation % 10 != 0 {
            population.calculate_objective_values_and_sort_fast(instance);
        } else {
            population.calculate_objective_values_and_sort_fast(instance);
        }
        population.members.truncate(population_size as usize);

        if population.members[0].objective_values["makespan"] < min_makespan {
            min_makespan_schedule = population.members[0].clone();
            min_makespan = min_makespan_schedule.objective_values["makespan"];
        }

        println!(
            "Min makespan: {}\nAverage makespan: {}",
            population.get_members()[0]
                .objective_values
                .get("makespan")
                .expect(""),
            population.get_average_makespan()
        );
        let elapsed = generation_start.elapsed();
        min_makespan_schedule.calculate_makespan_optimal(instance);
        let final_min_makespan = min_makespan_schedule.objective_values["makespan"];
        println!("Final min makespan: {}", final_min_makespan);
        println!("Time elapsed in generation: {:?}", elapsed);
    }
}
