use crate::{AstNode, Visitor};
use crate::primitives::node::Node;

#[derive(Clone)]
pub struct ReturnExpression {
    pub expr: Option<AstNode>
}

impl Node for ReturnExpression {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_return_expression(self);
    }

    fn clone_boxed(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }

    fn get_literal(&self) -> String {
        todo!()
    }

    fn get_type(&self) -> String {
        String::from("ReturnStatement")
    }
}