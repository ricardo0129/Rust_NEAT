use crate::constants::MAX_WEIGHT;
use crate::helper::{pertube, rand_f64, rand_i32};
use crate::node::Node;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::rc::Rc;

pub struct Genome {
    pub input_nodes: i32,
    pub output_nodes: i32,
    pub hidden_nodes: i32,
    pub nodes: Vec<Rc<RefCell<Node>>>,
    pub num_nodes: i32,
    pub num_connections: i32,
    pub edges: BTreeSet<(i32, i32)>,
    pub act: fn(f64) -> f64,
}

pub struct GenomeInfo {
    pub from: i32,
    pub to: i32,
    pub innovation_number: i32,
    pub weight: f64,
    pub active: bool,
}

impl Genome {
    pub fn new(input_nodes: i32, output_nodes: i32, act: fn(f64) -> f64) -> Self {
        //Initialize a Genome with no hidden nodes
        //and all Inputs connected to all Outputs
        let num_nodes = input_nodes + output_nodes + 1;
        let mut nodes: Vec<Rc<RefCell<Node>>> = vec![];
        for i in 0..(num_nodes) {
            nodes.push(Rc::new(RefCell::new(Node::new(i, i, act))));
        }
        return Self {
            nodes: nodes,
            input_nodes: input_nodes,
            output_nodes: output_nodes,
            hidden_nodes: 0,
            num_nodes: num_nodes,
            num_connections: 0,
            act: act,
            edges: BTreeSet::new(),
        };
    }

    pub fn un_flatten(
        genes: &Vec<GenomeInfo>,
        input_nodes: i32,
        output_nodes: i32,
        act: fn(f64) -> f64,
    ) -> Self {
        let mut base: Genome = Genome::new(input_nodes, output_nodes, act);
        let mut unique: BTreeSet<i32> = BTreeSet::new();
        let mut mapping: BTreeMap<i32, i32> = BTreeMap::new();
        for g in genes {
            unique.insert(g.from);
            unique.insert(g.to);
        }
        for g in unique {
            mapping.insert(g, mapping.len() as i32);
            if g >= input_nodes + output_nodes + 1 {
                base.add_node(g);
            }
        }
        for g in genes {
            let u = *mapping.get(&g.from).unwrap();
            let v = *mapping.get(&g.to).unwrap();
            base.add_edge(u, v, g.innovation_number, g.weight, g.active);
        }
        return base;
    }

    pub fn flatten(&self) -> Vec<GenomeInfo> {
        let mut genes: Vec<GenomeInfo> = vec![];
        for n in &self.nodes {
            for edge in &n.borrow().adj {
                genes.push(GenomeInfo {
                    from: n.borrow().global_id,
                    to: edge.to.borrow().global_id,
                    innovation_number: edge.inno_number,
                    weight: edge.weight,
                    active: edge.active,
                });
            }
        }
        genes.sort_by_key(|s| s.innovation_number);
        genes
    }

    pub fn clone(&self) -> Self {
        //Implemented from flatten/unflatten functions
        Genome::un_flatten(
            &self.flatten(),
            self.input_nodes,
            self.output_nodes,
            self.act,
        )
    }

    pub fn connect_ends(&mut self) {
        for i in 0..(self.input_nodes + 1) {
            for j in 0..self.output_nodes {
                self.add_edge(
                    i,
                    j + self.input_nodes + 1,
                    i * self.output_nodes + j,
                    rand_f64(-MAX_WEIGHT, MAX_WEIGHT),
                    true,
                );
            }
        }
    }

    pub fn permute_weights(&mut self) {
        for n in &self.nodes {
            for edges in &mut n.borrow_mut().adj {
                edges.weight = pertube(edges.weight);
            }
        }
    }

    pub fn new_weights(&mut self) {
        for n in &self.nodes {
            for edges in &mut n.borrow_mut().adj {
                edges.weight = rand_f64(-MAX_WEIGHT, MAX_WEIGHT);
            }
        }
    }

    pub fn random_disable(&mut self) -> bool {
        if self.edges.len() == 0 {
            return false;
        }
        let mut idx = rand_i32(0, self.edges.len() as i32 - 1);
        //Iterating over all elements since self.edges is a BST
        for e in self.edges.iter() {
            if idx == 0 {
                self.disable_edge(e.0, e.1);
                return true;
            }
            idx -= 1;
        }
        return false;
    }

    pub fn node_exists(&self, inno_number: i32) -> bool {
        for n in &self.nodes {
            if n.borrow().global_id == inno_number {
                return true;
            }
        }
        return false;
    }

    pub fn add_node(&mut self, inno_number: i32) -> i32 {
        self.nodes.push(Rc::new(RefCell::new(Node::new(
            self.num_nodes,
            inno_number,
            self.act,
        ))));
        self.num_nodes += 1;
        self.num_nodes - 1
    }

    pub fn add_edge(&mut self, from: i32, to: i32, inno_number: i32, weight: f64, active: bool) {
        assert!(from != to);
        if from != self.input_nodes {
            //Since we wont split bias edges dont add to edge set
            self.edges.insert((from, to));
        }
        self.num_connections += 1;
        self.nodes[from as usize].borrow_mut().add_edge(
            inno_number,
            weight,
            active,
            self.nodes[to as usize].clone(),
        );
    }

    pub fn rm_last(&mut self, from: i32, to: i32) {
        self.num_connections -= 1;
        self.edges.remove(&(from, to));
        self.nodes[from as usize].borrow_mut().del_back();
    }

    pub fn check_edge(&mut self, u_id: i32, v_id: i32) -> bool {
        self.edges.contains(&(u_id, v_id))
    }

    pub fn edge_exist(&self, from: i32, to: i32) -> bool {
        self.nodes[from as usize].borrow().edge_exist(to)
    }

