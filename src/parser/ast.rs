use crate::parser::lexer::Token;
use crate::tree::{attach_to, create, print_tree, TreeArena, TreeNode};
use std::slice::Iter;
use std::vec::Vec;

use super::lexer::Operator;

/// Node for an expression in the AST
type AST = TreeNode;

#[derive(Debug)]
enum ASTNodeData {
    Number(i32),
    BinaryExpr(Operator),
    // Variable(String),
    // Root,
}

fn parse_number_expr(
    _it: &mut Iter<Token>,
    arena: &mut TreeArena<ASTNodeData>,
    num: i32,
) -> Result<AST, String> {
    match create(arena, ASTNodeData::Number(num)) {
        Some(node) => Ok(node),
        _ => Err(format!("Failed to create number AST node: {}", num).to_string()),
    }
}

fn parse_paren_expr(
    it: &mut Iter<Token>,
    arena: &mut TreeArena<ASTNodeData>,
) -> Result<AST, String> {
    parse_expr(it, arena).and_then(|ast| {
        let next = it.next();
        if let Some(token) = next {
            if let Token::Identifier(id) = token {
                if id == ")" {
                    Ok(ast)
                } else {
                    Err(format!("Expected ')' found '{}'", id))
                }
            } else {
                Err(format!("Token '{:?}' is not an identifier", token))
            }
        } else {
            Err("Failed to find closing parenthesis ')'".to_string())
        }
    })
}

fn parse_identifier_expr(
    _it: &mut Iter<Token>,
    _arena: &mut TreeArena<ASTNodeData>,
) -> Result<AST, String> {
    Err("Variable names are not yet supported. Identifiers can only be parenthesis".to_string())
}

fn parse_primary_expr(
    it: &mut Iter<Token>,
    arena: &mut TreeArena<ASTNodeData>,
) -> Result<AST, String> {
    if let Some(token) = it.next() {
        match token {
            Token::Number(num) => parse_number_expr(it, arena, num.clone()),
            Token::Identifier(id) if id == "(" => parse_paren_expr(it, arena),
            Token::Identifier(_) => parse_identifier_expr(it, arena),
            _ => Err(format!("Token '{:?}' is not a primary expression", token)),
        }
    } else {
        Err("Failed to parse primary expression: No tokens left".to_string())
    }
}

fn parse_expr(it: &mut Iter<Token>, arena: &mut TreeArena<ASTNodeData>) -> Result<AST, String> {
    parse_primary_expr(it, arena).and_then(|lhs| {
        if let Some(&Token::Operator(op)) = it.next() {
            match create(arena, ASTNodeData::BinaryExpr(op)) {
                Some(parent) => parse_primary_expr(it, arena).and_then(|rhs| {
                    attach_to(arena, parent, lhs);
                    attach_to(arena, parent, rhs);
                    Ok(parent)
                }),
                None => Err("Failed to create new tree node for binary operation".to_string()),
            }
        } else {
            Ok(lhs)
        }
    })
}

pub fn parse(arg: Vec<Token>) -> Result<AST, String> {
    let mut it = arg.iter();
    let mut arena = TreeArena::<ASTNodeData>::new();
    parse_expr(&mut it, &mut arena).and_then(|ast| {
        print_tree(&arena, &ast, 0);
        Ok(ast)
    })
}
