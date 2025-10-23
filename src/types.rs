use std::collections::HashMap;

/** --------------------------------------------------------------------------------
 * Tokenization Types
 * ---------------------------------------------------------------------------------
 **/

#[derive(Debug,Clone,PartialEq,Hash,Eq)]
pub enum Token {
    LParen, RParen, LBrace, RBrace, Add, Mul, Sub, Div, Exp, Mod, Scln, Comma, Dot, Dollar,
    Assign, LessThan, GreaterThan, LessThanEq, GreaterThanEq, Equals, Return, Def,
    And, Or, If, Else, While, Num(String), Ident(String), Str(String), Bool(bool)
}

impl Token {
    pub fn get_for_single_char(ch: &char) -> Option<Token> {
        HashMap::from([
            ('(', Token::LParen), (')', Token::RParen), ('{', Token::LBrace), ('}', Token::RBrace),
            ('.', Token::Dot), ('+', Token::Add), ('*', Token::Mul), ('-', Token::Sub), ('/', Token::Div),
            ('%', Token::Mod), ('^', Token::Exp), (';', Token::Scln), (',', Token::Comma), ('$', Token::Dollar)
        ]).remove(ch)
    }

    pub fn get_for_keyword(keyword: &str) -> Option<Token> {
        HashMap::from([
            ("and", Token::And), ("or", Token::Or), ("true", Token::Bool(true)),
            ("false", Token::Bool(false)), ("if", Token::If), ("else", Token::Else),
            ("while", Token::While), ("return", Token::Return), ("def", Token::Def)
        ]).remove(keyword)
    }
}


/** --------------------------------------------------------------------------------
 * Parsing Types
 * ---------------------------------------------------------------------------------
 **/

#[derive(Debug,Clone,PartialEq)]
pub enum Expr {
    Ident(String),
    Lit(Value),
    BinOp { left: Box<Expr>, op: Operator, right: Box<Expr> },
    Call { func: String, args: Vec<Expr> }
}

#[derive(Debug,Clone,PartialEq)]
pub enum Stmt {
    Single(Expr),
    Return(Expr),
    While { cond: Expr, body: Program },
    Assign { left: String, right: Expr },
    If { cond: Expr, body: Program },
    IfElse { cond: Expr, then: Program, else_: Program },
    Function { name: String, params: Vec<String>, body: Vec<Stmt> }
}

pub type Program = Vec<Stmt>; // A program is a list of statements

#[derive(Debug,Clone,PartialEq)]
pub enum Value {
    Str(String),
    Num(i64),
    Bool(bool),
    Function { params: Vec<String>, body: Vec<Stmt> },
    Nothing
}

impl Value {
    pub fn expect_str(self) -> Result<String,&'static str> {
        if let Value::Str(s) = self {
            Ok(s)
        } else {
            Err("Expecting value to be string!")
        }
    }

    pub fn expect_num(self) -> Result<i64,&'static str> {
        if let Value::Num(n) = self {
            Ok(n)
        } else {
            Err("Expecting value to be number!")
        }
    }

    pub fn expect_bool(self) -> Result<bool,&'static str> {
        if let Value::Bool(b) = self {
            Ok(b)
        } else {
            Err("Expecting value to be boolean!")
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            Value::Str(s) => s.clone(),
            Value::Num(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Function { params:_, body:_ } => "<Function>".to_string(),
            Value::Nothing => "<Nothing>".to_string()
        }
    }

    pub fn is_function(&self) -> bool {
        match self {
            Value::Function { params:_, body:_ } => true,
            _ => false
        }
    }
}


#[derive(Debug,PartialEq,Copy,Clone)]
pub enum Operator {
    Add, Subtract, Divide, Multiply, Exponentiate, Modulo, And, Concat,
    Or, LessThan, GreaterThan, LessThanEq, GreaterThanEq, Equals
}

impl Operator {
    pub fn from_token(token: &Token) -> Option<Self> {
        return Some(match token {
            Token::Add => Operator::Add,
            Token::Sub => Operator::Subtract,
            Token::Mul => Operator::Multiply,
            Token::Div => Operator::Divide,
            Token::Exp => Operator::Exponentiate,
            Token::Mod => Operator::Modulo,
            Token::And => Operator::And,
            Token::Or => Operator::Or,
            Token::LessThan => Operator::LessThan,
            Token::GreaterThan => Operator::GreaterThan,
            Token::LessThanEq => Operator::LessThanEq,
            Token::GreaterThanEq => Operator::GreaterThanEq,
            Token::Equals => Operator::Equals,
            Token::Dot => Operator::Concat,
            _ => return None
        });
    }
}

/** --------------------------------------------------------------------------------
 * Evaluation Types
 * ---------------------------------------------------------------------------------
 **/

pub type Environment = HashMap<String,Value>;
