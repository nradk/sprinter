use std::str::Chars;
use std::iter::Peekable;

use crate::types::Token;

fn take_str(iter: &mut Peekable<Chars<'_>>) -> Result<String, &'static str> {
    let mut s = String::new();
    while iter.peek().is_some() && *iter.peek().unwrap() != '"' {
        let mut c = iter.next().unwrap();
        if c == '\\' {
            c = iter.next().ok_or("Source ends in a backslash!")?;
        }
        s.push(c);
    }
    iter.next().ok_or("Unclosed string!").map(|_| s)
}

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
        if c.is_ascii_whitespace() {                    // Ignore whitespace
            in_chars.next();
        } else if let Some(token) = Token::get_for_single_char(c) {
            tokens.push(token);
            in_chars.next();
        } else if c.is_ascii_digit() {      // If token starts with a digit, it's a number
            tokens.push(Token::Num(take_while(&mut in_chars, char::is_ascii_digit)));
        } else if c.is_ascii_alphabetic() || *c == '_' {
            // If it starts with an alphabet or underscore, it's an identifier
            let ident_name = take_while(&mut in_chars, |c| c.is_ascii_alphanumeric() || *c == '_');
            tokens.push(match Token::get_for_keyword(&ident_name.as_str()) {
                Some(keyword_token) => keyword_token,
                None => Token::Ident(ident_name)
            });
        } else if *c == '"' {               // If it starts with a double quote, it's a string
            in_chars.next();
            tokens.push(Token::Str(take_str(&mut in_chars)?));
        } else if *c == '>' {
            in_chars.next();
            if let Some('=') = in_chars.peek() {
                in_chars.next();
                tokens.push(Token::GreaterThanEq);
            } else {
                tokens.push(Token::GreaterThan);
            }
        } else if *c == '<' {
            in_chars.next();
            if let Some('=') = in_chars.peek() {
                in_chars.next();
                tokens.push(Token::LessThanEq);
            } else {
                tokens.push(Token::LessThan);
            }
        } else if *c == '=' {
            in_chars.next();
            if let Some('=') = in_chars.peek() {
                in_chars.next();
                tokens.push(Token::Equals);
            } else {
                tokens.push(Token::Assign);
            }
        } else {
            return Err("Unrecognized character in source!");
        }
    }
    return Ok(tokens);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_tokens_and_empty_input() {
        assert_eq!(Ok(vec![Token::Add, Token::LParen, Token::RParen]), tokenize("+()"));
        assert_eq!(Ok(vec![Token::Mul, Token::Div, Token::Sub]), tokenize("*/-"));
        assert_eq!(Ok(vec![Token::Exp, Token::Scln, Token::Comma]), tokenize("^;,"));
        assert_eq!(Ok(vec![]), tokenize(""));
    }

    #[test]
    fn test_numbers() {
        let token_1 = Token::Num(String::from("1"));
        let token_321 = Token::Num(String::from("321"));
        assert_eq!(Ok(vec![token_1.clone()]), tokenize("1"));
        assert_eq!(Ok(vec![token_321.clone()]), tokenize("321"));
        assert_eq!(Ok(vec![token_1, Token::Add, Token::LParen, token_321, Token::RParen]), tokenize("1+(321)"));
    }

    #[test]
    fn test_idents() {
        let token_a = Token::Ident(String::from("a"));
        let token_xyz = Token::Ident(String::from("xyz"));
        assert_eq!(Ok(vec![token_a.clone()]), tokenize("a"));
        assert_eq!(Ok(vec![token_xyz.clone()]), tokenize("xyz"));
        assert_eq!(Ok(vec![token_a, Token::Add, Token::LParen, token_xyz, Token::RParen]), tokenize("a+(xyz)"));
    }

    #[test]
    fn test_ident_numbers() {
        let token_a = Token::Ident(String::from("a"));
        let token_xyz = Token::Ident(String::from("xyz"));
        let token_321 = Token::Num(String::from("321"));
        let token_a321 = Token::Ident(String::from("a321"));
        assert_eq!(Ok(vec![token_a321.clone()]), tokenize("a321"));
        assert_eq!(Ok(vec![token_321.clone(), token_a.clone()]), tokenize("321a"));
        assert_eq!(Ok(vec![token_xyz, Token::LParen, token_321, Token::RParen, token_a]), tokenize("xyz(321)a"));
    }

    #[test]
    fn test_basic_strings() {
        assert_eq!(Ok(vec![Token::Str(String::from("abc"))]), tokenize(r#" "abc" "#));
        assert_eq!(Ok(vec![Token::Str(String::from(""))]), tokenize(r#" "" "#));
        assert_eq!(Ok(vec![Token::Str(String::from(" "))]), tokenize(r#" " " "#));
        assert_eq!(Ok(vec![Token::Str(String::from("1 23"))]), tokenize(r#" "1 23" "#));
        assert_eq!(Ok(vec![Token::Str(String::from("* -/"))]), tokenize(r#" "* -/" "#));
    }

    #[test]
    fn test_escaped_strings() {
        assert_eq!(Ok(vec![Token::Str(String::from("ab\"c"))]), tokenize(r#" "ab\"c" "#));
        assert_eq!(Ok(vec![Token::Str(String::from(r"\"))]), tokenize(r#" "\\" "#));
        assert_eq!(Ok(vec![Token::Str(String::from("\""))]), tokenize(r#" "\"" "#));
    }

    #[test]
    fn test_bad_strings() {
        assert_eq!(Err("Unclosed string!"), tokenize(r#" "abc "#));
        assert_eq!(Err("Unclosed string!"), tokenize(r#" "abc\ "#));
        assert_eq!(Err("Source ends in a backslash!"), tokenize(r#" "abc\"#));
    }

    #[test]
    fn test_bad_chars() {
        assert_eq!(Err("Unrecognized character in source!"), tokenize("!%"));
    }
}
