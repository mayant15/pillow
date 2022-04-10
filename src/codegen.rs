use crate::parser::{Expr, Op};

pub fn generate(arg: Expr) -> String {
    match arg {
        Expr::Num(n) => n.to_string(),
        Expr::Neg(expr) => {
            let mut s = String::from("-1 * ");
            s.push_str(generate(*expr).as_str());
            s
        }
        Expr::BinOp { op, lhs, rhs } => {
            let mut lhs_str = generate(*lhs);
            let rhs_str = generate(*rhs);
            let op_str = match op {
                Op::Add => "+",
                Op::Mul => "*",
            };
            lhs_str.push_str(op_str);
            lhs_str.push_str(rhs_str.as_str());
            lhs_str
        }
    }
}
