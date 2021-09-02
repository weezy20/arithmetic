use std::ops::Deref;

use super::token::Token;
#[derive(Debug)]
pub struct AST {
    pub root: Box<Node>,
}
pub type RefNode = Option<Box<Node>>;
#[derive(Debug, Clone)]
pub struct Node {
    kind: NodeKind,
    val: Token,
    left: RefNode,
    right: RefNode,
}
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum NodeKind {
    Operand,
    BinOp,
    UnaryOp,
    Invalid,
}

impl Node {
    pub fn new(kind: NodeKind, val: Token, left: RefNode, right: RefNode) -> Self {
        Node {
            kind,
            val,
            left,
            right,
        }
    }
}

impl AST {
    /// Compute the value of the AST and return
    pub fn eval(&mut self) -> Option<f64> {
        let left_node = *self.root.left.clone().unwrap();

        let left_val = get_node_val(left_node)?;
        let bin_op = self.root.val;
        let right_val = Self::compute_tree(self.root.right.clone())?;

        let result = Self::eval_expr(left_val, bin_op, right_val);
        result
    }
    pub fn eval_expr(left_val: f64, bin_op: Token, right_val: f64) -> Option<f64> {
        match bin_op {
            Token::Add => Some(left_val + right_val),
            Token::Sub => Some(left_val - right_val),
            Token::Div => Some(left_val / right_val),
            Token::Mul => Some(left_val * right_val),
            _ => None,
        }
    }

    /// Recursively traverse a tree, and compute value
    pub fn compute_tree(parent: Option<Box<Node>>) -> Option<f64> {
        if let Some(parent) = parent {
            match parent.kind {
                NodeKind::BinOp => {
                    let left = Self::compute_tree(parent.left)?;
                    let right = Self::compute_tree(parent.right)?;
                    let res = Self::eval_expr(left, parent.val, right);
                    return res;
                }
                NodeKind::Operand => return get_node_val(*parent),
                NodeKind::Invalid | _ => return None,
            }
        }
        None
    }
}

/// Unwrap a Token::Num()
pub fn get_node_val(n: Node) -> Option<f64> {
    if let Token::Num(number) = n.val {
        Some(number)
    } else {
        None
    }
}
