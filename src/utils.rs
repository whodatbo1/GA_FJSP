use rand::Rng;

pub fn construct_cdf(distribution: Vec<f64>) -> Vec<f64>{
    let x = distribution.len();
    let mut cdf = Vec::with_capacity(distribution.len());
    cdf.push(distribution[0]);
    for i in 1..distribution.len() {
        cdf.push(cdf[i - 1] + distribution[i]);
    }
    cdf
}

pub fn get_rand_element_from_cumulative_probability_distribution(cdf: &Vec<f64>) -> i32 {

    let mut rng = rand::thread_rng();

    let rand_uniform: f64 = rng.gen::<f64>();

    let index = cdf.binary_search_by(|probe| probe.total_cmp(&rand_uniform));

    let final_pos: i32;

    match index {
        Ok(i) => final_pos = i as i32,
        Err(i) => final_pos = i as i32
    }

    final_pos
}

