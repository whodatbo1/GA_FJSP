use std::collections::HashMap;

pub struct InstanceConstants {
    pub nr_machines: i32,
    pub products: Vec<String>,
    pub unit_machines: HashMap<i32, Vec<i32>>,
    pub processing_times: [Vec<i32>; 3],
    pub recipes: [Vec<i32>; 6],
    pub change_overs: HashMap<(i32, String, String), i32>
}

impl InstanceConstants {
    pub fn new(nr_machines: i32,
               products: Vec<String>,
               unit_machines: HashMap<i32, Vec<i32>>,
               processing_times: [Vec<i32>; 3],
               recipes: [Vec<i32>; 6],
               change_overs: HashMap<(i32, String, String), i32>) -> InstanceConstants{
        InstanceConstants {
            nr_machines,
            products,
            unit_machines,
            processing_times,
            recipes,
            change_overs
        }
    }
}

pub struct Order {
    pub product: String,
    pub due: i32
}

pub struct Instance<'a> {
    pub instance_constants: &'a InstanceConstants,
    pub nr_jobs: i32,
    pub orders: HashMap<i32, Order>,
    pub jobs: Vec<i32>,
    pub operations: HashMap<i32, Vec<i32>>,
    pub machine_alternatives: HashMap<(i32, i32), Vec<i32>>,
    pub processing_times: HashMap<(i32, i32, i32), i32>,
    pub job_vector: Vec<i32>,
    pub initial_operation_index_in_job_vector_per_job: Vec<i32>
}
