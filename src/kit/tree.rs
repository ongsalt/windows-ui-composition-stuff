use std::{collections::HashMap, rc::Rc};

use crate::kit::node::Node;

#[derive(Debug, Clone, Hash)]
pub enum NodeId {
    Auto(u64),
    Specified(String),
}

type EventType = u64;

// This should be in RenderContext
// we should rename it to
pub struct Tree {
    nodes: HashMap<NodeId, Rc<dyn Node>>,
    children: HashMap<NodeId, Vec<NodeId>>,
    // other stuff should be here too
    event_listener: HashMap<EventType, Vec<NodeId>>,
}

impl Tree {
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
            nodes: HashMap::new(),
            event_listener: HashMap::new(),
        }
    }

    // pub fn
}
