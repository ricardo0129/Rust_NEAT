pub fn sigmoid(x: f64) -> f64 {
    return 1.0 / (1.0 + f64::exp(-4.9 * x));
}

pub fn ignore(x: f64) -> f64 {
    return x;
}
