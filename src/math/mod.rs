//! This module contains all the logic required to process
//! arithmentic expressions with operators: `+`, `-`, `/`, `*`, `^`
//! We use `(`,`)` as brackets for enclosing other expressions

pub mod ast;
pub mod parser;
mod test;
pub mod token;
pub mod tokenizer;
