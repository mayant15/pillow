use std::println;

mod codegen;
mod optimizer;
mod parser;

pub fn compile(program: &str) -> Result<(), &str> {
    let ast = parser::parse(program);
    let new_ast = optimizer::optimize(ast);
    let ir = codegen::generate(new_ast);
    println!("{:?}", ir);
    Ok(())
}
