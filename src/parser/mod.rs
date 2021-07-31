pub use self::ast::parse;
pub use self::lexer::{Lexer, Token};

mod ast;
mod lexer;

/*
pub struct Parser {
    lexer: lexer::Lexer,
    ast: Box<dyn ast::ExprNode>,
}

impl Parser {
    pub fn new(program: &str) -> Result<Parser, String> {
        let mut lexer = lexer::Lexer::default();

        match lexer.load_program(program) {
            Err(msg) => return Err(msg),
            _ => (),
        };

        let mut parser = Parser {
            lexer: lexer,
            ast: Box::<ast::RootNode>::new(Default::default()),
        };
        Err(String::from("implement"))

        // parser.construct_ast();
    }

    fn construct_ast(&mut self) {
        // while self.lexer.peek() != Some(&Token::EOF) {
        // let token = self.lexer.get_next_token();
        // if self.lexer.peek() == Some(&Token::Op) {}
        // }
    }
    */

/*
fn create_node_from_token(token: &lexer::Token) -> Box<dyn ast::ExprNode> {
    match token {
        Token::&Number(num) => {
            ast::NumberExprNode::from(num)
        }
    }

}*/
// }
