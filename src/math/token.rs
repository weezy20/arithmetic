
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Token {
    /// `+`  Addition
    Add,
    /// `*`  Multiplication
    Mul,
    /// `/`  Division
    Div,
    /// `-`  Subtraction
    Sub,
    /// `^` Exponentiation
    Exp,
    /// `(` Open Parentheses
    OpenParen,
    /// `)` Close Parentheses
    CloseParen,
    /// Number
    Num(f64),
    /// Invalid token
    Invalid,
    // End of expression
    // EOF,
}

