use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};
use taffy::{NodeId, PrintTree, Size, Style, TaffyTree, prelude::TaffyMaxContent};
use windows::UI::Composition::{Compositor, ContainerVisual};
use windows::core::Interface;

use crate::{
    composition_host::CompositionHost,
    kit::{attribute::Attribute, layout::list_dirty_node_and_relayout, node::Node},
};

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
*/

#[derive(Debug)]
pub struct Tree {
    composition_host: CompositionHost,
    // what tree tho, BackingTree?
    root_id: Option<NodeId>,
    pub nodes: HashMap<NodeId, Node>,
    children: HashMap<NodeId, Vec<NodeId>>,
    // other stuff should be here too
    event_listener: HashMap<EventType, Vec<NodeId>>,

    pub layout: TaffyTree,
    // for diff/hittest
    pub previous_node_layouts: HashMap<NodeId, taffy::Layout>,
}

impl Tree {
    pub fn new(composition_host: CompositionHost) -> Self {
        Self {
            composition_host,
            children: HashMap::new(),
            nodes: HashMap::new(),
            event_listener: HashMap::new(),
            layout: TaffyTree::new(),
            previous_node_layouts: HashMap::new(),
            root_id: None,
        }
    }

    pub fn create_root(&mut self) -> NodeId {
        let id = self.new_div();
        self.root_id = Some(id);
        let root = self.nodes.get(&id).unwrap();

        let container: ContainerVisual =
            self.composition_host.target.Root().unwrap().cast().unwrap();
        container
            .Children()
            .unwrap()
            .InsertAtTop(&root.visual())
            .unwrap();

        // println!(
        //     "{:.?}",
        //     container
        //         .Children()
        //         .unwrap()
        //         .First()
        //         .unwrap()
        //         .Current()
        //         .unwrap()
        // );
        id
    }

    pub fn new_div(&mut self) -> NodeId {
        // TODO: handle collision
        let node_id = self.layout.new_with_children(Style::DEFAULT, &[]).unwrap();
        let node = Node::new_div(&self.composition_host.compositor);

        self.nodes.insert(node_id, node);

        node_id
    }

    pub fn new_leaf(&mut self) -> NodeId {
        let node_id = self.layout.new_leaf(Style::DEFAULT).unwrap();
        let node = Node::new_leaf(&self.composition_host.compositor);

        self.nodes.insert(node_id, node);

        node_id
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

        let children = self.children.entry(parent_id).or_insert(vec![]);
        children.push(child_id);
        self.layout.add_child(parent_id, child_id).unwrap();

        Ok(())
    }

    pub fn set_attribute(&mut self, node_id: NodeId, attribute: Attribute) {
        // TODO: remove unwrap
        let node = self.nodes.get_mut(&node_id).unwrap();

        if attribute.is_taffy_style() {
            let mut style = self.layout.style(node_id).unwrap().clone();
            // TODO: might make patch_taffy_style consuming
            attribute.patch_taffy_style(&mut style);
            self.layout.set_style(node_id, style).unwrap();

            self.invalidate_layout(node_id);
            // its dirty now
        } else {
            // TODO: deal with non layout stuff
            // so we need to do text layout in userland?
        }

        // self.layout.mark_dirty(node)
        // https://github.com/DioxusLabs/taffy/blob/main/examples/measure.rs
        // self.layout.compute_layout_with_measure(*layout_id, Size::MAX_CONTENT, |a| {
        // });
    }

    // TODO: Append after anchor
    // TODO: remove child at index

    // Transaction api?

    pub fn invalidate_layout(&mut self, node_id: NodeId) {
        let damaged_nodes = list_dirty_node_and_relayout(
            &mut self.layout,
            node_id,
            &mut self.previous_node_layouts,
        );

        println!("Damaged: {:.?}", damaged_nodes);
        for node_id in damaged_nodes {
            let node = self.nodes.get_mut(&node_id).unwrap();
            let layout = self.previous_node_layouts.get(&node_id).unwrap();

            // update the layout
            node.apply_layout(layout);
            // node
        }
    }
}

pub enum TreeError {
    NodeNotFound { node: NodeId },
    NotContainer { node: NodeId },
}
