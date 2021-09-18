use super::lexer::ExprParser;
use crate::tree::{TreeNode, TreeArena, create, attach_to, print_tree};
use std::ops::{Deref, DerefMut};
use std::sync::Mutex;
use lazy_static::lazy_static;
use lalrpop_util::{
    ParseError,
    lexer::Token,
};

/// Node for an expression in the AST
pub type AST = TreeNode;

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

// This is the only way I know to create a global mutable object :(
lazy_static! {
    static ref ARENA: Mutex<TreeArena<ENodeData>> = Mutex::new(TreeArena::new());
}

pub fn parse_binary_expr<'a, L, T>(lhs: AST, op: EBinaryOp, rhs: AST) -> Result<AST, ParseError<L, T, &'a str>> {
    ARENA.lock()
        .map_err(|_| ParseError::User {
            error: "Failed acquire AST mutex to parse binary expression"
        })
    .and_then(|mut guard| {
        let arena = guard.deref_mut();
        create(arena, ENodeData::BinaryOp(op))
            .and_then(|binop| {
                attach_to(arena, binop, lhs)
            })
        .and_then(|binop| {
            attach_to(arena, binop, rhs)
        }).ok_or(ParseError::User {
            error: "Failed to parse binary expression"
        })
    })
}

pub fn parse_num_expr<'a, L, T>(s: &str) -> Result<AST, ParseError<L, T, &'a str>> {
    ARENA.lock()
        .map_err(|_| ParseError::User {
            error: "Failed to acquire AST mutex to parse number expression"
        })
    .and_then(|mut guard| {
        let arena = guard.deref_mut();
        i32::from_str_radix(s, 10)
            .map_err(|_| ParseError::User {
                error: "Failed to parse numeric literal"
            })
        .and_then(|num| {
            create(arena, ENodeData::Number(num))
                .ok_or(ParseError::User {
                    error: "Failed to create new AST node for number"
                })
        })

    })
}

pub fn parse<'a>(program: &'a str) -> Result<AST, ParseError<usize, Token<'a>, &'static str>> {
    let ast = ExprParser::new().parse(program);

    ARENA.lock().and_then(|guard|{
        let arena = guard.deref();
        print_tree(arena, &ast.clone().unwrap(), 2);
        Ok(())
    });

    return ast;
}
