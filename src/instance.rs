use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct JsonOrder {
    product: String,
    due: String
}

#[derive(Serialize, Deserialize)]
pub struct JsonInstance {
    pub nr_machines: i32,
    pub nr_jobs: i32,
    pub orders: HashMap<String, JsonOrder>,
    pub machines: Vec<i32>,
    pub jobs: Vec<i32>,
    pub operations: HashMap<String, String>,
    pub machineAlternatives: HashMap<String, String>,
    pub processingTimes: HashMap<String, String>,
    pub changeOvers: HashMap<String, String> 
}

pub struct InstanceConstants {
    pub nr_machines: i32,
    pub products: Vec<String>,
    pub unit_machines: HashMap<i32, Vec<i32>>,
    pub processing_times: HashMap<(i32, i32, i32), i32>,
    pub change_overs: HashMap<(i32, String, String), i32>
}

pub struct Order<'a> {
    pub product: &'a String,
    pub due: i32
}

pub struct Instance<'a> {
    // pub instance_constants: &'static InstanceConstants,
    pub nr_jobs: i32,
    pub orders: HashMap<i32, Order<'a>>,
    pub jobs: Vec<i32>,
    pub operations: HashMap<i32, Vec<i32>>,
    pub machine_alternatives: HashMap<(i32, i32), Vec<i32>>,
    pub job_vector: Vec<i32>
}

// impl Instance {
//     pub fn getMachineALternatives(job: i32, operation: i32) {
//
//     }
// }

pub fn createInstanceFromJsonInstance(jsonInstance: JsonInstance) {

    // let mut orders: HashMap<i32, Order> = HashMap::new();
    // for (key, value) in jsonInstance.orders.into_iter() {
    //     let keyInt: i32 = key.parse::<i32>()?;
    //     let due: i32 = value.due.parse::<i32>()?;
    //     orders.insert(keyInt, Order {product: value.product, due});
    // }
    //
    // for (key, value) in jsonInstance.operations.into_iter() {
    //     let keyInt: i32 = key.parse::<i32>().expect("");
        // let ops: Vec<i32> = value.parse()::<Vec<i32>>().expect();
    // }
}