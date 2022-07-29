use crate::instance::Instance;
use rand::Rng;
use rand::seq::SliceRandom;

struct Schedule<'a> {
    instance: &'a Instance<'a>
}

pub fn generateRandomScheduleEncoding(instance: &Instance) -> (Vec<i32>, Vec<i32>)  {
    let mut v1: Vec<i32> = Vec::new();
    let mut rng = rand::thread_rng();

    for (job, ops) in &instance.operations {
        for i in 0..(ops.len() as i32) {
            let rand_num: i32 = *instance.machine_alternatives.get(&(*job, i)).expect("").choose(&mut rng).expect("");
            v1.push(rand_num);
        }
    }
    let mut v2: Vec<i32> = Vec::new();
    for (job, ops) in &instance.operations {
        for _ in ops {
            v2.push(*job);
        }
    }
    v2.shuffle(&mut rng);
    return (v1, v2);
}