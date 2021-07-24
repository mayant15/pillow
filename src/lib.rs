use std::println;

mod codegen;
mod optimizer;
mod parser;

pub fn compile(program: String) -> Result<(), String> {
    let tokens = parser::tokenize(program)?;
    let ast = parser::parse(tokens);
    let new_ast = optimizer::optimize(ast);
    let ir = codegen::generate(new_ast);
    println!("{:?}", ir);
    Ok(())
}
