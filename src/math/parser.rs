//! ## Build an AST from tokens and validate them
//! A parser is just that stage of code analysis where the tokens are checked for
//! correct grammar. A BinaryOp token for instance cannot appear on the left of another
//! BinaryOp and so on. For our basic calculator we must check for all legal ways
//! to construct an expression, which means checking operator precedence, groupings,
//! and lexical structure of expressions.

use super::ast::*;
use super::token::Token;
use super::tokenizer::Tokenizer;
use crate::{leaf, token_build};
use std::collections::binary_heap::Iter;
use std::iter::Peekable;

pub struct Parser<'expression> {
    token_stream: Peekable<Tokenizer<'expression>>,
    /// cursor stores the next Token returned from calling next() on token_stream
    cursor: Option<Token>,
}
#[derive(Debug)]
pub enum ParseError {
    InvalidOperator(String),
    MismatchedParen(String),
    InvalidStart(String),
    Incomplete(String),
}

impl<'expr> Parser<'expr> {
    /// A new instance of a Parser will be initialized with the first token from token_stream
    pub fn new(expr: &'expr str) -> Result<Self, ParseError> {
        let illegal_expr = ParseError::InvalidStart(
            "Illegal character detected. Unable to parse.".into(),
        );

        let mut token_stream = token_build!(&expr).peekable();
        let cursor = match token_stream.next() {
            None | Some(Token::Invalid) => return Err(illegal_expr),
            Some(token) => Some(token),
        };
        Ok(Parser {
            token_stream,
            cursor,
        })
    }

    /// Generate the AST from tokenizer_stream
    pub fn parse(&mut self) -> Result<AST, ParseError> {
        let left_child = if Self::get_kind(self.cursor.unwrap()) == NodeKind::Operand {
            Box::new(Node::new(
                NodeKind::Operand,
                self.cursor.unwrap(),
                None,
                None,
            ))
        } else {
            return Err(ParseError::InvalidStart("Invalid Start".into()));
        };
        let root = match self.token_stream.next() {
            Some(token) => {
                if Self::get_kind(token) == NodeKind::BinOp {
                    Box::new(Node::new(
                        NodeKind::BinOp,
                        token,
                        Some(left_child),
                        Self::gen_ast(&mut self.token_stream)?,
                    ))
                } else {
                    return Err(ParseError::InvalidOperator("Invalid Operator".into()));
                }
            }
            None => {
                return Err(ParseError::Incomplete("Expression too short".into()));
            }
        };
        let ast = AST { root };
        Ok(ast)
        // let mut ast = AST {
        //     root: Box::new(Node::new(
        //         Self::get_kind(self.cursor.unwrap()),
        //         // Guaranteed to be safe because `new()` checks for Token::Invalid
        //         self.cursor.unwrap(),
        //         None,
        //         None,
        //     )),
        // };
        // Let's not worry about parens for now
        // 3+4/2 =>
        // root: Node(+)
        //      /      \
        //      Node(3)  Node(/)
        //               /     \
        //          Node(4)  Node(2)

        // let current = self.cursor.unwrap();
        // if Self::get_kind(current) == NodeKind::Operand {

        // let left_child =
        //     Some(Box::new(Node::new(NodeKind::Operand, current, None, None)));
        // if let Some(possible_operator) =
        //     self.token_stream.next_if(|&possible_operator| {
        //         Self::get_kind(possible_operator) == NodeKind::BinOp
        //     })
        // {
        //     let right_child = Self::gen_ast(&mut self.token_stream)?;
        //     let parent = Node::new(
        //         NodeKind::BinOp,
        //         possible_operator,
        //         left_child,
        //         right_child,
        //     );
        // }

        // Our code doesn't allow expressions to begin with `-` right now which is the
        // only legal NodeKind::BinOp to begin an expression with
        // } else {
        //     return Err(ParseError::InvalidStart(String::from(
        //         "Invalid start of expression",
        //     )));
        // }
    }
    // If there are more tokens, return a NodeKind::BinOp otherwise return NodeKind::Operand
    fn gen_ast(
        mut token_stream: &mut Peekable<Tokenizer>,
    ) -> Result<RefNode, ParseError> {
        if let Some(token) = token_stream.next() {
            // Token::Num kinds occupy the lowest level of any branch
            if let Token::Num(_) = token {
                let next_token = token_stream.peek();
                if next_token == None {
                    return Ok(Some(Box::new(Node::new(
                        NodeKind::Operand,
                        token,
                        None,
                        Self::gen_ast(token_stream)?,
                    ))));
                } else {
                    let next_token = *next_token.unwrap();
                    let token_kind = Self::get_kind(next_token);
                    match token_kind {
                        NodeKind::BinOp => {
                            let left = leaf!(token);
                            // advance the iterator
                            token_stream.next();
                            return Ok(Some(Box::new(Node::new(
                                NodeKind::BinOp,
                                next_token,
                                Some(left),
                                Self::gen_ast(token_stream)?,
                            ))));
                        }
                        _ => {
                            return Err(ParseError::InvalidOperator(format!(
                                "Invalid token kind {:?}",
                                next_token
                            )));
                        }
                    }
                }
            }
            // else if not an operand but an operator
            // else {
            //     match token {
            //         Token::Add | Token::Div | Token::Mul | Token::Sub => {
            //             return Ok(Some(Box::new(Node::new(
            //                 NodeKind::BinOp,
            //                 token,
            //                 Self::gen_ast(token_stream)?,
            //                 Self::gen_ast(token_stream)?,
            //             ))))
            //         }
            //         _ => {
            //             return Err(ParseError::InvalidOperator(
            //                 "Invalid Operator found".into(),
            //             ))
            //         }
            //     }
            // }
        }
        // No more tokens left
        Ok(None)
    }

    // Recieve a Token and return its Node type for tree.
    // BinOp must have two children wherease Operands should be terminating nodes
    // UnaryOp should have only one child
    // Return Invalid if token fits none of the kinds
    pub fn get_kind(token: Token) -> NodeKind {
        match token {
            Token::Add | Token::Sub | Token::Mul | Token::Div => NodeKind::BinOp,
            Token::Num(_) => NodeKind::Operand,
            _ => NodeKind::Invalid,
        }
    }
}
#[macro_export]
macro_rules! leaf {
    ($e:expr) => {{
        Box::new(Node::new(NodeKind::Operand, $e, None, None))
    }};
}
