use std::slice::IterMut;
use std::iter::Peekable;

use super::types::{Token,Expr,Stmt,Value,Operator,Program};

/**
 * Syntax
 *
 * Expr -> Ident | Lit | $ Call | ( Expr ) | Expr OP Expr
 * Stmt -> Expr ; | while Expr { [Stmt] } | Ident = Expr ;
 *         | if Expr { Stmt } <else { Stmt }>
 *         | def Ident ( [Ident] ) { [Stmt] }
 *         | return Expr ;
 * Call -> Ident ( [Expr] )
 * Lit  -> NumLit | StrLit | BoolLit
*/

pub fn parse(mut tokens: Vec<Token>) -> Result<Program,&'static str> {
    let mut statements = Vec::new();
    let token_iter = &mut tokens.iter_mut().peekable();
    while token_iter.peek().is_some() {
        statements.push(parse_stmt(token_iter)?);
    }
    Ok(statements)
}

fn parse_expr(tokens: &mut Peekable<IterMut<'_,Token>>) -> Result<Expr,&'static str> {
    let first = tokens.next().ok_or("Unexpected end of input!")?;
    let first_expr = match first {
        Token::LParen => {
            let e = parse_expr(tokens)?;
            expect_token(tokens, Token::RParen, Some("Mismatched parenthesis!"))?;
            e
        },
        Token::Dollar => parse_call(tokens)?,
        Token::Ident(name) => Expr::Ident(name.clone()),
        Token::Num(n) => Expr::Lit(Value::Num(n.parse::<i64>().map_err(|_| "Bad number!")?)),
        Token::Str(s) => Expr::Lit(Value::Str(s.clone())),
        Token::Bool(b) => Expr::Lit(Value::Bool(*b)),
        _ => return Err("Unexpected token when parsing expression!")
    };

    if let Some(token) = tokens.peek() && Operator::from_token(token).is_some() {
        let op = Operator::from_token(token).unwrap();
        tokens.next(); // Consume the operator token
        Ok(Expr::BinOp { left: Box::new(first_expr), op: op,
                         right: Box::new(parse_expr(tokens)?) })
    } else {
        Ok(first_expr)
    }
}


fn expect_token(tokens: &mut Peekable<IterMut<'_,Token>>, expected: Token,
                msg: Option<&'static str>) -> Result<(),&'static str> {
    let token = tokens.next();
    if token.is_none() { return Err(msg.unwrap_or("Expected token, found end of source!")); }
    if *token.unwrap() != expected { return Err(msg.unwrap_or("Expected one token, found something else!")); }
    Ok(())
}

fn parse_stmt(tokens: &mut Peekable<IterMut<'_,Token>>) -> Result<Stmt,&'static str> {
    match tokens.peek() {
        Some(Token::While) => {
            tokens.next(); // Consume the "while"
            let cond = parse_expr(tokens)?;
            expect_token(tokens, Token::LBrace, Some("Left brace is needed to start while block!"))?;
            let mut body = Vec::new();
            while !matches!(tokens.peek(), Some(Token::RBrace)) {
                body.push(parse_stmt(tokens)?);
            }
            expect_token(tokens, Token::RBrace, Some("Right brace is needed to close while block!"))?;
            return Ok(Stmt::While { cond: cond, body: body });
        }
        Some(Token::Def) => {
            tokens.next(); // Consume the "def"
            let fname_token = tokens.next().ok_or("Unexpected end of input!")?;
            let fname = if let Token::Ident(f) = fname_token {
                f
            } else {
                return Err("Function name is required after 'def'!");
            };
            expect_token(tokens, Token::LParen, Some("Parameter list expected!"))?;
            let mut params = Vec::new();
            loop {
                match tokens.next() {
                    Some(Token::RParen) => { break; }
                    Some(Token::Ident(arg)) => {
                        params.push(arg.to_string());
                        tokens.next_if_eq(&&Token::Comma); // Consume trailing comma if it exists
                    }
                    _ => { return Err("Unexpected token while parsing argument list!"); }
                }
            }
            expect_token(tokens, Token::LBrace, Some("Left brace is needed to start function body!"))?;
            let mut body = Vec::new();
            while !matches!(tokens.peek(), Some(Token::RBrace)) {
                body.push(parse_stmt(tokens)?);
            }
            expect_token(tokens, Token::RBrace, Some("Right brace is needed to end function body!"))?;
            return Ok(Stmt::Function { name: fname.to_string(), params, body: body });
        }
        Some(Token::If) => {
            tokens.next();
            let cond = parse_expr(tokens)?;
            expect_token(tokens, Token::LBrace, Some("Left brace is needed to start if block!"))?;
            let mut body = Vec::new();
            while !matches!(tokens.peek(), Some(Token::RBrace)) {
                body.push(parse_stmt(tokens)?);
            }
            expect_token(tokens, Token::RBrace, Some("Right brace is needed to close if block!"))?;
            if tokens.next_if_eq(&&Token::Else).is_some() {
                expect_token(tokens, Token::LBrace, Some("Left brace is needed to start else block!"))?;
                let mut else_body = Vec::new();
                while !matches!(tokens.peek(), Some(Token::RBrace)) {
                    else_body.push(parse_stmt(tokens)?);
                }
                expect_token(tokens, Token::RBrace, Some("Right brace is needed to close else block!"))?;
                return Ok(Stmt::IfElse { cond: cond, then: body, else_: else_body });
            } else {
                return Ok(Stmt::If { cond: cond, body: body });
            }
        }
        Some(Token::Ident(_)) => {
            // It starts with an identifier, it could be either a 'single' (one-expression statement)
            // or an assignment statement. We'll parse as an expression to find out. This should always
            // succeed because an identifier by itself is a valid expression too.
            let expr = parse_expr(tokens)?;
            let stmt = if let Expr::Ident(name) = expr {
                if tokens.next_if_eq(&&Token::Assign).is_some() {
                    let right = parse_expr(tokens)?; // Expression on the right of the assignment
                    Ok(Stmt::Assign { left: name, right: right })
                } else {
                    Ok(Stmt::Single(Expr::Ident(name))) // Not an assignment: a single identifier expr
                }
            } else {
                Ok(Stmt::Single(expr))
            };
            expect_token(tokens, Token::Scln, Some("Semicolon expected at end of statement!"))?;
            stmt
        }
        Some(Token::Return) => {
            tokens.next(); // Consume 'return' token
            let exp = parse_expr(tokens)?;
            expect_token(tokens, Token::Scln, Some("Semicolon expected at end of return statement!"))?;
            Ok(Stmt::Return(exp))
        }
        _ => {
            let expr = parse_expr(tokens)?;
            expect_token(tokens, Token::Scln, Some("Semicolon expected at end of statement!"))?;
            Ok(Stmt::Single(expr))
        }
    }
}

fn parse_call(tokens: &mut Peekable<IterMut<'_,Token>>) -> Result<Expr,&'static str> {
    let function_name = match tokens.next() {
        Some(Token::Ident(name)) => name.clone(),
        _ => return Err("Incorrect syntax for function call!")
    };
    expect_token(tokens,Token::LParen,Some("Function call must begin with an LParen!"))?;
    if tokens.next_if_eq(&&Token::RParen).is_some() { // No args. Consume the RParen & return.
        return Ok(Expr::Call { func: function_name, args: Vec::new() });
    }
    let mut args = Vec::new();
    loop {
        let arg = parse_expr(tokens)?;
        args.push(arg);
        match tokens.peek() {
            Some(Token::RParen) => { tokens.next(); break; } // Consume right parenthesis
            Some(Token::Comma) => { tokens.next(); } // Consume comma, continue
            _ => return Err("Unexpected tokens in argument list!")
        };
    }
    Ok(Expr::Call { func: function_name, args: args })
}
