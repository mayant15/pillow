use std::ops::{Deref, DerefMut};
use std::fmt::Debug;
use std::println;
use petgraph::{
    graph::{DiGraph, NodeIndex, EdgeIndex},
    dot::{Dot, Config},
};

pub struct Tree<N: Debug> {
    data: DiGraph<N, ()>
}

pub type TreeNode = NodeIndex;
pub type TreeEdge = EdgeIndex;

impl<N: Debug> Deref for Tree<N> {
    type Target = DiGraph<N, ()>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<N: Debug> DerefMut for Tree<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<N: Debug> Tree<N> {
    pub fn new() -> Self {
        Tree {
            data: DiGraph::<N, ()>::new()
        }
    }

    pub fn print(&self) {
        println!("{:?}", Dot::with_config(&self.data, &[Config::EdgeNoLabel]));
    }

    pub fn add_node(&mut self, data: N) -> TreeNode {
        self.data.add_node(data)
    }

    pub fn add_edge(&mut self, from: TreeNode, to: TreeNode) -> TreeEdge {
        self.data.add_edge(from, to, ())
    }
}

