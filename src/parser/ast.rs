use crate::parser::lexer::Token;
use crate::tree::{Node, Tree};
use std::slice::Iter;
use std::vec::Vec;

use super::lexer::Operator;

/// Node for an expression in the AST

type AST = Tree<ASTNode>;

#[derive(Debug)]
enum ASTNode {
    Number(i32),
    Variable(String),
    BinaryExpr(Operator),
    Root,
}

fn parse_number_expr(it: &mut Iter<Token>, num: i32) -> Result<AST, String> {
    let ast = AST::new();
    ast.create_root(ASTNode::Number(num));
    Ok(ast)
}

fn parse_paren_expr(it: &mut Iter<Token>) -> Result<AST, String> {
    parse_expr(it).and_then(|ast| match it.next() {
        Some(&Token::Identifier(next)) if next == ")" => Ok(ast),
        _ => Err("Failed to find closing parenthesis ')'".to_string()),
    })
}

fn parse_identifier_expr(it: &mut Iter<Token>) -> Result<AST, String> {
    Err("Variable names are not yet supported. Identifiers can only be parenthesis".to_string())
}

fn parse_primary_expr(it: &mut Iter<Token>) -> Result<AST, String> {
    if let Some(token) = it.next() {
        match token {
            &Token::Number(num) => parse_number_expr(it, num),
            &Token::Identifier(id) if id == "(" => parse_paren_expr(it),
            &Token::Identifier(_) => parse_identifier_expr(it),
            _ => Err("Not a primary expression".to_string()),
        }
    } else {
        Err("Failed to parse primary expression: No tokens left".to_string())
    }
}

fn parse_expr(it: &mut Iter<Token>) -> Result<AST, String> {
    parse_primary_expr(it).and_then(|lhs| {
        if let Some(&Token::Operator(op)) = it.next() {
            let binop = AST::new();
            let root = binop.create_root(ASTNode::BinaryExpr(op));

            parse_primary_expr(it).and_then(|rhs| {
                binop.merge(root, lhs);
                binop.merge(root, rhs);
                return Ok(binop);
            })
        } else {
            Ok(lhs)
        }
    })
}

pub fn parse(arg: Vec<Token>) -> Result<AST, String> {
    let mut it = arg.iter();
    return parse_expr(&mut it);
}
