use std::collections::HashSet;
use taffy::prelude::*;

/*
i want all node that need to be resize
taffy will report some (only those that is ancestor of damaged node)
in some case such as flex grow damaged node can cause the sibling to be
resize too.

so we need to remember every node sizing info, and diff all those...

start diffing at the highest damaged node
*/
pub fn list_dirty_node(
    tree: &mut TaffyTree,
    node_hint: taffy::NodeId,
    previous_node_layouts: &mut HashSet<taffy::NodeId, taffy::Layout>,
) -> Vec<NodeId> {
    // get guess?

    // firsy
    vec![]
}

pub fn diff() {}

// fuck, we still need to diff the entire subtree anyways
pub fn list_dirty_node_nonono(tree: &TaffyTree, start_at: taffy::NodeId) -> Vec<taffy::NodeId> {
    fn traverse(
        tree: &TaffyTree,
        current: taffy::NodeId,
        visited: &mut HashSet<taffy::NodeId>,
        dirty_nodes: &mut Vec<taffy::NodeId>,
    ) {
        if visited.contains(&current) {
            return;
        }
        visited.insert(current);

        if !tree.dirty(current).unwrap() {
            return;
        }
        dirty_nodes.push(current);

        let parent = tree.parent(current).unwrap();
        traverse(tree, parent, visited, dirty_nodes);

        for c in tree.children(current).unwrap() {
            traverse(tree, c, visited, dirty_nodes);
        }
    }

    let mut nodes = HashSet::new();
    let mut dirty_nodes = vec![];
    traverse(tree, start_at, &mut nodes, &mut dirty_nodes);

    dirty_nodes
}
