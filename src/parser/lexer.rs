#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Operator {
    Add,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Identifier(String),
    Number(i32),
    Operator(Operator),
}

// NOTE: This is pretty impure
pub struct Lexer<'a> {
    program: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(program: &str) -> Lexer {
        Lexer {
            program: program.trim(),
        }
    }

    pub fn get_next_token(&mut self) -> Option<Token> {
        // Consume whitespace, if any
        match tokenizer::whitespace(self.program) {
            Err(_) => (),
            Ok((i, _o)) => {
                self.program = i;
            }
        };

        if let Some(char) = self.program.chars().next() {
            if Lexer::is_identifier(char) {
                return match tokenizer::identifier(self.program) {
                    Err(_) => None,
                    Ok((i, o)) => {
                        self.program = i;
                        Some(Token::Identifier(o.to_string()))
                    }
                };
            } else if char.is_numeric() {
                return match tokenizer::decimal_literal(self.program) {
                    Err(_) => None,
                    Ok((i, o)) => {
                        self.program = i;
                        Some(Token::Number(o.to_string().parse::<i32>().unwrap()))
                    }
                };
            } else if Lexer::is_operator(char) {
                return match tokenizer::operator(self.program) {
                    Err(_) => None,
                    Ok((i, o)) => {
                        self.program = i;
                        Some(Token::Operator(Lexer::operator_from_str(o)))
                    }
                };
            }
        }

        return None;
    }

    fn is_operator(ch: char) -> bool {
        ch == '+' || ch == '-'
    }

    fn is_identifier(ch: char) -> bool {
        // NOTE: An identifier can start with anything that's not a number
        ch.is_alphabetic() || ch == '_'
    }

    fn operator_from_str(s: &str) -> Operator {
        match s {
            "+" => Operator::Add,
            _ => Operator::Add,
        }
    }
}

mod tokenizer {

    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alpha1, alphanumeric1, char, multispace0, one_of},
        combinator::recognize,
        multi::{many0, many1},
        sequence::{pair, terminated},
        IResult,
    };

    pub fn decimal_literal(input: &str) -> IResult<&str, &str> {
        recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
    }

    pub fn identifier(input: &str) -> IResult<&str, &str> {
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        ))(input)
    }

    pub fn operator(input: &str) -> IResult<&str, &str> {
        recognize(one_of("+-/*="))(input)
    }

    pub fn whitespace(input: &str) -> IResult<&str, &str> {
        multispace0(input)
    }
}

#[cfg(test)]
mod test {
    use super::tokenizer;
    use super::{Lexer, Operator, Token};

    #[test]
    fn test_parse_decimal_literal() {
        assert_eq!(tokenizer::decimal_literal("2 + 3"), Ok((" + 3", "2")));
        assert_eq!(tokenizer::decimal_literal("2+ 3"), Ok(("+ 3", "2")));
        assert_eq!(tokenizer::decimal_literal("2 +3"), Ok((" +3", "2")));
        assert_eq!(tokenizer::decimal_literal("2+3"), Ok(("+3", "2")));
        assert_eq!(tokenizer::decimal_literal("234 + 3"), Ok((" + 3", "234")));

        // TODO: This would have nested errors? I don't want to bother with that
        match tokenizer::decimal_literal("s+3") {
            Err(_) => assert_eq!(1, 1), // This branch implies we've passed, this should indeed be an error
            Ok(_) => assert_eq!(1, 0),  // This branch implies we've failed, this should not be Ok
        }
    }

    #[test]
    fn test_parse_identifier() {
        assert_eq!(tokenizer::identifier("x + 2"), Ok((" + 2", "x")));
        assert_eq!(tokenizer::identifier("x   +2"), Ok(("   +2", "x")));
        assert_eq!(tokenizer::identifier("_d+2"), Ok(("+2", "_d")));

        match tokenizer::identifier("$x - 2") {
            Err(_) => assert_eq!(1, 1), // This branch implies we've passed, this should indeed be an error
            Ok(_) => assert_eq!(1, 0),  // This branch implies we've failed, this should not be Ok
        }
    }

    #[test]
    fn test_numeric_expression() {
        let mut lexer = Lexer::new("2 + 4");
        assert_eq!(Some(Token::Number(2)), lexer.get_next_token());
        assert_eq!(Some(Token::Operator(Operator::Add)), lexer.get_next_token());
        assert_eq!(Some(Token::Number(4)), lexer.get_next_token());
    }
}
