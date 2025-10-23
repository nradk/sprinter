use super::types::Expr;

pub fn evaluate(expr: Expr) -> Result<f64,&'static str> {
    Ok(match expr {
        Expr::Num(n) => n,
        Expr::Add(left, right) => evaluate(*left)? + evaluate(*right)?
    })
}
