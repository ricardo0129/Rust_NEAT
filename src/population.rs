use crate::constants::*;
use crate::genome::Genome;
use rand::Rng;
use std::collections::{BTreeMap, BTreeSet};

pub struct Species {
    pub organisms: Vec<i32>,
    pub leader: i32,
}

impl Species {
    pub fn new() -> Self {
        return Self {
            organisms: vec![],
            leader: 0,
        };
    }
}

pub struct Population {
    pub population: Vec<Genome>,
    pub inno_count: i32,
    pub unique_nodes: i32,
    pub inno_split: BTreeMap<(i32, i32), i32>,
    pub inno_edges: BTreeMap<(i32, i32), i32>,
    pub inputs: i32,
    pub outputs: i32,
    pub act: fn(f64) -> f64,
    pub previous_gen: Vec<Species>,
    pub gen: i32,
}

impl Population {
    pub fn new(size: i32, inputs: i32, outputs: i32, act: fn(f64) -> f64) -> Self {
        let mut pop: Vec<Genome> = vec![];
        for _ in 0..size {
            let mut g = Genome::new(inputs, outputs, act);
            g.connect_ends();
            pop.push(g);
        }
        return Self {
            gen: 0,
            previous_gen: vec![],
            population: pop,
            inno_count: inputs * outputs,
            unique_nodes: inputs + outputs,
            inno_split: BTreeMap::new(),
            inno_edges: BTreeMap::new(),
            act: act,
            inputs: inputs,
            outputs: outputs,
        };
    }

    pub fn initialize_inno(&mut self) {
        for i in 0..self.inputs {
            for j in 0..self.outputs {
                self.inno_edges
                    .insert((i, self.inputs + j), i * self.outputs + j);
            }
        }
    }

    pub fn get_inno_split(&mut self, from: i32, to: i32) -> i32 {
        if !self.inno_split.contains_key(&(from, to)) {
            self.inno_split.insert((from, to), self.unique_nodes);
            self.unique_nodes += 1;
        }
        *self.inno_split.get(&(from, to)).unwrap()
    }

    pub fn get_inno_edge(&mut self, from: i32, to: i32) -> i32 {
        if !self.inno_edges.contains_key(&(from, to)) {
            self.inno_edges.insert((from, to), self.inno_count);
            self.inno_count += 1;
        }
        *self.inno_edges.get(&(from, to)).unwrap()
    }

