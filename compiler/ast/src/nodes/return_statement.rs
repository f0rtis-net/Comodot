use crate::{AstNode, Visitor};
use crate::primitives::node::Node;

#[derive(Clone)]
pub struct ReturnStatement {
    pub value: Option<AstNode>
}

impl Node for ReturnStatement {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_return_statement(self)
    }

    fn clone_boxed(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }

    fn get_literal(&self) -> String {
        todo!()
    }
}
