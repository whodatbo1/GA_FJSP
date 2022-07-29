use std::collections::HashMap;
use serde_json;
use std::fs::File;
pub mod instance;
pub mod utils;
mod generate_instances;
mod encode_decode;
mod schedule;

fn main() {
    let instance_0 = File::open("./json_instances/instance_0.json").expect("");
    let ins: instance::JsonInstance = serde_json::from_reader(instance_0).expect("");
    let nr_instances = 20;
    let destination = String::from("str_instance_");
    let products: Vec<String> = Vec::from([
        String::from("enzyme0"),
        String::from("enzyme1"),
        String::from("enzyme2"),
        String::from("enzyme3"),
        String::from("enzyme4"),
        String::from("enzyme5")]
    );

    let nr_products: i32 = products.len() as i32;

    let recipes: [Vec<i32>; 6] = [
        Vec::from([0, 1, 2]),
        Vec::from([0, 1]),
        Vec::from([1, 2]),
        Vec::from([0, 1, 2]),
        Vec::from([0, 1, 2]),
        Vec::from([1, 2])
    ];

    const nr_machines: i32 = 9;

    let mut unitMachines: HashMap<i32, Vec<i32>> = HashMap::new();
    unitMachines.insert(0, Vec::from([0, 1, 2]));
    unitMachines.insert(1, Vec::from([3, 4, 5, 6]));
    unitMachines.insert(2, Vec::from([7, 8]));

    let processing_times: [Vec<i32>; 3] = [
        Vec::from([8, 3, 0, 4, 5, 0]),
        Vec::from([4, 2, 3, 6, 4, 8]),
        Vec::from([4, 0, 3, 6, 7, 3])
    ];

    let instances = generate_instances::generateAllInstances(nr_instances, &products, &recipes, nr_machines, &unitMachines, &processing_times);
    schedule::generateRandomScheduleEncoding(&instances.get(&0).expect(""));
}
