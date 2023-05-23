pub mod activation;
pub mod constants;
pub mod genome;
pub mod node;
pub mod population;
pub mod test;
use crate::activation::sigmoid;
use crate::population::Population;

pub fn mse(inputs: &Vec<f64>, outputs: &Vec<f64>) -> f64 {
    //return MSE of two vectors
    assert_eq!(inputs.len(), outputs.len());
    let mut error: f64 = 0.0;
    for i in 0..inputs.len() {
        error += (inputs[i] - outputs[i]) * (inputs[i] - outputs[i]);
    }
    error = error / (inputs.len() as f64);
    return error;
}

pub fn metric(inputs: &Vec<f64>, outputs: &Vec<f64>) -> f64 {
    let xor: f64 = ((inputs[0] as i32) ^ (inputs[1] as i32)) as f64;
    let mut out: Vec<f64> = vec![];
    out.push(xor);
    return mse(&out, &outputs);
}

fn main() {
    let p1: Population = Population::new(10, 2, 1, sigmoid);
    let mut in1: Vec<f64> = vec![];
    in1.push(0.0);
    in1.push(1.0);
    let outs = p1.evaluate_all(&in1, metric);
    for o in outs {
        println!("{}", o);
    }
}
