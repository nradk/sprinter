use super::types::{Expr,Factor};

pub fn evaluate_expr(expr: Expr) -> Result<f64,&'static str> {
    Ok(match expr {
        Expr::Factor(f) => evaluate_factor(f)?,
        Expr::Add(left, right) => evaluate_factor(left)? + evaluate_expr(*right)?,
    })
}

pub fn evaluate_factor(factor: Factor) -> Result<f64,&'static str> {
    Ok(match factor {
        Factor::Num(n) => n,
        Factor::Mult(left, right) => left * evaluate_factor(*right)?
    })
}
