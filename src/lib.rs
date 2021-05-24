pub mod repl {
    pub fn eval(program: String) -> i32 {
        0
    }
}

#[cfg(test)]
mod test {
    use super::repl;

    #[test]
    fn test_add() {
        let program = "2 + 2";
        let result = repl::eval(program.to_string());
        assert_eq!(result, 4);
    }
}

