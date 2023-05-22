use crate::genome::Genome;
use rand::Rng;
use std::collections::{BTreeMap, BTreeSet};

pub struct Population {
    population: Vec<Genome>,
    inno_count: i32,
    unique_nodes: i32,
    inno_split: BTreeMap<(i32, i32), i32>,
    inno_edges: BTreeMap<(i32, i32), i32>,
    inputs: i32,
    outputs: i32,
}

impl Population {
    fn new(size: i32, inputs: i32, outputs: i32) -> Self {
        let mut pop: Vec<Genome> = vec![];
        for _ in 0..size {
            let mut g = Genome::new(inputs, outputs);
            g.connect_ends();
            pop.push(g);
        }
        return Self {
            population: pop,
            inno_count: inputs * outputs,
            unique_nodes: inputs + outputs,
            inno_split: BTreeMap::new(),
            inno_edges: BTreeMap::new(),
            inputs: inputs,
            outputs: outputs,
        };
    }

    fn initialize_inno(&mut self) {
        for i in 0..self.inputs {
            for j in 0..self.outputs {
                self.inno_edges
                    .insert((i, self.inputs + j), i * self.outputs + j);
            }
        }
    }

    fn get_inno_split(&mut self, from: i32, to: i32) -> i32 {
        if !self.inno_split.contains_key(&(from, to)) {
            self.inno_split.insert((from, to), self.unique_nodes);
            self.unique_nodes += 1;
        }
        *self.inno_split.get(&(from, to)).unwrap()
    }

    fn get_inno_edge(&mut self, from: i32, to: i32) -> i32 {
        if !self.inno_edges.contains_key(&(from, to)) {
            self.inno_edges.insert((from, to), self.inno_count);
            self.inno_count += 1;
        }
        *self.inno_edges.get(&(from, to)).unwrap()
    }

    fn mutate(&mut self, id: i32) {
        //Mutate the nth Genome
        let choice: f64 = rand::thread_rng().gen();
        if choice < 0.3 {
            //Try to split a connection
            let e: (i32, i32) = self.population[id as usize].random_split();
            if e.0 != -1 {
                let u_global = self.population[id as usize].local_to_global(e.0);
                let v_global = self.population[id as usize].local_to_global(e.1);
                let split_node = self.get_inno_split(u_global, v_global);
                let inno = self.get_inno_edge(u_global, split_node);
                self.get_inno_edge(split_node, v_global);
                self.population[id as usize].split_edge(e.0, e.1, inno, split_node);
            }
        } else {
            //Try to add new connection
            let e: (i32, i32) = self.population[id as usize].random_edge();
            if e.0 != -1 {
                let u_global = self.population[id as usize].local_to_global(e.0);
                let v_global = self.population[id as usize].local_to_global(e.1);
                let inno = self.get_inno_edge(u_global, v_global);
                self.population[id as usize].add_edge(e.0, e.1, inno, 1.0, true);
            }
        }
    }

    // from: i32,
    // to: i32,
    // innovation_number: i32,
    // weight: f64,
    // active: bool,
    fn breed(&mut self, u: i32, v: i32) -> Genome {
        //Assume u is the more fit parent
        //For matching genes randomley pick between both parents
        //Otherwise only chose the more fit parents genes
        let genome_u = self.population[u as usize].flatten();
        let genome_v = self.population[v as usize].flatten();
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut base: Genome = Genome::new(self.inputs, self.outputs);
        let mut unique: BTreeSet<i32> = BTreeSet::new();
        let mut mapping: BTreeMap<i32, i32> = BTreeMap::new();
        for g in &genome_u {
            unique.insert(g.from);
            unique.insert(g.to);
        }
        for g in unique {
            mapping.insert(g, mapping.len() as i32);
            if g >= self.inputs + self.outputs {
                base.add_node(g);
            }
        }
        while i < genome_u.len() && j < genome_v.len() {
            let v1: i32 = genome_u[i].innovation_number;
            let v2: i32 = genome_v[j].innovation_number;
            let u: i32 = *mapping.get(&genome_u[i].from).unwrap();
            let v: i32 = *mapping.get(&genome_u[i].to).unwrap();
            if v1 == v2 {
                let choice: f64 = rand::thread_rng().gen();
                assert_eq!(genome_u[i].from, genome_v[i].from);
                assert_eq!(genome_u[i].to, genome_v[i].to);
                if choice < 0.5 {
                    base.add_edge(u, v, v1, genome_u[i].weight, genome_u[i].active);
                } else {
                    base.add_edge(u, v, v1, genome_v[i].weight, genome_v[i].active);
                }
                i += 1;
                j += 1;
            } else if v1 < v2 {
                base.add_edge(u, v, v1, genome_u[i].weight, genome_u[i].active);
                i += 1;
            } else {
                j += 1;
            }
        }
        while i < genome_u.len() {
            let v1: i32 = genome_u[i].innovation_number;
            let u: i32 = *mapping.get(&genome_u[i].from).unwrap();
            let v: i32 = *mapping.get(&genome_u[i].to).unwrap();
            base.add_edge(u, v, v1, genome_u[i].weight, genome_u[i].active);
            i += 1;
        }
        //we dont care about the excess genes from parent v
        return base;
    }
}
