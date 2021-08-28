#![allow(unused)]
//! Reads one or more characters from expression and create a
//! or a number like `Num(23.23_f64)`

use std::iter::Peekable;
use std::str::Chars;
/// We can store the input arithmetic expression as a string slice, since
/// we don't require ownership for the duration of processing.
/// Also note that for making tokens we would need to read not only the current
/// character but also the character that follows. If we are using a type Iterator
/// such as `Chars` which is returned from calling `chars()` on a string type, we can
/// take a look at the next character by convering it into a `Peekable` Iterator.
/// Also our tokenizer ignores whitespaces
pub struct Tokenizer<'expression> {
    // expr: &'input str,
    expr: Peekable<Chars<'expression>>,
}
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
}

impl<'a> Tokenizer<'a> {
    /// Build a Tokenizer instance from raw string input
    pub fn new(s: &'a str) -> Self {
        let mut expr = s.chars().peekable();
        Self { expr }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;
    /// Reads a character and/or the character following it to construct
    fn next(&mut self) -> Option<Self::Item> {
        // "3+4*(3/2)+6"
        let mut ch = self.expr.next();
        while ch == Some(' ') {
            ch = self.expr.next();
        }
        let mut token = match ch {
            Some('0'..='9') => {
                // begin with obviously a number.
                // After every digit there can be either a binary operator,
                // or another digit, or a decimal after which no more decimals should
                // appear. We only peek if there is one decimal or if there are other digits
                let ch = ch.unwrap();
                let mut number = String::from(ch);
                // Peek until look_next is unset
                let mut look_next = true;
                let mut decimal_once: u8 = 0;
                while look_next {
                    let next_char = self.expr.peek();
                    // Start peeking
                    match next_char {
                        None => {
                            // End of Expression reached
                            look_next = false;
                        }
                        Some('.') => {
                            if decimal_once == 0 {
                                decimal_once += 1;
                                // Push the dot while stepping our iterator
                                number.push(self.expr.next().unwrap());
                            } else if decimal_once >= 1 {
                                // Something went wrong here
                                // must end parsing
                                look_next = false;
                                println!("Multiple decimal points!");
                                println!("Number scanned upto: {:?}" , number);
                            }
                        }
                        Some('0'..='9') => {
                            number.push(self.expr.next().unwrap());
                        }
                        Some('+' | '-' | '(' | ')' | '/' | '*' | '^') => {
                            look_next = false;
                        }
                        Some(t) => {
                            // Unrecognized char
                            println!("Unrecognized token: {:?}", t);
                            look_next = false;
                        }
                    } // End of peeking
                }
                let number = number.parse::<f64>();

                if let Ok(number) = number {
                    Some(Token::Num(number))
                } else {
                    None
                }
            } // End of Number tokenizer
            Some('+') => Some(Token::Add),
            Some('-') => Some(Token::Sub),
            Some('*') => Some(Token::Mul),
            Some('/') => Some(Token::Div),
            Some('^') => Some(Token::Exp),
            Some('(') => Some(Token::OpenParen),
            Some(')') => Some(Token::CloseParen),
            None => None,
            // None => Some(Token::EOF),
            // This is bad design. Iterators should end with a None
            // not a Some. For example if it was Some(Token::EOF)
            // and you were using a for loop to iterate over Tokernizer
            // the loop would never end
            _ => Some(Token::Invalid),
        };
        token
    } // end of next()
} // end of Iterator impl
