use std::slice::IterMut;
use std::iter::Peekable;

use super::types::{Token,Expr};

/**
 * Syntax:
 *
 * Expr -> Num | Num + Expr
*/

pub fn parse(mut tokens: Vec<Token>) -> Result<Expr,&'static str> {
    let token_iter = &mut tokens.iter_mut().peekable();
    Ok(parse_expr(token_iter)?)
}

fn parse_expr(tokens: &mut Peekable<IterMut<'_,Token>>) -> Result<Expr,&'static str> {
    let first = tokens.next().ok_or("Unexpected end of input!")?;
    let first_expr = match first {
        Token::Num(n) => Expr::Num(n.parse::<f64>().map_err(|_| "Bad number!")?),
        _ => return Err("Unexpected token when parsing expression!")
    };

    match tokens.peek() {
        Some(Token::Plus) => {
            tokens.next();
            Ok(Expr::Add(Box::new(first_expr), Box::new(parse_expr(tokens)?)))
        },
        _ => Ok(first_expr)
    }
}