    pub fn mutate(&mut self, id: i32) {
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
                if self.population[id as usize].edge_exist(e.0, e.1) {
                    self.population[id as usize].enable_edge(e.0, e.1);
                } else {
                    self.population[id as usize].add_edge(e.0, e.1, inno, 1.0, true);
                }
            }
        }
    }

    pub fn breed(&self, u: &Genome, v: &Genome) -> Genome {
        //Assume u is the more fit parent
        //For matching genes randomley pick between both parents
        //Otherwise only chose the more fit parents genes
        let genome_u = u.flatten();
        let genome_v = v.flatten();
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut base: Genome = Genome::new(self.inputs, self.outputs, self.act);
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

    pub fn evaluate_all(
        &self,
        inputs: &Vec<f64>,
        metric: fn(&Vec<f64>, &Vec<f64>) -> f64,
    ) -> Vec<f64> {
        //Given an input vector return an array of fitness functions for each individual in the
        //population
        let mut fitness: Vec<f64> = vec![];
        for sp in &self.population {
            let fit = metric(&inputs, &sp.evaluate(inputs));
            fitness.push(fit);
        }
        return fitness;
    }

    pub fn delta(&self, u: &Genome, v: &Genome) -> f64 {
        //calculate combatability between two organisms
        let genome_u = u.flatten();
        let genome_v = v.flatten();
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut disjoint: i32 = 0;
        let mut matching: i32 = 0;
        let mut excess: i32 = 0;
        let mut weights: f64 = 0.0;
        while i < genome_u.len() && j < genome_v.len() {
            if genome_u[i].innovation_number == genome_v[j].innovation_number {
                weights += f64::abs(genome_u[i].weight - genome_v[i].weight);
                matching += 1;
                i += 1;
                j += 1;
            } else if genome_u[i].innovation_number < genome_v[j].innovation_number {
                disjoint += 1;
                i += 1;
            } else {
                j += 1;
            }
        }
        excess += i32::abs((genome_u.len() - i) as i32) + i32::abs((genome_v.len() - j) as i32);
        let mut n: i32 = i32::max(genome_u.len() as i32, genome_v.len() as i32);
        if n < 20 {
            n = 1;
        }
        let n: f64 = n as f64;
        let matching: f64 = matching as f64;
        let delta: f64 =
            (C1 * (excess as f64)) / n + (C2 * (disjoint as f64)) / n + (C3 * weights) / matching;
        return delta;
    }

    pub fn speciate(&self, new_gen: &Vec<Genome>) -> Vec<Species> {
        let mut sp: Vec<Species> = vec![];
        let mut leaders: Vec<&Genome> = vec![];
        for i in 0..self.previous_gen.len() {
            leaders.push(&self.population[self.previous_gen[i].leader as usize]);
            sp.push(Species::new());
        }
        let mut idx: i32 = 0;
        for p in new_gen {
            let mut added: bool = false;
            for i in 0..leaders.len() {
                let d: f64 = self.delta(leaders[i], &p);
                if d < 0.2 {
                    added = true;
                    sp[i].organisms.push(idx);
                    break;
                }
            }
            if !added {
                leaders.push(&new_gen[idx as usize]);
                sp.push(Species::new());
                let size: usize = sp.len();
                sp[size - 1].organisms.push(idx);
            }
            idx += 1;
        }
        let mut new_species: Vec<Species> = vec![];
        for i in 0..sp.len() {
            if sp[i].organisms.len() == 0 {
                continue;
            }
            new_species.push(Species::new());
            for g in &sp[i].organisms {
                let size: usize = new_species.len();
                new_species[size - 1].organisms.push(*g);
            }
        }
        for i in 0..new_species.len() {
            let u = rand::thread_rng().gen_range(1..=new_species[i].organisms.len()) - 1;
            let leader_idx = new_species[i].organisms[u];
            new_species[i].leader = leader_idx;
        }
        return sp;
    }

    pub fn create_species(
        &self,
        curr_gen: &Vec<&Genome>,
        fitness: &Vec<f64>,
        number_offspring: i32,
    ) -> Vec<Genome> {
        assert_eq!(curr_gen.len(), fitness.len());
        //only use the highest performing members of each species
        let mut best_ones: Vec<(f64, i32)> = vec![];
        for i in 0..fitness.len() {
            best_ones.push((fitness[i], i as i32));
        }
        let top_members: i32 = ((curr_gen.len() as f64) * 0.5).round() as i32;
        let mut new_gen: Vec<Genome> = vec![];
        for _ in 0..number_offspring {
            assert!(top_members > 0);
            let u = (rand::thread_rng().gen_range(1..=top_members) - 1) as usize;
            let v = (rand::thread_rng().gen_range(1..=top_members) - 1) as usize;
            new_gen.push(self.breed(&curr_gen[u], &curr_gen[v]));
        }
        new_gen
    }

    pub fn next_generation(&mut self, fitness: &mut Vec<f64>) {
        //population stores the current generation with an input of fitness values
        //create a new gereration after specification
        if self.gen == 0 {
            self.previous_gen = self.speciate(&self.population);
        }
        let mut assigned: Vec<i32> = vec![0; fitness.len()];
        let mut mapping: Vec<i32> = vec![0; self.population.len()];
        let mut idx: i32 = 0;
        for s in &self.previous_gen {
            for a in &s.organisms {
                assigned[*a as usize] = s.organisms.len() as i32;
                mapping[*a as usize] = idx;
            }
            idx += 1;
        }
        for i in 0..fitness.len() {
            fitness[i] = fitness[i] / (assigned[i] as f64);
        }
        let mut sum_fitness: f64 = 0.0;
        let mut species_fitness: Vec<f64> = vec![0.0; self.previous_gen.len()];
        for i in 0..fitness.len() {
            sum_fitness += fitness[i];
            species_fitness[mapping[i] as usize] += fitness[i];
        }
        let mut new_gen: Vec<Genome> = vec![];
        let mut idx: usize = 0;
        for s in &self.previous_gen {
            let number_offspring: i32 = (species_fitness[idx] * (self.population.len() as f64)
                / sum_fitness)
                .round() as i32;
            println!("{} {}", species_fitness[idx], number_offspring);
            if s.organisms.len() == 0 || number_offspring == 0 {
                continue;
            }
            let mut curr: Vec<&Genome> = vec![];
            let mut fit: Vec<f64> = vec![];
            for a in &s.organisms {
                curr.push(&self.population[*a as usize]);
                fit.push(fitness[*a as usize]);
            }
            let adding = self.create_species(&curr, &fit, number_offspring);
            new_gen.extend(adding);
            idx += 1;
        }

        assert_eq!(new_gen.len(), self.population.len());
        self.previous_gen = self.speciate(&new_gen);
        self.population = new_gen;
        self.gen += 1;
    }
}
