use std::assert_eq;

use crate::genome::Genome;

#[test]
fn fully_connected() {
    let mut g = Genome::new(4, 4);
    g.connect_ends();
    let input: Vec<f64> = vec![1.0; 4 as usize];
    let output = g.evaluate(&input);
    let expected = vec![4.0, 4.0, 4.0, 4.0];
    assert_eq!(output, expected);
}

#[test]
fn split_node_working() {
    let mut g = Genome::new(1, 1);
    g.add_edge(0, 1, 1, 10.0, true);
    g.split_edge(0, 1, 2, 10);
    let input: Vec<f64> = vec![1.0; 1 as usize];
    let output = g.evaluate(&input);
    let expected = vec![10.0];
    assert_eq!(output, expected);
}

#[test]
fn add_edge() {
    let mut g = Genome::new(1, 1);
    g.add_edge(0, 1, 1, 15.0, true);
    let input: Vec<f64> = vec![1.0; 1 as usize];
    let output = g.evaluate(&input);
    let expected = vec![15.0];
    assert_eq!(output, expected);
}

#[test]
fn unflatte_size() {
    let mut g = Genome::new(10, 10);
    g.connect_ends();
    let temp = g.flatten();
    assert_eq!(temp.len(), 10 * 10);
}

#[test]
fn flatten_unflatten() {
    let mut g = Genome::new(10, 10);
    g.connect_ends();
    let input: Vec<f64> = vec![1.0; 10 as usize];
    let o1 = g.evaluate(&input);
    let temp = g.flatten();
    let g2 = Genome::un_flatten(&temp, 10, 10);
    let o2 = g2.evaluate(&input);
    assert_eq!(o1, o2);
}

#[test]
fn random_flatten() {
    let mut g = Genome::new(10, 10);
    for _ in 0..5 {
        g.random_split();
    }
    for _ in 0..5 {
        g.random_edge();
    }
    let input: Vec<f64> = vec![1.0; 10 as usize];
    let o1 = g.evaluate(&input);
    let temp = g.flatten();
    let g2 = Genome::un_flatten(&temp, 10, 10);
    let o2 = g2.evaluate(&input);
    assert_eq!(o1, o2);
}
