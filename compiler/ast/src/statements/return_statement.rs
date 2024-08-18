use crate::{AstExpression, Visitor};
use crate::primitives::node::Node;
use crate::primitives::statement::Statement;

#[derive(Clone)]
pub struct ReturnStatement {
    pub value: Option<AstExpression>
}

impl Node for ReturnStatement {
    fn get_literal(&self) -> String {
        todo!()
    }
}

impl Statement for ReturnStatement {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_return_statement(self)
    }

    fn clone_boxed(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}