    pub fn disable_edge(&mut self, from: i32, to: i32) {
        self.edges.remove(&(from, to));
        self.num_connections -= 1;
        self.nodes[from as usize].borrow_mut().disable_edge(to);
    }

    pub fn enable_edge(&mut self, from: i32, to: i32) {
        //never add a bias edge to the edge set since it will try to split it
        if from != self.input_nodes {
            self.edges.insert((from, to));
        }
        self.num_connections += 1;
        self.nodes[from as usize].borrow_mut().enable_edge(to);
    }

    pub fn split_edge(&mut self, from: i32, to: i32, inno_number: i32, new_node_id: i32) {
        if self.node_exists(new_node_id) {
            return;
        }
        self.disable_edge(from, to);
        let old_weight = self.nodes[from as usize].borrow().edge_weight(to);
        let id = self.add_node(new_node_id);
        self.add_edge(from, id, inno_number, 1.0, true);
        self.add_edge(id, to, inno_number + 1, old_weight, true);
    }

    pub fn random_edge(&mut self) -> (i32, i32) {
        for _ in 0..100 {
            //try a random edge if after 100 attempts then ignore
            //TODO change this to a more optimal way of finding random edges
            let hidden_nodes = self.num_nodes - self.input_nodes - self.output_nodes - 1;
            let mut u = rand_i32(0, self.input_nodes + hidden_nodes);
            if u > self.input_nodes {
                u += self.output_nodes;
            }
            let v = self.input_nodes + 1 + rand_i32(0, self.output_nodes + hidden_nodes - 1);
            if u == v || self.check_edge(u, v) {
                continue;
            }
            //weight irrelevant since we are just checking for cycles
            self.add_edge(u, v, -1, 1.0, true);
            let cycle: bool = self.check_cycle();
            self.rm_last(u, v);
            if cycle {
                continue;
            } else {
                return (u, v);
            }
        }
        return (-1, -1);
    }

    pub fn local_to_global(&self, local_id: i32) -> i32 {
        self.nodes[local_id as usize].borrow().global_id
    }

    pub fn random_split(&mut self) -> (i32, i32) {
        //Return a pair of local ids (u, v) that could be split by adding a new edge in the middle
        if self.edges.len() == 0 {
            return (-1, -1);
        }
        let mut idx = rand_i32(0, self.edges.len() as i32 - 1);
        //Iterating over all elements since self.edges is a BST
        for e in self.edges.iter() {
            if idx == 0 {
                return (e.0, e.1);
            }
            idx -= 1;
        }
        return (-1, -1);
    }

    pub fn evaluate(&self, input: &Vec<f64>) -> Vec<f64> {
        //Use topological sorting to evaluate outputs of the network
        //given an input vector
        let mut in_deg: Vec<i32> = vec![0; self.num_nodes as usize];
        let mut node_values: Vec<f64> = vec![0.0; self.num_nodes as usize];

        for i in 0..input.len() {
            node_values[i] = input[i];
        }
        //bias node
        node_values[self.input_nodes as usize] = 1.0;

        for u in &self.nodes {
            for v in &u.borrow().adj {
                if !v.active {
                    continue;
                }
                //println!("{}", v.to.borrow().local_id);
                let id: i32 = v.to.borrow().local_id;
                in_deg[id as usize] += 1;
            }
        }
        let mut q: VecDeque<i32> = VecDeque::new();
        for u in 0..self.num_nodes {
            if in_deg[u as usize] == 0 {
                q.push_back(u);
            }
        }

        while q.len() != 0 {
            let u: i32 = q.pop_front().unwrap();
            for neighboor in &self.nodes[u as usize].borrow().adj {
                if !neighboor.active {
                    continue;
                }
                let v = neighboor.to.borrow();
                node_values[v.local_id as usize] += node_values[u as usize] * neighboor.weight;
                in_deg[v.local_id as usize] -= 1;
                if in_deg[v.local_id as usize] == 0 {
                    node_values[v.local_id as usize] = v.evaluate(node_values[v.local_id as usize]);
                    q.push_back(v.local_id);
                }
            }
        }

        return (&node_values[(self.input_nodes + 1) as usize
            ..(self.input_nodes + self.output_nodes + 1) as usize])
            .to_vec();
    }

    pub fn check_cycle(&mut self) -> bool {
        //Returns if there exists a directed cycle in the graph
        let mut color: Vec<i32> = vec![0; self.num_nodes as usize];
        let mut q: Vec<i32> = vec![];
        for i in 0..self.num_nodes {
            if color[i as usize] != 0 {
                continue;
            }
            q.push(i);
            while q.len() != 0 {
                let v: usize = (*q.last().unwrap()) as usize;
                if color[v] != 1 {
                    color[v] = 1;
                    for e in &self.nodes[v].borrow().adj {
                        if !e.active {
                            continue;
                        }
                        let w = e.to.borrow().local_id;
                        let c = color[w as usize];
                        //println!("{v} -> {w}");
                        if c == 0 {
                            q.push(w);
                        } else if c == 1 {
                            return true;
                        }
                    }
                } else if color[v] == 1 {
                    q.pop();
                    color[v] = 2;
                }
            }
        }
        return false;
    }

    pub fn network_info(&self) {
        println!(
            "#Nodes {}, #Edges {}, #Active Connections {}",
            self.num_nodes,
            self.num_connections,
            self.edges.len(),
        );
        for i in 0..self.nodes.len() {
            println!("{}", self.nodes[i].borrow().global_id);
        }
        for n in &self.nodes {
            for e in &n.borrow().adj {
                println!("{} {}", n.borrow().global_id, e.to.borrow().global_id)
            }
        }
    }
}
