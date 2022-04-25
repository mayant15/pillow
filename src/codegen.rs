use crate::parser::{Expr, Op};
use inkwell::OptimizationLevel;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use std::error::Error;
use inkwell::values::AnyValue;

/// Convenience type alias for the `sum` function.
///
/// Calling this is innately `unsafe` because there's no guarantee it doesn't
/// do `unsafe` operations internally.
type SumFunc = unsafe extern "C" fn(u64, u64, u64) -> u64;

struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    fn jit_compile_sum(&self) -> Option<JitFunction<SumFunc>> {
        let i64_type = self.context.i64_type();
        let fn_type = i64_type.fn_type(&[i64_type.into(), i64_type.into(), i64_type.into()], false);
        let function = self.module.add_function("sum", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(basic_block);

        let x = function.get_nth_param(0)?.into_int_value();
        let y = function.get_nth_param(1)?.into_int_value();
        let z = function.get_nth_param(2)?.into_int_value();

        let sum = self.builder.build_int_add(x, y, "sum");
        let sum = self.builder.build_int_add(sum, z, "sum");

        self.builder.build_return(Some(&sum));

        unsafe { self.execution_engine.get_function("sum").ok() }
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

    let sum = codegen.jit_compile_sum().ok_or("Unable to JIT compile `sum`")?;

    println!("{}", codegen.module.get_function("sum").unwrap().print_to_string());


    let x = 1u64;
    let y = 2u64;
    let z = 3u64;

    return unsafe {
        let result = format!("{} + {} + {} = {}", x, y, z, sum.call(x, y, z));
        Ok(result)
    }

    // match arg {
    //     Expr::Num(n) => n.to_string(),
    //     Expr::Neg(expr) => {
    //         let mut s = String::from("-1 * ");
    //         s.push_str(generate(*expr).as_str());
    //         s
    //     }
    //     Expr::BinOp { op, lhs, rhs } => {
    //         let mut lhs_str = generate(*lhs);
    //         let rhs_str = generate(*rhs);
    //         let op_str = match op {
    //             Op::Add => "+",
    //             Op::Mul => "*",
    //         };
    //         lhs_str.push_str(op_str);
    //         lhs_str.push_str(rhs_str.as_str());
    //         lhs_str
    //     }
    // }
}
