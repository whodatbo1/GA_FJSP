use rand::Rng;

pub fn construct_cdf(distribution: &Vec<f64>) -> Vec<f64>{
    let x = distribution.len();
    let mut cdf = Vec::with_capacity(distribution.len());
    cdf.push(distribution[0]);
    for i in 1..distribution.len() {
        cdf.push(cdf[i - 1] + distribution[i]);
    }
    cdf
}

pub fn binary_search_cdf(cdf: &Vec<f64>, value: f64) -> i32 {

    let mut left = 0 as i32;
    let mut right = cdf.len() as i32;
    let mut mid = (left + right) / 2;

    while left < right {
        if value < cdf[mid as usize] as f64 {
            right = mid - 1;
        } else if value > cdf[mid as usize] as f64 {
            left = mid + 1;
        } else {
            return mid;
        }
        mid = (left + right) / 2;
    }
    if value >= cdf[mid as usize] {
        mid + 1
    } else {
        mid
    }
}

pub fn get_rand_element_from_cumulative_probability_distribution(cdf: &Vec<f64>) -> i32 {

    let mut rng = rand::thread_rng();

    let rand_uniform = rng.gen::<f64>();

    binary_search_cdf(&cdf, rand_uniform)
}

