// https://rust-leipzig.github.io/architecture/2016/12/20/idiomatic-trees-in-rust/

use std::fmt::Debug;
use std::vec::Vec;

/// Expose nodes through IDs only
pub type TreeNode = usize;

/// Actual data storage for tree nodes
#[derive(Debug)]
struct NodeImpl<T> {
    data: T,
    parent: Option<TreeNode>,
    children: Vec<TreeNode>,
}

/// I would like to be able to merge trees very often, so keep nodes as pure data and manipulate
/// them through free functions that take both node and arena
pub struct TreeArena<T> {
    nodes: Vec<NodeImpl<T>>,
}

impl<T> TreeArena<T> {
    pub fn new() -> Self {
        TreeArena { nodes: Vec::new() }
    }

    fn add(&mut self, data: T) -> Option<TreeNode> {
        let node = NodeImpl {
            data,
            children: Vec::new(),
            parent: None,
        };
        self.nodes.push(node);
        Some(self.nodes.len() - 1)
    }

    fn set_parent(&mut self, node_index: TreeNode, parent_index: TreeNode) {
        match self.nodes.get_mut(node_index) {
            Some(node) => node.parent = Some(parent_index),
            None => (),
        };
    }

    fn add_child(&mut self, node_index: TreeNode, parent_index: TreeNode) {
        match self.nodes.get_mut(parent_index) {
            Some(parent) => parent.children.push(node_index),
            None => (),
        }
    }
}

/// Create a new node in arena
pub fn create<T>(arena: &mut TreeArena<T>, data: T) -> Option<TreeNode> {
    arena.add(data)
}

/// Attach a given node to a given parent
pub fn attach_to<T>(
    arena: &mut TreeArena<T>,
    parent: TreeNode,
    node: TreeNode,
) -> Option<TreeNode> {
    arena.set_parent(node, parent);
    arena.add_child(node, parent);
    Some(node)
}

/// Create a new node and add it as a child in arena
/// Returns the newly created node
#[allow(dead_code)]
pub fn add_child<T>(arena: &mut TreeArena<T>, parent: TreeNode, data: T) -> Option<TreeNode> {
    arena
        .add(data)
        .and_then(|node| attach_to(arena, parent, node))
}

pub fn print_tree<T>(arena: &TreeArena<T>, node: &TreeNode, indent: u8)
where
    T: Debug,
{
    for _ in 1..indent {
        print!("  ");
    }

    let index = node.clone();
    println!("{:?}", arena.nodes[index].data);
    for node in arena.nodes[index].children.iter() {
        print_tree(arena, node, indent + 1)
    }
}
