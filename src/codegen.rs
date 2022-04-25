use crate::parser::{Expr, Op};
use inkwell::OptimizationLevel;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use inkwell::values::{FloatValue, AnyValue, BasicValue};
use std::error::Error;
use std::ops::Add;

const MAIN_FUNCTION_NAME: &str = "main";
const MAIN_FUNCTION_ENTRY_BASIC_BLOCK_NAME: &str = "entry";

/// Type for the entry point of the program
/// NOTE: .pw files must have a main function, REPL creates an implicit main function
/// TODO: Have a generic return type for this?
type MainFunction = unsafe extern "C" fn() -> f64;

struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    fn generate_num(&self, value: f64) -> FloatValue {
        let f64_type = self.context.f64_type();
         f64_type.const_float(value)
    }

    fn generate_expr(&self, expr: Expr) -> Result<FloatValue, String> {
        match expr {
            Expr::Num(n) => Ok(self.generate_num(n)),
            // Expr::Neg(expr) => {
            //     let mut s = String::from("-1 * ");
            //     s.push_str(generate(*expr).as_str());
            //     s
            // }
            Expr::BinOp { op, lhs, rhs} => {
                let lhs_value = self.generate_expr(*lhs)?;
                let rhs_value = self.generate_expr(*rhs)?;
                let op_value = match op {
                    Op::Add => self.builder.build_float_add(lhs_value, rhs_value, "addtmp"),
                    Op::Mul => self.builder.build_float_mul(lhs_value, rhs_value, "multmp"),
                };
                Ok(op_value)
            }
            _ => Err(format!("Failed to generate code for unsupported expression {:?}", expr))
        }
    }

    fn build_with_implicit_main(&self, expr: Expr) -> Result<JitFunction<MainFunction>, String> {
            let f64_type = self.context.f64_type();
            let fn_type = f64_type.fn_type(&[], false);
            let function = self.module.add_function(MAIN_FUNCTION_NAME, fn_type, None);

            // TODO: Basic block creation and organization?
            let basic_block = self.context.append_basic_block(function, MAIN_FUNCTION_ENTRY_BASIC_BLOCK_NAME);
            self.builder.position_at_end(basic_block);

            let value = self.generate_expr(expr)?;

            self.builder.build_return(Some(&value));

            return unsafe { self.execution_engine.get_function::<MainFunction>(MAIN_FUNCTION_NAME) }.map_err(|e| {
                format!("Function lookup in execution engine failed: {:?}", e.to_string())
            });
    }
}

pub fn generate(arg: Expr) -> Result<String, Box<dyn Error>> {
    let context = Context::create();
    let module = context.create_module("sum");
    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None)?;
    let codegen = CodeGen {
        context: &context,
        module,
        builder: context.create_builder(),
        execution_engine,
    };

    // TODO: Only for REPL
    let main = codegen.build_with_implicit_main(arg).map_err(|e|
        e.add( ". Unable to JIT compile expression")
    )?;

    // DEBUG: Print generated main function IR
    println!("{}", codegen.module.get_function(MAIN_FUNCTION_NAME).unwrap().print_to_string());

    return unsafe {
        let result = format!("Output: {}", main.call());
        Ok(result)
    }
}
