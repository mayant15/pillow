use std::{collections::VecDeque, str::Chars};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Operator {
    Addition,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Identifier(String),
    Number(i32),
    Operator(Operator),
    EOF,
}

fn get_word<F>(chars: &mut Chars, filter: F) -> Option<String>
where
    F: Fn(char) -> bool,
{
    let mut current_token = String::from("");
    while let Some(next_char) = chars.next() {
        if next_char.is_whitespace() {
            break;
        } else if filter(next_char) {
            current_token.push(next_char);
        } else {
            return None;
        }
    }
    return Some(current_token);
}

/*
TODO: Return error when error happens
TODO: Load tokens one by one, read char by char, don't split_whitespace the complete
thing
*/
pub fn tokenize(program: String) -> Result<VecDeque<Token>, &'static str> {
    let mut tokens: VecDeque<Token> = VecDeque::new();

    let mut chars = program.chars();
    while let Some(char) = chars.next() {
        if char.is_whitespace() {
            continue;
        } else if char == '+' {
            tokens.push_back(Token::Operator(Operator::Addition));
        } else if char.is_numeric() {
            // This is a number
            match get_word(&mut chars, char::is_numeric) {
                Some(mut word) => {
                    word.insert(0, char);
                    tokens.push_back(Token::Number(word.parse::<i32>().unwrap()));
                }
                None => return Err("Failed to parse number"),
            };
        } else if char.is_alphabetic() {
            // This is an identifier
            match get_word(&mut chars, char::is_alphanumeric) {
                Some(mut word) => {
                    word.insert(0, char);
                    tokens.push_back(Token::Identifier(word));
                }
                None => return Err("Failed to parse identifier"),
            };
        }
    }

    tokens.push_back(Token::EOF);
    Ok(tokens)
}

#[cfg(test)]
mod test {
    use super::{tokenize, Operator, Token};

    #[test]
    fn test_numeric_expression() {
        match tokenize("2 + 4".to_string()) {
            Err(error) => eprintln!("ERROR: Failed to parse 2 + 4\nDETAILS: {}", error),
            Ok(tokens) => {
                assert_eq!(Some(&Token::Number(2)), tokens.get(0));
                assert_eq!(Some(&Token::Operator(Operator::Addition)), tokens.get(1));
                assert_eq!(Some(&Token::Number(4)), tokens.get(2));
                assert_eq!(Some(&Token::EOF), tokens.get(3));
            }
        }
    }
}
