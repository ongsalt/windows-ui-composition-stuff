use std::{collections::HashMap, rc::Rc};

use crate::kit::node::Node;

#[derive(Debug, Clone, Hash)]
pub enum NodeId {
    Auto(u64),
    Specified(String),
}

// This should be in RenderContext
// we should rename it to
pub struct Tree {
    nodes: HashMap<NodeId, Rc<dyn Node>>,
    children: HashMap<NodeId, Vec<NodeId>>,
    // other stuff should be here too
}

impl Tree {
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
            nodes: HashMap::new(),
        }
    }

    // pub fn 
}
