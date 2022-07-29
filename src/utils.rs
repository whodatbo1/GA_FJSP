use serde_json;
use std::fs::File;
use crate::instance::JsonInstance;

fn load_json_instance(filename: String) -> serde_json::Value {
    let instance_0 = File::open("./json_instances/instance_0.json").expect("Oh oh");
    let json: serde_json::Value = serde_json::from_reader(instance_0).expect("uuuuh");
    return json;
}

fn create_instance_from_json(json: serde_json::Value) {
    let nr_machines = json.get("nr_machines").expect("nr_machines not found in json.").as_i64();
    let nr_jobs = json.get("nr_jobs").expect("nr_jobs not found in json.").as_i64();
    let orders_json = json.get("nr_machines")
        .expect("orders not found in json.")
        .as_object()
        .expect("Not an object...");
    let nr_machines = json.get("nr_machines").expect("nr_machines not found in json.");
    let nr_machines = json.get("nr_machines").expect("nr_machines not found in json.");
    let nr_machines = json.get("nr_machines").expect("nr_machines not found in json.");
    let nr_machines = json.get("nr_machines").expect("nr_machines not found in json.");

    // let res = instance {
    //     nr_machines: nr_machines,

    // }
}