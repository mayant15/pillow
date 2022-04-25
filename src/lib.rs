use nom::{Err, error::Error};

mod codegen;
mod optimizer;
mod parser;

pub fn compile(program: &str) -> Result<(), String> {
    parser::parse(program)
        .map_err(|e: Err<Error<&str>>| format!("{:?}", e))
        .map(|(_, parsed)| parsed)
        .map(optimizer::optimize)
        .and_then(|op| codegen::generate(op).map_err(|e|
            (*e).to_string()
        ))
        .map(|ir| {
            println!("{:?}", ir);
        })
}
