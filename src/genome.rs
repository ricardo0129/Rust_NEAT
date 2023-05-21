use crate::node::Node;
use rand::Rng;
use std::cell::RefCell;
use std::collections::{BTreeSet, VecDeque};
use std::rc::Rc;

pub struct Genome {
    input_nodes: i32,
    output_nodes: i32,
    hidden_nodes: i32,
    nodes: Vec<Rc<RefCell<Node>>>,
    num_nodes: i32,
    num_connections: i32,
    edges: BTreeSet<(i32, i32)>,
}

pub struct GenomeInfo {
    from: i32,
    to: i32,
    innovation_number: i32,
    weight: f64,
    disable_bit: bool,
}

impl Genome {
    pub fn new(input_nodes: i32, output_nodes: i32) -> Self {
        //Initialize a Genome with no hidden nodes
        //and all Inputs connected to all Outputs
        let num_nodes = input_nodes + output_nodes;
        let num_connections = input_nodes * output_nodes;
        let mut nodes: Vec<Rc<RefCell<Node>>> = vec![];
        for i in 0..(input_nodes + output_nodes) {
            nodes.push(Rc::new(RefCell::new(Node::new(i, i))));
        }
        return Self {
            nodes: nodes,
            input_nodes: input_nodes,
            output_nodes: output_nodes,
            hidden_nodes: 0,
            num_nodes: num_nodes,
            num_connections: num_connections,
            edges: BTreeSet::new(),
        };
    }

    pub fn from_genes(genes: Vec<GenomeInfo>, input_nodes: i32, output_nodes: i32) -> Self {
        let mut base: Genome = Genome::new(input_nodes, output_nodes);
        let mut unique: BTreeSet<i32> = BTreeSet::new();
        for g in genes {
            unique.insert(g.from);
            unique.insert(g.to);
        }
        for g in unique {
            if g >= input_nodes + output_nodes {
                base.add_node(g);
            }
        }
        return base;
    }

    pub fn flatten(&mut self) -> Vec<GenomeInfo> {
        let genes: Vec<GenomeInfo> = vec![];
        genes
    }

    pub fn connect_ends(&mut self) {
        for i in 0..self.input_nodes {
            for j in 0..self.output_nodes {
                self.add_edge(
                    i,
                    j + self.input_nodes,
                    i * self.output_nodes + j,
                    1.0,
                    true,
                );
            }
        }
    }

    pub fn add_node(&mut self, inno_number: i32) -> i32 {
        self.nodes.push(Rc::new(RefCell::new(Node::new(
            self.num_nodes,
            inno_number,
        ))));
        self.num_nodes += 1;
        self.num_nodes - 1
    }

    pub fn add_edge(&mut self, from: i32, to: i32, inno_number: i32, weight: f64, active: bool) {
        self.edges.insert((from, to));
        self.nodes[from as usize].borrow_mut().add_edge(
            inno_number,
            weight,
            active,
            self.nodes[to as usize].clone(),
        );
    }

    pub fn rm_last(&mut self, from: i32) {
        self.nodes[from as usize].borrow_mut().del_back();
    }

    pub fn check_edge(&mut self, u_id: i32, v_id: i32) -> bool {
        self.edges.contains(&(u_id, v_id))
    }

    pub fn disable_edge(&mut self, from: i32, to: i32) {
        self.edges.remove(&(from, to));
        self.nodes[from as usize].borrow_mut().disable_edge(to);
    }

    pub fn split_edge(&mut self, from: i32, to: i32, inno_number: i32, new_node_id: i32) {
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
            let mut u = rand::thread_rng().gen_range(0..(self.input_nodes + self.hidden_nodes));
            if u >= self.input_nodes {
                u += self.output_nodes;
            }
            let v = self.input_nodes
                + rand::thread_rng().gen_range(0..(self.output_nodes + self.hidden_nodes));
            if u == v || self.check_edge(u, v) {
                continue;
            }
            self.add_edge(u, v, -1, 1.0, true);
            let cycle: bool = self.check_cycle();
            self.rm_last(u);
            if cycle {
                //println!("Bad Edge");
                continue;
            } else {
                println!("Add Edge {u} {v}");
                //self.add_edge(u, v);
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
        let mut idx = rand::thread_rng().gen_range(0..self.edges.len() as usize);
        for e in self.edges.iter() {
            if idx == 0 {
                return (e.0, e.1);
            }
            idx -= 1;
        }
        return (-1, -1);
    }

    pub fn evaluate(&self, input: Vec<f64>) -> Vec<f64> {
        //Use topological sorting to evaluate outputs of the network
        //given an input vector
        let mut in_deg: Vec<i32> = vec![0; self.num_nodes as usize];
        let mut node_values: Vec<f64> = vec![0.0; self.num_nodes as usize];

        for i in 0..input.len() {
            node_values[i] = input[i];
        }

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
                    q.push_back(v.local_id);
                }
            }
        }

        return (&node_values
            [self.input_nodes as usize..(self.input_nodes + self.output_nodes) as usize])
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
}
