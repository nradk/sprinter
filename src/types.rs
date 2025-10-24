/** ----------------------
 * Tokenization Types
 * -----------------------
 **/

#[derive(Debug,Clone,PartialEq,Hash,Eq)]
pub enum Token {
    Plus, Mult, Num(String)
}

/** ----------------------
 * Parsing Types
 * -----------------------
 */

#[derive(Debug,Clone,PartialEq)]
pub enum Expr {
    Factor(Factor),
    Add(Factor,Box<Expr>)
}

#[derive(Debug,Clone,PartialEq)]
pub enum Factor {
    Num(f64),
    Mult(f64,Box<Factor>)
}
