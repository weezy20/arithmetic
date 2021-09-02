use super::token::Token;
#[derive(Debug)]
pub struct AST {
    pub root: Box<Node>,
}
pub type RefNode = Option<Box<Node>>;
#[derive(Debug)]
pub struct Node {
    kind: NodeKind,
    val: Token,
    left: RefNode,
    right: RefNode,
}
#[derive(PartialEq, Debug)]
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
