/** ----------------------
 * Tokenization Types
 * -----------------------
 **/

#[derive(Debug,Clone,PartialEq,Hash,Eq)]
pub enum Token {
    Plus, Num(String)
}

/** ----------------------
 * Parsing Types
 * -----------------------
 */

#[derive(Debug,Clone,PartialEq)]
pub enum Expr {
    Num(f64),
    Add(Box<Expr>,Box<Expr>)
}
