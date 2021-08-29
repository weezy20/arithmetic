#![allow(unused)]
mod math;
use math::token::Token;
use math::tokenizer::Tokenizer;
fn main() {
    let quit = "quitexit";
    let mut buf = String::new();
    let stdin = std::io::stdin();
    loop {
        buf.clear();
        println!("Enter an expression to evaluate");
        stdin.read_line(&mut buf);
        remove_crlf(&mut buf);
        if quit.contains(&buf) {
            break;
            println!("Exiting ...");
        }
        let tkn = Tokenizer::new(&buf);
        for t in tkn {
            print!("{:?} ", t);
        }
        println!("\n****************************");
    }
}

fn remove_crlf(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}
