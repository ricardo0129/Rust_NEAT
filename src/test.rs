use crate::genome::Genome;

#[test]
fn fully_connected() {
    let mut g = Genome::new(4, 4);
    g.connect_ends();
    let input: Vec<f64> = vec![1.0; 4 as usize];
    let output = g.evaluate(input);
    let expected = vec![4.0, 4.0, 4.0, 4.0];
    assert_eq!(output, expected);
}

#[test]
fn split_node_working() {
    let mut g = Genome::new(1, 1);
    g.add_edge(0, 1, 1, 10.0, true);
    g.split_edge(0, 1, 2, 10);
    let input: Vec<f64> = vec![1.0; 1 as usize];
    let output = g.evaluate(input);
    let expected = vec![10.0];
    assert_eq!(output, expected);
}
