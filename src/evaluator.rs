use std::io;

use super::types::{Expr, Stmt, Program, Value, Operator, Environment};

pub fn evaluate(program: Program, env: &mut Environment) -> Result<(),&'static str> {
    for stmt in program {
        evaluate_stmt(&stmt, env, true)?;
    }
    Ok(())
}

fn evaluate_stmt(stmt: &Stmt, env: &mut Environment, top_level: bool) -> Result<Option<Value>,&'static str> {
    match stmt {
        Stmt::Single(e) => { evaluate_expr(e,env)?; }
        Stmt::While {cond,body} => loop {
            let cond_val = evaluate_expr(cond,env)?;
            if let Value::Bool(b) = cond_val {
                if !b { break; }
            } else {
                return Err("The condition for a while loop must be a boolean!");
            }
            for body_stmt in body {
                let maybe_return = evaluate_stmt(body_stmt,env,false)?;
                if maybe_return.is_some() { return Ok(maybe_return); }
            }
        }
        Stmt::Function { name, params, body } => {
            if !top_level {
                return Err("Functions can only be defined at the top level!");
            }
            env.insert(name.clone(), Value::Function{params: params.clone(), body: body.clone()});
        }
        Stmt::If {cond,body} => {
            let cond_val = evaluate_expr(cond,env)?;
            if let Value::Bool(b) = cond_val {
                if b {
                    for body_stmt in body {
                        let maybe_return = evaluate_stmt(body_stmt,env,false)?;
                        if maybe_return.is_some() { return Ok(maybe_return); }
                    }
                }
            } else {
                return Err("The condition for an if statement must be a boolean!");
            }
        }
        Stmt::IfElse {cond,then,else_} => {
            let cond_val = evaluate_expr(cond,env)?;
            if let Value::Bool(b) = cond_val {
                for body_stmt in if b { then } else { else_ } {
                    let maybe_return = evaluate_stmt(body_stmt,env,false)?;
                    if maybe_return.is_some() { return Ok(maybe_return); }
                }
            } else {
                return Err("The condition for an if-else statement must be a boolean!");
            }
        }
        Stmt::Assign {left,right} => { env.insert(left.clone(), evaluate_expr(right,env)?); }
        Stmt::Return(e) => { return Ok(Some(evaluate_expr(e,env)?)); }
    };
    Ok(None)
}


fn evaluate_expr(term: &Expr, env: &Environment) -> Result<Value,&'static str> {
    match term {
        Expr::Ident(name) => env.get(name.as_str()).map(Value::clone).ok_or("Use of undefined variable!"),
        Expr::Lit(val) => Ok(val.clone()),
        Expr::BinOp {left,op,right} => evaluate_binop(left,op,right,env),
        Expr::Call {func,args} => evaluate_call(&func,&args,env)
    }
}

fn evaluate_binop(left: &Expr, op: &Operator, right: &Expr,
                  env: &Environment) -> Result<Value,&'static str> {
    let (left_v,right_v) = (evaluate_expr(left,env)?, evaluate_expr(right,env)?);
    Ok(match op {
        Operator::Add => Value::Num(left_v.expect_num()? + right_v.expect_num()?),
        Operator::Subtract => Value::Num(left_v.expect_num()? - right_v.expect_num()?),
        Operator::Divide => Value::Num(left_v.expect_num()? / right_v.expect_num()?),
        Operator::Multiply => Value::Num(left_v.expect_num()? * right_v.expect_num()?),
        Operator::Exponentiate => Value::Num(left_v.expect_num()?.pow(right_v.expect_num()?.try_into().unwrap())),
        Operator::Modulo => Value::Num(left_v.expect_num()? % right_v.expect_num()?),
        Operator::Concat => Value::Str(format!("{}{}",left_v.expect_str()?, right_v.expect_str()?)),
        Operator::LessThan => Value::Bool(left_v.expect_num()? < right_v.expect_num()?),
        Operator::LessThanEq => Value::Bool(left_v.expect_num()? <= right_v.expect_num()?),
        Operator::GreaterThan => Value::Bool(left_v.expect_num()? > right_v.expect_num()?),
        Operator::GreaterThanEq => Value::Bool(left_v.expect_num()? >= right_v.expect_num()?),
        Operator::Equals => Value::Bool(left_v.expect_num()? == right_v.expect_num()?),
        Operator::And => Value::Bool(left_v.expect_bool()? && right_v.expect_bool()?),
        Operator::Or => Value::Bool(left_v.expect_bool()? || right_v.expect_bool()?)
    })
}

fn evaluate_call(func: &str, args: &Vec<Expr>, env: &Environment) -> Result<Value,&'static str> {
    match func {
        "print" => {
            if args.len() != 1 {
                return Err("\"print\" takes exactly one argument!");
            }
            println!("{}", evaluate_expr(&args[0],env)?.as_string());
        },
        "input" => {
            if args.len() != 0 {
                return Err("\"input\" takes no arguments!");
            }
            let mut input = String::new();
            io::stdin().read_line(&mut input).map_err(|_| "Failed to read line")?;
            return Ok(Value::Str(input[..(input.len() - 1)].to_string())); // Chop trailing newline
        },
        "not" => {
            if args.len() != 1 {
                return Err("\"not\" takes exactly one argument!");
            }
            if let Value::Bool(b) = evaluate_expr(&args[0],env)? {
                return Ok(Value::Bool(!b));
            } else {
                return Err("Argument to \"not\" needs to be a boolean!");
            }
        }
        "str" => {
            if args.len() != 1 {
                return Err("\"str\" takes exactly one argument!");
            }
            return Ok(Value::Str(evaluate_expr(&args[0],env)?.as_string()));
        }
        "num" => {
            if args.len() != 1 {
                return Err("\"num\" takes exactly one argument!");
            }
            let maybe_num = evaluate_expr(&args[0],env)?;
            if let Value::Str(n_str) = maybe_num && n_str.parse::<i64>().is_ok() {
                return Ok(Value::Num(n_str.parse::<i64>().unwrap()));
            }
        }
        fname => {
            if let Some(Value::Function { params, body }) = env.get(fname) {
                if args.len() != params.len() {
                    return Err("Wrong number of arguments to function!");
                }
                let mut fun_env = Environment::new();
                // Insert all defined functions to the function's environment
                env.iter().filter(|(_,v)| v.is_function()).for_each(|(name, value)| {
                    fun_env.insert(name.clone(), value.clone());
                });
                for i in 0..args.len() { // Insert argument bindings to functions' environment
                    fun_env.insert(params[i].clone(), evaluate_expr(&args[i],env)?);
                }
                for stmt in body {  // Evaluate function body
                    let maybe_return = evaluate_stmt(stmt, &mut fun_env, false)?;
                    if maybe_return.is_some() {
                        return Ok(maybe_return.unwrap());
                    }
                }
            } else {
                return Err("Cannot call a function that has not been defined!");
            }
        }
    };
    Ok(Value::Nothing)
}
