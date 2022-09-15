use std::collections::HashMap;

#[derive(Clone)]
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

pub struct Instance {
    pub instance_constants: InstanceConstants,
    pub instance_num: i32,
    pub nr_jobs: i32,
    pub orders: HashMap<i32, Order>,
    pub jobs: Vec<i32>,
    pub operations: HashMap<i32, Vec<i32>>,
    pub machine_alternatives: HashMap<(i32, i32), Vec<i32>>,
    pub processing_times: HashMap<(i32, i32, i32), i32>,
    pub job_vector: Vec<i32>,
    pub initial_operation_index_in_job_vector_per_job: Vec<i32>
}

impl Instance {

    pub fn new(instance_constants: InstanceConstants,
               instance_num: i32,
               nr_jobs: i32,
               orders: HashMap<i32, Order>,
               jobs: Vec<i32>,
               operations: HashMap<i32, Vec<i32>>,
               machine_alternatives: HashMap<(i32, i32), Vec<i32>>,
               processing_times: HashMap<(i32, i32, i32), i32>) -> Instance {
        let job_vector = Instance::generate_job_vector(&operations);
        let initial_operation_index_in_job_vector_per_job = Instance::generate_initial_operation_index_in_job_vector_per_job(&operations);
        Instance {
            instance_constants,
            instance_num,
            nr_jobs,
            orders,
            jobs,
            operations,
            machine_alternatives,
            processing_times,
            job_vector,
            initial_operation_index_in_job_vector_per_job
        }
    }

    pub fn generate_job_vector(operations: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
        let mut job_vector: Vec<i32> = Vec::new();
        let mut index: i32 = 0;
        for job in 0..operations.len() {
            for _ in &operations[&(job as i32)] {
                job_vector.push(job as i32);
                index += 1;
            }
        }
        job_vector
    }

    pub fn generate_initial_operation_index_in_job_vector_per_job(operations: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
        let mut initial_operation_index_in_job_vector_per_job = Vec::new();
        let mut index: i32 = 0;
        for job in 0..operations.len() {
            initial_operation_index_in_job_vector_per_job.push(index);
            for _ in &operations[&(job as i32)] {
                index += 1;
            }
        }
        initial_operation_index_in_job_vector_per_job
    }
}