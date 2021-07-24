pub fn parse<T>(arg: T) -> T {
    arg
}

/*
use std::boxed::Box;
use std::vec::Vec;

/// Node for an expression in the AST
pub trait ExprNode {
    fn parse(&mut self, lexer: &mut Lexer) -> Result<(), String>;
}

#[derive(Default)]
pub struct RootNode {
    children: Vec<Box<dyn ExprNode>>,
}

impl ExprNode for RootNode {
    fn parse(&mut self, lexer: &mut Lexer) -> Result<(), String> {
        Ok(())
    }
}

pub struct NumberExprNode {
    value: i32,
}

impl ExprNode for NumberExprNode {
    fn parse(&mut self, lexer: &mut Lexer) -> Result<(), String> {
        let token = lexer.get_next_token();
        match token {
            Some(Token::Number(n)) => {
                self.value = n;
                Ok(())
            },
            _ => Err(format!("Cannot parse token {:?} into NumberExprNode", token))
        }
    }
}

struct VariableExprNode {
    name: String,
}

struct BinaryExprNode {
    op: char, // Use the lexer's tokens?
    lhs: Box<dyn ExprNode>,
    rhs: Box<dyn ExprNode>,
}

struct CallExprNode {
    callee: String,
    args: Vec<Box<dyn ExprNode>>,
}

struct PrototypeNode {
    name: String,
    args: Vec<String>,
}

struct FunctionNode {
    prototype: Box<PrototypeNode>,
    body: Box<dyn ExprNode>,
}

*/
