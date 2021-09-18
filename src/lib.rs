use std::println;

mod codegen;
mod optimizer;
mod parser;

mod tree;

// NOTE: compile takes ownership of the program string.
// The Lexer looks at views on this string only
pub fn compile(program: String) -> Result<(), &'static str> {
    let ast = parser::parse(program.as_str());
    let new_ast = optimizer::optimize(ast);
    let ir = codegen::generate(new_ast);
    println!("{:?}", ir);
    Ok(())
}
