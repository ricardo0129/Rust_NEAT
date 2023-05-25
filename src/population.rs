use crate::genome::Genome;
use crate::helper::rand_f64;
use crate::{constants::*, helper::chance, helper::rand_i32};
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
    pub fn new(
        size: i32,
        inputs: i32,
        outputs: i32,
        act: fn(f64) -> f64,
        connect_ends: bool,
    ) -> Self {
        let mut pop: Vec<Genome> = vec![];
        for _ in 0..size {
            let mut g = Genome::new(inputs, outputs, act);
            if connect_ends {
                g.connect_ends();
            }
            pop.push(g);
        }
        let mut obj = Self {
            gen: 0,
            previous_gen: vec![],
            population: pop,
            inno_count: (inputs + 1) * outputs,
            unique_nodes: inputs + outputs + 1,
            inno_split: BTreeMap::new(),
            inno_edges: BTreeMap::new(),
            act: act,
            inputs: inputs,
            outputs: outputs,
        };
        obj.initialize_inno();
        obj
    }

    pub fn initialize_inno(&mut self) {
        for i in 0..(self.inputs + 1) {
            for j in 0..self.outputs {
                self.inno_edges
                    .insert((i, self.inputs + 1 + j), i * self.outputs + j);
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

    pub fn random_split(&mut self, genome: &mut Genome) {
        let e: (i32, i32) = genome.random_split();
        if e.0 != -1 {
            let u_global = genome.local_to_global(e.0);
            let v_global = genome.local_to_global(e.1);
            let split_node = self.get_inno_split(u_global, v_global);
            let inno = self.get_inno_edge(u_global, split_node);
            self.get_inno_edge(split_node, v_global);
            genome.split_edge(e.0, e.1, inno, split_node);
        }
    }

    pub fn random_edge(&mut self, genome: &mut Genome) {
        let e: (i32, i32) = genome.random_edge();
        if e.0 != -1 {
            let u_global = genome.local_to_global(e.0);
            let v_global = genome.local_to_global(e.1);
            let inno = self.get_inno_edge(u_global, v_global);
            if genome.edge_exist(e.0, e.1) {
                genome.enable_edge(e.0, e.1);
            } else {
                assert!(e.0 != e.1);
                genome.add_edge(e.0, e.1, inno, rand_f64(-1.0, 1.0), true);
            }
        }
    }

    pub fn mutate(&mut self, genome: &mut Genome) {
        //Mutate the nth Genome
        if chance(0.20) {
            //Try to split a connection
            self.random_split(genome);
        } else {
            //Try to add new connection
            self.random_edge(genome);
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
            if g >= self.inputs + 1 + self.outputs {
                base.add_node(g);
            }
        }
        while i < genome_u.len() && j < genome_v.len() {
            let v1: i32 = genome_u[i].innovation_number;
            let v2: i32 = genome_v[j].innovation_number;
            let u: i32 = *mapping.get(&genome_u[i].from).unwrap();
            let v: i32 = *mapping.get(&genome_u[i].to).unwrap();
            if v1 == v2 {
                assert_eq!(genome_u[i].from, genome_v[j].from);
                assert_eq!(genome_u[i].to, genome_v[j].to);
                let one_disabled: bool = !(genome_u[i].active && genome_v[j].active);
                let mut active: bool;
                let weight: f64;
                if chance(0.5) {
                    active = genome_u[i].active;
                    weight = genome_u[i].weight;
                } else {
                    active = genome_v[j].active;
                    weight = genome_v[j].weight;
                }
                if one_disabled && chance(0.75) {
                    active = false;
                }
                assert!(u != v);
                base.add_edge(u, v, v1, weight, active);
                i += 1;
                j += 1;
            } else if v1 < v2 {
                assert!(u != v);
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
            assert!(u != v);
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
        let excess: i32;
        let mut weights: f64 = 0.0;
        while i < genome_u.len() && j < genome_v.len() {
            if genome_u[i].innovation_number == genome_v[j].innovation_number {
                weights += f64::abs(genome_u[i].weight - genome_v[j].weight);
                matching += 1;
                i += 1;
                j += 1;
            } else if genome_u[i].innovation_number < genome_v[j].innovation_number {
                disjoint += 1;
                i += 1;
            } else {
                disjoint += 1;
                j += 1;
            }
        }
        excess = i32::abs((genome_u.len() - i) as i32) + i32::abs((genome_v.len() - j) as i32);
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
            if self.previous_gen[i].organisms.len() == 0 {
                continue;
            }
            leaders.push(&self.population[self.previous_gen[i].leader as usize]);
            sp.push(Species::new());
        }
        let mut idx: i32 = 0;
        for p in new_gen {
            let mut added: bool = false;
            for i in 0..leaders.len() {
                let d: f64 = self.delta(leaders[i], &p);
                if d < DT {
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
            //let u = rand::thread_rng().gen_range(1..=new_species[i].organisms.len()) - 1;
            let u = rand_i32(1, new_species[i].organisms.len() as i32) - 1;
            let leader_idx = new_species[i].organisms[u as usize];
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
        //only use the highest performing members of each species to reproduce
        //If the species has > 5 networks then keep the champion
        let mut best_ones: Vec<(f64, i32)> = vec![];
        for i in 0..fitness.len() {
            best_ones.push((fitness[i], i as i32));
        }
        best_ones.sort_by(|a, b| b.partial_cmp(a).unwrap());
        let top_members: i32 = ((curr_gen.len() as f64) * 0.35).ceil() as i32;
        let mut champion_flag: i32 = 0;
        let mut new_gen: Vec<Genome> = vec![];
        if curr_gen.len() > 5 {
            //champion_flag = ((curr_gen.len() as f64) * 0.2).floor() as i32;
            champion_flag = 1;
            if champion_flag > number_offspring {
                champion_flag = number_offspring;
            }
            for k in 0..champion_flag {
                new_gen.push(curr_gen[best_ones[k as usize].1 as usize].clone())
            }
        }
        for _ in 0..(number_offspring - champion_flag) {
            assert!(top_members > 0);
            let mut u = (rand_i32(1, top_members) - 1) as usize;
            let mut v = (rand_i32(1, top_members) - 1) as usize;
            if best_ones[u].0 < best_ones[v].0 {
                let t = u;
                u = v;
                v = t;
            }
            assert!(best_ones[u].0 >= best_ones[v].0);
            u = best_ones[u].1 as usize;
            v = best_ones[v].1 as usize;
            new_gen.push(self.breed(&curr_gen[u], &curr_gen[v]));
        }
        new_gen
    }

    pub fn next_generation(&mut self, fitness: &mut Vec<f64>) {
        //population stores the current generation with an input of fitness values
        //create a new gereration after specification
        assert_eq!(fitness.len(), self.population.len());
        if self.gen == 0 {
            self.previous_gen = self.speciate(&self.population);
        }
        println!(" number of species {}", self.previous_gen.len());
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
        let mut number_offspring: Vec<i32> = vec![];
        let mut total: i32 = 0;
        let mut ss: Vec<(f64, i32)> = vec![];
        for i in 0..self.previous_gen.len() {
            ss.push((species_fitness[i], i as i32));
            let val: i32 =
                (species_fitness[i] * (self.population.len() as f64) / sum_fitness).floor() as i32;
            total += val;
            number_offspring.push(val);
        }
        ss.sort_by(|a, b| b.partial_cmp(a).unwrap());
        let mut idx: usize = 0;
        while total < self.population.len() as i32 {
            let numb = self.previous_gen[ss[idx].1 as usize].organisms.len() == 0;
            if numb || idx == self.previous_gen.len() {
                idx = 0;
                continue;
            }
            number_offspring[ss[idx].1 as usize] += 1;
            idx += 1;
            total += 1;
        }
        let mut new_gen: Vec<Genome> = vec![];
        let mut idx: usize = 0;
        for s in &self.previous_gen {
            if s.organisms.len() == 0 || number_offspring[idx] == 0 {
                idx += 1;
                continue;
            }
            let mut curr: Vec<&Genome> = vec![];
            let mut fit: Vec<f64> = vec![];
            for a in &s.organisms {
                curr.push(&self.population[*a as usize]);
                fit.push(fitness[*a as usize]);
            }
            let adding = self.create_species(&curr, &fit, number_offspring[idx]);
            new_gen.extend(adding);
            idx += 1;
        }

        assert_eq!(new_gen.len(), self.population.len());
        for i in 0..new_gen.len() {
            if chance(0.03) {
                self.mutate(&mut new_gen[i]);
            }
        }
        self.previous_gen = self.speciate(&new_gen);
        self.population = new_gen;
        self.gen += 1;
    }
}
