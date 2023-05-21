use std::collections::BTreeMap;
struct Node {
    key: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(key: i32) -> Self {
        return Self {
            key: key,
            left: None,
            right: None,
        };
    }

    fn add(&mut self, key: i32) {
        if key < self.key {
            if let Some(ref mut node) = self.left {
                node.add(key);
            } else {
                self.left = Some(Box::new(Node::new(key)));
            }
        } else {
            if let Some(ref mut node) = self.right {
                node.add(key);
            } else {
                self.right = Some(Box::new(Node::new(key)));
            }
        }
    }

    fn dfs(&self) {
        if let Some(ref node) = self.left {
            node.dfs();
        }
        //println!("{}", self.key);
        if let Some(ref node) = self.right {
            node.dfs();
        }
    }
}

fn main() {
    let key = (12, 12);
    println!("{}", key.0);
}
