pub mod genome;
pub mod node;
pub mod population;
pub mod test;
use crate::genome::Genome;

fn main() {
    let mut g1 = Genome::new(3, 2);
    let vals = vec![1.0; 3];
    g1.connect_ends();
    let outs = g1.evaluate(vals);
    for o in outs {
        println!("{}", o);
    }
}
