#![cfg(test)]
#[allow(unused, unused_mut)]
use crate::math::tokenizer::{Token, Tokenizer};

#[test]
fn test_tokenizer() {
    use Token::*;
    #[macro_export]
    macro_rules! token_build {
        ($e:expr) => {{
            Tokenizer::new(&$e)
        }};
    }
    let mut expr = "3+4/2";
    let mut tkn = token_build!(&expr);
    assert_eq!(Some(Num(3_f64)), tkn.next());
    assert_eq!(Some(Add), tkn.next());
    assert_eq!(Some(Num(4_f64)), tkn.next());
    assert_eq!(Some(Div), tkn.next());
    assert_eq!(Some(Num(2_f64)), tkn.next());

    expr = "3.001     +     4";
    tkn = token_build!(&expr);
    assert_eq!(Some(Num(3.001_f64)), tkn.next());
    assert_eq!(Some(Add), tkn.next());
    assert_eq!(Some(Num(4_f64)), tkn.next());

    // This one should fail: Token::Invalid indicates failure
    expr = "3.2123.233 / ()";
    tkn = token_build!(&expr);
    // for t in tkn {
    //     println!("{:?}", t);
    // }
    assert_eq!(Some(Num(3.2123_f64)), tkn.next() );
    assert_eq!(Some(Token::Invalid), tkn.next() );
    assert_eq!(Some(Num(233_f64)), tkn.next() );
    assert_eq!(Some(Div), tkn.next() );
    assert_eq!(Some(OpenParen), tkn.next() );
    assert_eq!(Some(CloseParen), tkn.next() );
    
    expr = "(69.96)";
    tkn = token_build!(&expr);
    assert_eq!(Some(OpenParen), tkn.next());
    assert_eq!(Some(Num(69.96_f64)), tkn.next());
    assert_eq!(Some(CloseParen), tkn.next());
    assert_eq!(None, tkn.next());
}
