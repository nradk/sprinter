use std::str::Chars;
use std::iter::Peekable;

use crate::types::Token;

fn take_while<P>(iter: &mut Peekable<Chars<'_>>, pred: P) -> String where P: Fn(&char) -> bool {
    let mut s = String::new();
    while iter.peek().is_some() && pred(iter.peek().unwrap()) {
        s.push(iter.next().unwrap());
    }
    return s;
}

pub fn tokenize(input: &str) -> Result<Vec<Token>,&'static str> {
    let mut tokens = vec![];
    let mut in_chars : Peekable<Chars<'_>> = input.chars().peekable();
    while in_chars.peek().is_some() {
        let c = in_chars.peek().unwrap();
        if c.is_ascii_whitespace() {        // Ignore whitespace
            in_chars.next();
        } else if *c == '+' {
            tokens.push(Token::Plus);
            in_chars.next();
        } else if *c == '*' {
            tokens.push(Token::Mult);
            in_chars.next();
        } else if c.is_ascii_digit() {      // If token starts with a digit, it's a number
            tokens.push(Token::Num(take_while(&mut in_chars, char::is_ascii_digit)));
        } else {
            return Err("Unrecognized character in source!");
        }
    }
    return Ok(tokens);
}
