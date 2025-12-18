use std::collections::{HashMap, HashSet};
use taffy::prelude::*;

/*
i want all node that need to be resize
taffy will report some (only those that is ancestor of damaged node)
in some case such as flex grow damaged node can cause the sibling to be
resize too.

so we need to remember every node sizing info, and diff all those...

start diffing at the highest damaged node
*/
pub fn list_dirty_node_and_relayout(
    tree: &mut TaffyTree,
    // should it be array tho
    node_hint: NodeId, // this must be dirty/changed by our code not taffy
    previous_node_layouts: &mut HashMap<NodeId, taffy::Layout>,
) -> Vec<NodeId> {
    // if first_run { compute_the_entire_layouts }

    // first, get highest damaged node
    let damaged_root = {
        let mut node = node_hint;
        loop {
            let Some(parent) = tree.parent(node) else {
                break;
            };
            if !tree.dirty(parent).unwrap() {
                break;
            }
            node = parent;
        }
        node
    };

    // recompute
    // TODO: text measuring
    tree.compute_layout(damaged_root, Size::MAX_CONTENT)
        .unwrap();

    let mut damaged_nodes = vec![];
    // We can ignore deletion becuase it was handle in other fn,
    // mutation, addition will be mark as dirty
    // why didnt i just make this recursive
    let mut to_diff: Vec<NodeId> = vec![damaged_root];

    while let Some(node) = to_diff.pop() {
        let layout = tree.layout(node).unwrap();
        if let Some(previous) = previous_node_layouts.get(&node)
            && layout == previous
        {
            continue;
        };

        previous_node_layouts.insert(node, *layout);
        damaged_nodes.push(node);
        let children = tree.children(node).unwrap();
        to_diff.extend_from_slice(&children);
    }

    // TODO: what if damaged_root is not dirty
    damaged_nodes
}
