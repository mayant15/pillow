pub mod parser;

#[derive(Default)]
pub struct REPL {
    lexer: parser::Lexer,
}

impl REPL {
    pub fn eval(&mut self, program: String) -> i32 {
        0
    }
}

#[cfg(test)]
mod test {
    use super::REPL;

    #[test]
    fn test_add() {
        let program = "2 + 2";
        let mut repl: REPL = Default::default();
        let result = repl.eval(program.to_string());
        assert_eq!(result, 4);
    }
}

