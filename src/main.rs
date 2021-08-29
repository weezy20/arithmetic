#![allow(unused)]
mod math;
use math::tokenizer::Tokenizer;
use math::token::Token;
fn main() {
    let quit = "quitexit";
    let mut buf = String::new();
    let stdin = std::io::stdin();
    loop {
        buf.clear();
        println!("Enter an expression to evaluate");
        stdin.read_line(&mut buf);
        if quit.contains(&buf[..]) {
            break;
            println!("Exiting ...");
        }
        let tkn = Tokenizer::new(&buf);
        for t in tkn {
            print!("{:?} ", t);
        }
        println!("");
    }
}
