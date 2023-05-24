use rand::Rng;
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

pub fn pertube(x: f64) -> f64 {
    return x;
}
