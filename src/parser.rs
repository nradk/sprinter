use std::slice::IterMut;
use std::iter::Peekable;

use super::types::{Token,Expr,Factor};

/**
 * Syntax:
 *
 * Expr -> Factor | Factor + Expr
 * Factor -> Num | Num * Factor
 *
*/

pub fn parse(mut tokens: Vec<Token>) -> Result<Expr,&'static str> {
    let token_iter = &mut tokens.iter_mut().peekable();
    Ok(parse_expr(token_iter)?)
}

fn parse_factor(tokens: &mut Peekable<IterMut<'_,Token>>) -> Result<Factor,&'static str> {
    let first = tokens.next().ok_or("Unexpected end of input!")?;
    let first_num = match first {
        Token::Num(n) => n.parse::<f64>().map_err(|_| "Bad number!")?,
        _ => return Err("Unexpected token when parsing expression!")
    };

    match tokens.peek() {
        Some(Token::Mult) => {
            tokens.next();
            Ok(Factor::Mult(first_num, Box::new(parse_factor(tokens)?)))
        },
        _ => Ok(Factor::Num(first_num))
    }
}

fn parse_expr(tokens: &mut Peekable<IterMut<'_,Token>>) -> Result<Expr,&'static str> {
    let factor = parse_factor(tokens)?;
    match tokens.peek() {
        Some(Token::Plus) => {
            tokens.next();
            Ok(Expr::Add(factor, Box::new(parse_expr(tokens)?)))
        },
        _ => Ok(Expr::Factor(factor))
    }
}
