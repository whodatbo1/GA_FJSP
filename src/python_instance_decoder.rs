use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use crate::{Instance, InstanceConstants};
use crate::instance::Order;
use regex::Regex;

const INSTANCE_PATH: &str = "D:\\Code\\Rust\\GA\\src\\python_instances\\";

pub fn decode_python_instance(filename: &str, instance_constants: InstanceConstants, instance_num: i32) -> Instance {

    let contents = fs::read_to_string(format!("{}{}", INSTANCE_PATH, filename)).expect("Couldn't find file.");

    let mut nr_machines: i32;
    let mut nr_jobs: i32 = 0;
    let mut orders: HashMap<i32, Order> = HashMap::new();
    let mut machines: Vec<i32> = Vec::new();
    let mut jobs: Vec<i32> = Vec::new();
    let mut operations: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut machine_alternatives: HashMap<(i32, i32), Vec<i32>> = HashMap::new();
    let mut processing_times: HashMap<(i32, i32, i32), i32> = HashMap::new();

    for line in contents.split("\n") {
        let name_value_vec = line.split(" = ").collect::<Vec<&str>>();
        if name_value_vec.len() < 2 {
            break;
        } else {
            let name = name_value_vec[0];
            let value = name_value_vec[1];
            match name {
                "nr_machines" => nr_machines = value.parse::<i32>().unwrap(),
                "nr_jobs" => nr_jobs = value.parse::<i32>().unwrap(),
                "orders" => parse_orders(value, &mut orders),
                "machines" => machines = parse_array_from_str::<i32>(value).expect("Error parsing machines"),
                "jobs" => jobs = parse_array_from_str::<i32>(value).expect("Error parsing jobs"),
                "operations" => parse_operations(value, &mut operations),
                "machineAlternatives" => parse_machine_alternatives(value, &mut machine_alternatives),
                "processingTimes" => parse_processing_times(value, &mut processing_times),
                _ => {}
            }
        }
    }

    Instance::new(
        instance_constants.clone(),
        instance_num,
        nr_jobs,
        orders,
        jobs,
        operations,
        machine_alternatives,
        processing_times
    )
}

fn parse_orders(orders_string: &str, mut orders: &mut HashMap<i32, Order>) {
    let r = Regex::new(r"\d+: \{'product': 'enzyme\d+', 'due': \d+}").expect("");

    for op in r.captures_iter(orders_string) {
        let job = op[0]
            .split(":")
            .next().unwrap()
            .parse::<i32>().unwrap();
        let product = op[0]
            .split("'product': ")
            .collect::<Vec<&str>>()[1]
            .split(",")
            .next().unwrap()
            .split("'")
            .collect::<Vec<&str>>()[1];
        let due = op[0]
            .split("'due': ")
            .collect::<Vec<&str>>()[1]
            .split("}")
            .next().unwrap()
            .parse::<i32>().unwrap();
        orders.insert(job, Order{product: product.to_owned(), due});
    }
}

fn parse_operations(operations_string: &str, operations: &mut HashMap<i32, Vec<i32>>) {
    let r = Regex::new(r"\d+: \[(\d+(, )?)+]").unwrap();

    for operation in r.captures_iter(operations_string) {
        let split_str = operation[0].split(": ").collect::<Vec<&str>>();
        let job = split_str[0].parse::<i32>().unwrap();
        let operations_for_job = parse_array_from_str::<i32>(split_str[1]).unwrap();
        operations.insert(job, operations_for_job);
    }
}

fn parse_machine_alternatives(machine_alternatives_string: &str, machine_alternatives: &mut HashMap<(i32, i32), Vec<i32>>) {
    let r = Regex::new(r"\(\d+, \d+\): \[(\d+(, )?)+]").unwrap();

    for alternative in r.captures_iter(machine_alternatives_string) {
        let split_str = alternative[0].split(": ").collect::<Vec<&str>>();
        let job_op = parse_array_from_str::<i32>(split_str[0]).unwrap();
        let job = job_op[0];
        let operation = job_op[1];
        let machines = parse_array_from_str::<i32>(split_str[1]).unwrap();
        machine_alternatives.insert((job, operation), machines);
    }
}

fn parse_processing_times(processing_times_str: &str, processing_times: &mut HashMap<(i32, i32, i32), i32>) {
    let r = Regex::new(r"\(\d+, \d+, \d+\): \d+").unwrap();

    for p_time in r.captures_iter(processing_times_str) {
        let split_str = p_time[0].split(": ").collect::<Vec<&str>>();
        let key = parse_array_from_str::<i32>(split_str[0]).unwrap();
        let job = key[0];
        let operation = key[1];
        let machine = key[2];
        let time = split_str[1].parse::<i32>().unwrap();
        processing_times.insert((job, operation, machine), time);
    }
}

fn parse_array_from_str<T: FromStr>(value: &str) -> Result<Vec<T>, T::Err> {
    let mut trimmed = value.clone().trim();
    let v = &trimmed[1..trimmed.len() - 1];
    v
        .split(", ")
        .map(|x| x.parse::<T>())
        .collect()
}