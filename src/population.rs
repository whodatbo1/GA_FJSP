use crate::instance::Instance;
use crate::schedule::{generate_random_schedule_encoding, Schedule};

pub struct Population<'a> {
    pub members: Vec<Schedule<'a>>
}

impl Population<'_> {
    pub fn generate_starting_population<'a>(instance: &'a Instance, size: i32) -> Population<'a>{
        let mut schedules: Vec<Schedule<'a>> = Vec::with_capacity(size as usize);
        for _ in 0..size {
            schedules.push(Schedule::generate_random_schedule(instance));
        }
        Population::new(schedules)
    }

    pub fn new(schedules: Vec<Schedule>) -> Population {
        Population {
            members: schedules
        }
    }
}