use std::println;

mod codegen;
mod optimizer;
mod parser;

mod tree;

// NOTE: compile takes ownership of the program string.
// The Lexer looks at views on this string only
pub fn compile(program: &str) -> Result<(), &'static str> {
    let mut lexer = parser::Lexer::new(&program);

    let mut tokens: Vec<parser::Token> = Vec::new();
    while let Some(token) = lexer.get_next_token() {
        tokens.push(token)
    }

    let ast = parser::parse(tokens);
    let new_ast = optimizer::optimize(ast);
    let ir = codegen::generate(new_ast);
    println!("{:?}", ir);
    Ok(())
}
