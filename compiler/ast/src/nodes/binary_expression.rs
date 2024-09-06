use tokens::Token;
use crate::{AstNode, Visitor};
use crate::primitives::node::Node;

#[derive(Clone)]
pub struct BinaryExpression {
    pub left: AstNode,
    pub right: AstNode,
    pub operator: Token,
}

impl Node for BinaryExpression {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_binary_expression(self);
    }

    fn clone_boxed(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }

    fn get_literal(&self) -> String {
        todo!()
    }

    fn get_type(&self) -> String {
        String::from("BinaryExpression")
    }
}