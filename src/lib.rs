mod codegen;
mod optimizer;
mod parser;

pub fn compile(program: &str) -> Result<(), String> {
    parser::parse(program)
        .map_err(|e| format!("{:?}", e))
        .map(|(_, parsed)| parsed)
        .map(optimizer::optimize)
        .map(codegen::generate)
        .map(|ir| {
            println!("{:?}", ir);
        })
}
