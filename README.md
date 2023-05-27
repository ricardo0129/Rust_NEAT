# NEAT

## A Rust implementation of evolving neural networks through augmenting topologies  

NEAT is a type of genetic algorithm which evolves neural networks by modifying both the structure
and the weight parameters.

#### Progress

- Tracking gene history through innovation numbers
    - Done
- Node activation function
    - Done
- Avoid adding cycles
    - Done
- Random edge & random split
    - Done
- Speciate after each Generation
    - Done
- Species breeding 
    - Done
- Organism mutation
    - Done

```rust
fn main() {
    /*
    Initialize a population of 150 with structure (2 inputs, 1 output, activation function sigmoid)
    true to connect all inputs to outputs in the network for all organisms
    */

    let mut p1: Population = Population::new(150, 2, 1, sigmoid, true);
    
    /*
    evaluate_all takes in a 

    inputs: &Vec<f64> 
    which is the values each organism will use to evaluate 

    metric: fn (Vec<f64>, Vec<f64>) -> f64
    given the input vector & the output vector from a network calculate the fitness

    outs: stores the fitness vector of the entire population in no particular order
    */
    
    let outs: Vec<f64> = p1.evaluate_all(&in1, metric);

    /*    
    outs: &mut Vec<f64> 
    A vector of fitness values for each organism in the population

    Speciates and produces the next generation depending on the fitness values & the previous 
    generation information which is stored inside the Population struct
    */
    p1.next_generation(&mut outs);
}

```

An example is shown in main.rs where a population of 150 networks is trained to learn XOR  
From my testing the population takes ~40 generations to find an optimal solution   
where the optimal solution is taking the sum of the absolute difference of error from expected  
XOR output and network computed output from all 4 XOR cases. The error must be less than 0.0001 to converge.  
One downside is converging networks are quite large compared to the optimal solution of ~5 nodes ~7 edges typically being  
10 nodes 15 active edges. 



## TODO
- Refactor code & do some further optimizations 
