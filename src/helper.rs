use rand::Rng;

use crate::constants::DELTA_PERMUTE;
pub fn chance(p: f64) -> bool {
    //given a probability returns 1 with probability p or 0 with probability 1-p
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen();
    if x <= p {
        return true;
    }
    return false;
}

pub fn rand_i32(a: i32, b: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let x: i32 = rng.gen_range(a..=b);
    return x;
}

pub fn rand_f64(a: f64, b: f64) -> f64 {
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen_range(a..=b);
    return x;
}

pub fn pertube(mut x: f64) -> f64 {
    x += rand_f64(-DELTA_PERMUTE, DELTA_PERMUTE);
    x = f64::min(x, 1.0);
    x = f64::max(x, -1.0);
    return x;
}
