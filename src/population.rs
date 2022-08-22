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

    pub fn calculate_objective_values_and_sort(&mut self) {
        for schedule in self.members.iter_mut() {
           schedule.calculate_makespan();
        }
        self.members.sort_by(|a, b| a.order_by_makespan(b));
    }

    pub fn get_average_makespan(&mut self) -> f64{
        self.calculate_objective_values_and_sort();
        let mut average: f64 = 0.0;
        for schedule in self.members.iter() {
            average += schedule.objective_values["makespan"] as f64;
        }
        average / self.members.len() as f64
    }

    pub fn generate_probability_distribution(&self) -> Vec<f64>{
        let min_makespan = self.members[0].objective_values["makespan"];
        let max_makespan = self.members[self.members.len() - 1].objective_values["makespan"];

        let sum_min_max = min_makespan + max_makespan;

        let mut probability = Vec::with_capacity(self.members.len());
        let mut probability_sum: f64 = 0.0;

        for schedule in self.members.iter() {
            let val = sum_min_max - schedule.objective_values["makespan"];
            probability.push(val as f64);
            probability_sum += val as f64;
        }
        for i in 0..probability.len() {
            probability[i] = probability[i] / probability_sum;
        }
        probability
    }
}