pub mod activation;
pub mod constants;
pub mod genome;
pub mod helper;
pub mod node;
pub mod population;
use std::time::Instant;
pub mod test;
use crate::activation::sigmoid;
use crate::population::Population;

pub fn mse(inputs: &Vec<f64>, outputs: &Vec<f64>) -> f64 {
    //return MSE of two vectors
    assert_eq!(inputs.len(), outputs.len());
    let mut error: f64 = 0.0;
    for i in 0..inputs.len() {
        //error += (inputs[i] - outputs[i]) * (inputs[i] - outputs[i]);
        error += (inputs[i] - outputs[i]).abs();
    }
    error = error / (inputs.len() as f64);
    //error = 1.0 / (f64::exp(error));
    return error;
}

pub fn metric(inputs: &Vec<f64>, outputs: &Vec<f64>) -> f64 {
    let xor: f64 = ((inputs[0] as i32) ^ (inputs[1] as i32)) as f64;
    let mut out: Vec<f64> = vec![];
    out.push(xor);
    return mse(&out, &outputs);
}

pub fn run_all(pop: &Population, stop: &mut bool) -> Vec<f64> {
    let mut cummulative: Vec<f64> = vec![0.0; pop.population.len()];
    for i in 0..=1 {
        for j in 0..=1 {
            let mut in1: Vec<f64> = vec![];
            in1.push(i as f64);
            in1.push(j as f64);
            let outs = pop.evaluate_all(&in1, metric);
            for k in 0..outs.len() {
                cummulative[k] += outs[k] / 4.0;
            }
        }
    }
    for k in 0..cummulative.len() {
        cummulative[k] = 1.0 - cummulative[k];
        //cummulative[k] = cummulative[k] * cummulative[k];
    }
    let max_idx: usize = cummulative
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(idx, _)| idx)
        .unwrap();
    let mut abs_error: f64 = 0.0;
    for i in 0..=1 {
        for j in 0..=1 {
            let mut in1: Vec<f64> = vec![];
            in1.push(i as f64);
            in1.push(j as f64);
            let outs = pop.population[max_idx].evaluate(&in1);
            let actual = i ^ j;
            abs_error += f64::abs(actual as f64 - outs[0]);
            println!("actual {} expected {}", outs[0], actual)
        }
    }
    println!("best fitness {} error {}", cummulative[max_idx], abs_error);
    if abs_error <= 0.0001 {
        pop.population[max_idx].network_info();
        *stop = true;
    }
    cummulative
}

fn main() {
    let mut p1: Population = Population::new(1000, 2, 1, sigmoid, true);
    for i in 0..3000 {
        let start = Instant::now(); // Record the starting time
        let mut stop: bool = false;
        let mut outs = run_all(&p1, &mut stop);
        println!("iteration {}", i);
        if stop {
            println!("Found Optimal Solution After {} generations", i);
            break;
        }
        p1.next_generation(&mut outs);
        println!("Elapsed time: {} milliseconds", start.elapsed().as_millis());
    }
}
