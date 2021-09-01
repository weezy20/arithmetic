#![allow(unused)]
mod math;
use math::parser::{ParseError, Parser};
use math::{token::Token, tokenizer::Tokenizer};
use math::ast::*;
use std::io::Write;
fn main() -> Result<(), ParseError> {
    let quit = "quitexit";
    let mut buf = String::new();
    let stdin = std::io::stdin();
    loop {
        buf.clear();
        println!("Enter an expression to evaluate");
        print!("> ");
        std::io::stdout().flush();
        stdin.read_line(&mut buf);
        remove_crlf(&mut buf);
        if quit.contains(&buf) {
            break;
            println!("Exiting ...");
        }
        // let tkn = Tokenizer::new(&buf);
        // for t in tkn {
        //     print!("{:?} ", t);
        // }
        // println!("\n****************************");
        let mut parser = Parser::new(&buf)?;
        let mut ast = parser.parse()?;
        println!("Printing Abstract syntax tree:");
        println!("{:#?}", ast);
    }
    Ok(())
}

fn remove_crlf(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}
