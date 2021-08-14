// https://rust-leipzig.github.io/architecture/2016/12/20/idiomatic-trees-in-rust/

use std::vec::Vec;

#[derive(Debug)]
struct Arena<T> {
    nodes: Vec<NodeImpl<T>>,
}

impl<T> Arena<T> {
    fn new() -> Self {
        Arena { nodes: Vec::new() }
    }

    fn insert(&mut self, parent: Option<Node>, data: T) -> Node {
        let node = NodeImpl {
            data: data,
            children: Vec::new(),
            parent: parent,
        };
        self.nodes.push(node);
        return Node(self.nodes.len() - 1);
    }
}

/// A public interface to operate on tree nodes
#[derive(Clone, Copy, Debug)]
pub struct Node(usize);

/// Actual data storage for tree nodes
#[derive(Debug)]
struct NodeImpl<T> {
    data: T,
    parent: Option<Node>,
    children: Vec<Node>,
}

#[derive(Debug)]
pub struct Tree<T> {
    arena: Arena<T>,
    root: Option<Node>,
}

impl<T> Tree<T> {
    pub fn new() -> Self {
        Tree {
            arena: Arena::new(),
            root: None,
        }
    }

    pub fn has_root(&self) -> bool {
        self.root.is_some()
    }

    pub fn create_root(&mut self, data: T) -> Node {
        let node = self.arena.insert(None, data);
        self.root = Some(node.clone());
        return node;
    }

    pub fn insert_child(&mut self, parent: Node, data: T) -> Node {
        let node = self.arena.insert(Some(parent), data);
        match self.arena.nodes.get(parent.0) {
            Some(pnode) => pnode.children.push(node.clone()),
            None => panic!("No parent node found"),
        };
        return node;
    }

    pub fn merge(&mut self, node: Node, other: Tree<T>) -> Node {
        // TODO

        // Concatenate arenas
        // Update node ids on other
        // Connect node to other.root
    }
}
