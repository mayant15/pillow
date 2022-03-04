use super::lexer::ExprParser;
use lazy_static::lazy_static;
use crate::graph::{Tree, TreeNode, TreeEdge};
use std::boxed::Box;
use lalrpop_util::{
    ParseError,
    lexer::Token,
};

pub type ASTNode = TreeNode;
pub type AST = Tree<ENodeData>;

/// Binary operations in order of decreasing precedence
#[derive(Debug)]
pub enum EBinaryOp {
    Mul, Div,
    Add, Sub,
}

#[derive(Debug)]
pub enum ENodeData {
    Number(i32),
    BinaryOp(EBinaryOp),
}

lazy_static! {
    static ref AST_DATA: AST = AST::new();
}

fn add_node(data: ENodeData) -> TreeNode {
    AST_DATA.add_node(data)
}

fn add_edge(from: TreeNode, to: TreeNode) -> TreeEdge {
    AST_DATA.add_edge(from, to)
}

pub fn parse_binary_expr<'a, L, T>(lhs: ASTNode, op: EBinaryOp, rhs: ASTNode) -> Result<ASTNode, ParseError<L, T, &'a str>> {
    let binop_node = add_node(ENodeData::BinaryOp(op));
    add_edge(binop_node, lhs);
    add_edge(binop_node, rhs);

    // NOTE: Is this still valid if it is moved above?
    return Ok(binop_node);
}

pub fn parse_num_expr<'a, L, T>(s: &str) -> Result<ASTNode, ParseError<L, T, &'a str>> {
    i32::from_str_radix(s, 10)
            .map_err(|_| ParseError::User {
                error: "Failed to parse numeric literal"
            })
    .and_then(|num| {
        Ok(add_node(ENodeData::Number(num)))
    })
}

pub fn parse<'a>(program: &'a str) -> Result<ASTNode, ParseError<usize, Token<'a>, &'static str>> {
    let root = ExprParser::new().parse(program);
    return root;
}

