use std::collections::HashMap;
use crate::encode_decode::decode_simple;
use crate::generate_instances::load_change_overs;
use crate::instance::InstanceConstants;
use crate::schedule::Schedule;

pub mod instance;
pub mod utils;
mod generate_instances;
mod encode_decode;
mod schedule;
mod population;

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

    for _ in 0..10 {
        let (v1, v2) = schedule::generate_random_schedule_encoding(&instances.get(&0).expect(""));

        let s: Schedule = Schedule::new(&instances[&0], v1, v2);

        // let decoded_schedule = decode_simple(&s);
        println!("{}", s.calculate_makespan());
    }

    println!("Done");
}
