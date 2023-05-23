use std::cell::RefCell;
use std::rc::Rc;

pub struct Connection {
    pub to: Rc<RefCell<Node>>,
    pub inno_number: i32,
    pub active: bool,
    pub weight: f64,
}

pub struct Node {
    pub global_id: i32,
    pub local_id: i32,
    pub adj: Vec<Connection>,
    pub active_edges: i32,
    pub inactive_edges: i32,
    pub act: fn(f64) -> f64,
}

impl Node {
    pub fn new(local_id: i32, global_id: i32, act: fn(f64) -> f64) -> Self {
        return Self {
            act: act,
            global_id: global_id,
            local_id: local_id,
            adj: vec![],
            active_edges: 0,
            inactive_edges: 0,
        };
    }

    pub fn edge_exist(&self, to: i32) -> bool {
        for e in &self.adj {
            let v = e.to.borrow();
            if v.local_id == to {
                return true;
            }
        }
        return false;
    }

    pub fn evaluate(&self, x: f64) -> f64 {
        (self.act)(x)
    }

    pub fn add_edge(&mut self, inno_number: i32, weight: f64, active: bool, to: Rc<RefCell<Node>>) {
        self.adj
            .push(Connection::new(inno_number, weight, active, to));
    }

    pub fn disable_edge(&mut self, to: i32) {
        for e in &mut self.adj {
            let v = e.to.borrow_mut();
            if v.local_id == to {
                e.active = false;
            }
        }
    }

    pub fn enable_edge(&mut self, to: i32) {
        for e in &mut self.adj {
            let v = e.to.borrow_mut();
            if v.local_id == to {
                e.active = true;
            }
        }
    }

    pub fn edge_weight(&self, to: i32) -> f64 {
        for e in &self.adj {
            let v = e.to.borrow_mut();
            if v.local_id == to {
                return e.weight;
            }
        }
        return -1.0;
    }

    pub fn del_back(&mut self) {
        self.adj.pop();
    }
}

impl Connection {
    pub fn new(inno_number: i32, weight: f64, active: bool, to: Rc<RefCell<Node>>) -> Self {
        return Self {
            inno_number: inno_number,
            to: to,
            weight: weight,
            active: active,
        };
    }
}
