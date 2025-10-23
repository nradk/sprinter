---
title: "Building an interpreter in Rust"
author: Neeraj Adhikari
---

Hello!
===
<!-- end_slide -->

Caveats
===
- Not a detailed tutorial. Only a high-level look at how interpreters work and a look at the structure of a simple
  one implemented in Rust.
- I am not an expert in either interpreters or Rust, so there is a lot of room for improvement!
<!-- end_slide -->

What is an interpreter?
===
- An interpreter is a program that _executes_ programs
- The two paradigms of language implementation: _compilation_ and _interpretation_
    - But the lines are a bit blurred
<!-- end_slide -->

Why would you want one?
===
Benefits of an interpreter (vs a compiler)
- Far less complex
    - Mainly because evaluation is far less complex than code generation
    - You can use the host language's features to implement your language
- Can be embedded within larger programs
    - This allows users to extend/customize the behavior of the program
    - Examples
        - JavaScript in browsers
        - VBA in Microsoft Office
        - Lua/VimScript in (Neo)Vim, Emacs Lisp in Emacs
        - Lua in many video games
        - etc. etc.
- Can serve as sandboxed execution environments
- Can emulate/virtualize other machines
<!-- end_slide -->

How does an interpreter work?
===
Three major steps:
1. Tokenization
2. Parsing
3. Execution

<!-- end_slide -->

Tokenization (Lexical Analysis)
===
Turning a string/file/stream of bytes into a list of lexical **tokens**. Tokens are
the smallest unit of meaning in a language.

Tokenization converts something like
```
a = f(b + 1);
```
into something like
```
[
    Identifier(a), AssignmentOperator, Identifier(f), LeftParen,  Identifer(b),
    PlusOperator, IntValue(1), RightParen, Semicolon
]
```
<!-- end_slide -->

Parsing (Syntactic Analysis)
===
Turning a list of tokens into an **Abstract Syntax Tree (AST)**

Parsing converts something like the token list we just saw into something like this tree:
```
            Assignment
                |
    ------------------------------
    |                            |
Identifier(a)               FunctionCall
                                 |
                  ------------------------------
                  |                            |
             Identifier(f)              SumExpression
                                               |
                                   --------------------------
                                   |                        |
                            Identifier(b)              IntValue(1)
```

To do this, the parser needs a **grammar** for your language

A **grammar** is the set of rules that specify what valid syntactic structures in your language look like.

For this language, the grammar might include the rules

```
...
Assignment -> Identifier "=" Expr
Expr       -> Identifier | Number | Expr "+" Expr
SumExpr    -> Expr "+" Expr
FuncCall   -> Identifier "(" Expr ")"
...
```

<!-- end_slide -->

Parsing Techniques
===
- Parsing is a huge field! There are a bunch of different parsing algorithms, but they can generally be classified into
  **top-down parsing** and **bottom-up parsing**.
- We will only look at **Recursive Descent Parsing**, which is a top-down parser built using mutually recursive
  functions that closely mirror the language's grammar.
- In a "real world" language, you would define the grammar using a language like BNF or EBNF, then use a **parser
  generator**, which produces the parsing logic based on your grammar. No need to write the parsing logic yourself!
- Parser generators usually generate LL, LR, or LALR parsers instead of Recursive Descent Parsers.
- Examples:
    - YACC for C
    - LALRPOP, Pest, etc. for Rust

<!-- end_slide -->


Execution
===
- Execution is the step that takes in the AST and actually runs the program.
- Typically involves 'walking' the AST: execute the leaves, use the values of
  those to execute nodes above them, and so on, until the whole thing is done.
- Since trees are recursive structures, this is very naturally implemented as
  a group of mutually recursive functions.
- For example, to execute/evaluate a sum expression, we would need to evaluate both
  sides of the expression, then add the resulting values together. Logic like this
  can be expressed as

```Rust
fn evaluate(expr: Expression) -> Number { // Assuming the language has only one type: numbers
    match expr {
        Expression::Sum { left: Expression, right: Expression } => evaluate(left) + evaluate(right)
        .....
        .....
    }
}
- Notice how Rust's data-carrying enums and pattern matching make this very natural to write!
```
<!-- end_slide -->

Minimal Example
===
A language that only allows (whole) numbers and addition expressions

https://github.com/nradk/sprinter/tree/microlang

<!-- end_slide -->

A Language With Some More Features
===
Some goals:
- Absolute basics: variables, assignment, infix binary operators.
- Syntax similar to popular languages like Python, C-style languages, etc.
- Ability to manipulate values of basic types: numbers, strings and booleans.
- The bare minimum control structures: while loops and if-then-else statements.
- Ability to define and call functions.
- Bare minimum in IO: `print()` and `input()`.
- An implementation not too complex for a presentation like this!

Simplifying constraints:
- Dynamically typed. Variables have no types, only values do.
- Variables are created when first assigned.
- No operator precedence levels. All binary operators have the same precedence and are
  right-associative.
- There is only one numeric type. Internally, it is represented as an `i64`.
- Functions can only be defined at the top level and can't reference anything except other functions.
  This means no lambdas, closures or 'global' variables.
- Special function call syntax: `$print(...)` for easier parsing.
- No "real world" language features: structs, classes, enums, objects, packages, modules, etc.
<!-- end_slide -->

Example Programs
===
Print the factorial of 5:
```
n = 1;
p = 1;
while n <= 5 {
    p = p * n;
    n = n + 1;
}
print(p);
```
<!-- end_slide -->

Abstract Syntax
===
```
Lit  -> NumLit | StrLit | BoolLit
Call -> Ident ( [Expr] )
Expr -> Ident
        | Lit
        | Expr OP Expr
        | Ident ( [Expr] )
        | $ Call
        | ( Expr )
Stmt -> Expr ;
        | while Expr { [Stmt] }
        | Ident = Expr ;
        | if Expr { Stmt } <else { Stmt }>
        | def Ident ( [Ident] ) { [Stmt] }
        | return Expr ;
Program -> [Stmt]
```
This is a very simplified (and so a somewhat inaccurate) description of the syntax. It does not capture some important
rules like the fact that functions are only allowed at the top level, and that return statements are only allowed inside
functions.

<!-- end_slide -->

Design Decisions
===
- Tokenization and parsing by "hand" as opposed to using a lexer/parser generator.
- The beauty of **Recursive descent parsing** is that the structure of the parser almost exactly mirrors the abstract
  syntax.

<!-- end_slide -->
Rust Features I Love For Language Implementation
===
- Data-carrying Enums and Structs make it easy to model tree-like structures.
- Pattern matching and exhaustiveness checking means you never forget to handle a case!
- `Option` and `Result` types means it is difficult to ignore failure.
- The `?` operator for the `Result` type allows propagating failures with elegance and conciseness!
- Iterators and their features like `map`, `filter`, etc. make it easy to write concise,
  functional-like code.

<!-- end_slide -->
Code Walkthrough
===

https://github.com/nradk/sprinter/tree/minilang

<!-- end_slide -->
Questions?
===
