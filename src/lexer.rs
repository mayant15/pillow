use std::vec::Vec;
use std::slice::Iter;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Operator {
    Addition
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Identifier(String),
    Number(i32),
    Op(Operator),
    EOF
}

#[derive(Default)]
pub struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn load_program(&mut self, program: &str) -> Result<(), String> {
        // TODO: Return error when error happens
        // TODO: Load tokens
        for word in program.split_whitespace() {
            if word.chars().all(char::is_numeric) {
                self.tokens.push(Token::Number(word.parse::<i32>().unwrap()));
                continue;
            }

            if word == "+" {
                self.tokens.push(Token::Op(Operator::Addition));
                continue;
            }

            // Not an operator or a number, treat it as an identifier
            self.tokens.push(Token::Identifier(word.to_string()));
        }

        self.tokens.push(Token::EOF);
        Ok(())
    }

    pub fn tokens(&self) -> Iter<Token> {
        self.tokens.iter()
    }
}

#[cfg(test)]
mod test {
    use super::{ Lexer, Token, Operator };

    #[test]
    fn test_numeric_expression() {
        let mut lexer: Lexer = Default::default();
        lexer.load_program("2 + 4").expect("Failed to parse string");
        
        let mut iter = lexer.tokens();
        assert_eq!(Some(&Token::Number(2)), iter.next());
        assert_eq!(Some(&Token::Op(Operator::Addition)), iter.next());
        assert_eq!(Some(&Token::Number(4)), iter.next());
        assert_eq!(Some(&Token::EOF), iter.next());
    }
} 

