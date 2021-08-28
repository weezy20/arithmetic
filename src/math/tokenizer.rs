#![allow(unused)]
//! Reads one or more characters from expression and create a
//! or a number like `Num(23.23_f64)`

use std::collections::binary_heap::Iter;
use std::iter::Peekable;
use std::str::Chars;
/// We can store the input arithmetic expression as a string slice, since
/// we don't require ownership for the duration of processing.
/// Also note that for making tokens we would need to read not only the current
/// character but also the character that follows. If we are using a type Iterator
/// such as `Chars` which is returned from calling `chars()` on a string type, we can
/// take a look at the next character by convering it into a `Peekable` Iterator.
///
pub struct Tokenizer<'expression> {
    // expr: &'input str,
    expr: Peekable<Chars<'expression>>,
}

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
    /// `(` Left Parentheses
    LeftParen,
    /// `)` Right Parentheses
    RightParen,
    /// Number
    Num(f64),
    /// End of Line
    EOL,
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
        let mut token = None;
        let ch = self.expr.next();
        token = match ch {
            Some('0'..='9') => {
                // begin with obviously a number.
                // After every digit there can be either a binary operator,
                // or another digit, or a decimal after which no more decimals should
                // appear. We only peek if there is one decimal or if there are other digits
                let ch = ch.unwrap();
                let mut number = String::from(ch);
                // Peek until look_next is unset
                let mut look_next = true;
                let mut decimal_once = 0;
                while look_next {
                    let next_char = self.expr.peek();
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
                                look_next = false;
                                return None;
                            }
                        }
                        Some('0'..='9') => {
                            number.push(self.expr.next().unwrap());
                        }
                        Some('+' | '-' | '(' | ')' | '/' | '*' | '^') => {
                            look_next = false;
                        }
                        _ => {
                            // Unrecognized char
                            return None;
                        }
                    } // End of peeking
                }

                None
            } // End of Number tokenizer
            _ => None,
        };
        token
    } // end of next()
} // end of Iterator impl
