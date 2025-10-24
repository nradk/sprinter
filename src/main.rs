use std::io::{self, Write};

mod tokenizer;
mod parser;
mod evaluator;
mod types;

use tokenizer::tokenize;
use parser::parse;
use evaluator::evaluate_expr;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        run_repl();
    } else if args.len() == 2 {
        execute_file(args[1].as_str());
    } else {
        println!("Error: wrong number of arguments!");
    }
}

/**
 * Read and execute the program from a file
 */
fn execute_file(filename: &str) {
    let program = std::fs::read_to_string(filename).expect("Can't read file!");
    let result = Ok(program.as_str())
        .and_then(tokenize)
        .and_then(parse)
        .and_then(evaluate_expr);
    match result {
        Ok(n) => println!("{}",n),
        Err(e) => println!("Error: {}", e)
    }
}

/**
 * Run the Read-Eval-Print Loop (REPL)
 */
fn run_repl() {
    loop {
        print!(">>> "); io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.is_empty() {
            println!("\nBye.");
            break;
        }
        let result = Ok(input.as_str())
            .and_then(tokenize)
            .and_then(parse)
            .and_then(evaluate_expr);
        match result {
            Ok(n) => println!("{}",n),
            Err(e) => { println!("Error: {}", e); break; }
        }
    }
}
