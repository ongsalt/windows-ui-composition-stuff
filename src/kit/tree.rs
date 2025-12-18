use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use taffy::{PrintTree, Size, Style, TaffyTree, prelude::TaffyMaxContent};
use windows::UI::Composition::Compositor;

use crate::kit::{attribute::Attribute, node::Node};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
pub struct NodeId(u64);

impl NodeId {
    pub fn new() -> Self {
        Self(rand::random())
    }
}

type EventType = u64;

// This should be in RenderContext

/*
There is no actual DomNode struct,
each field group in DomNode will be stored seperately

// just node handle
let div_handle = tree.new_node()
let leaf_handle = tree.new_leaf()

how many fundemental node type will there be?
Div, TextNode, VisualNode (for image/canvas)

Comment and TextNode can be implement in userland

text node is visual node tho

tree.insert_child(parent_handle, div)

how do i do comment tho
display: none?


this tree manage event_listener/layout/hittest
TODO: hit test

how do i implement tree.update_style?
*/
pub struct Tree {
    compositor: Compositor,
    // what tree tho, BackingTree?
    nodes: HashMap<NodeId, Node>,
    children: HashMap<NodeId, Vec<NodeId>>,
    // other stuff should be here too
    event_listener: HashMap<EventType, Vec<NodeId>>,

    // all set size ops should target this
    layout_id_map: HashMap<NodeId, taffy::NodeId>,
    layout: TaffyTree,
}

impl Tree {
    pub fn new(compositor: Compositor) -> Self {
        Self {
            compositor,
            children: HashMap::new(),
            nodes: HashMap::new(),
            event_listener: HashMap::new(),
            layout_id_map: HashMap::new(),
            layout: TaffyTree::new(),
        }
    }

    pub fn new_div(&mut self) -> NodeId {
        // TODO: handle collision
        let id = NodeId::new();
        let taffy_node = self.layout.new_with_children(Style::DEFAULT, &[]).unwrap();
        let node = Node::new_div(&self.compositor);

        self.nodes.insert(id, node);
        self.layout_id_map.insert(id, taffy_node);

        id
    }

    pub fn new_leaf(&mut self) -> NodeId {
        let id = NodeId::new();
        let taffy_node = self.layout.new_leaf(Style::DEFAULT).unwrap();
        let node = Node::new_leaf(&self.compositor);

        self.nodes.insert(id, node);
        self.layout_id_map.insert(id, taffy_node);

        id
    }

    pub fn append_child(&mut self, parent_id: NodeId, child_id: NodeId) -> Result<(), TreeError> {
        let Some(parent) = self.nodes.get(&parent_id) else {
            return Err(TreeError::NodeNotFound { node: parent_id });
        };
        if let Node::Leaf { .. } = parent {
            return Err(TreeError::NotContainer { node: parent_id });
        }
        if !self.nodes.contains_key(&child_id) {
            return Err(TreeError::NodeNotFound { node: child_id });
        };

        let t_parent = self.layout_id_map.get(&parent_id).unwrap();
        let t_child = self.layout_id_map.get(&child_id).unwrap();

        let children = self.children.entry(parent_id).or_insert(vec![]);
        children.push(child_id);
        self.layout.add_child(*t_parent, *t_child).unwrap();

        Ok(())
    }

    pub fn set_attribute(&mut self, node_id: NodeId, attribute: Attribute) {
        // TODO: remove unwrap
        let node = self.nodes.get_mut(&node_id).unwrap();
        let layout_id = self.layout_id_map.get(&node_id).unwrap();

        if attribute.is_taffy_style() {
            let mut style = self.layout.style(*layout_id).unwrap().clone();
            attribute.patch_taffy_style(&mut style);
            self.layout.set_style(*layout_id, style).unwrap();
        } else {
            // TODO: deal with non layout stuff
            // so we need to do text layout in userland?
            // how do i deal with this
        }

        // self.layout.mark_dirty(node)
        // https://github.com/DioxusLabs/taffy/blob/main/examples/measure.rs
        // self.layout.compute_layout_with_measure(*layout_id, Size::MAX_CONTENT, |a| {
        // });
    }

    // TODO: Append after anchor
    // TODO: remove child at index

    // Transaction api?
}

pub enum TreeError {
    NodeNotFound { node: NodeId },
    NotContainer { node: NodeId },
}

