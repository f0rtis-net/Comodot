use crate::{AstExpression, Visitor};
use crate::primitives::node::Node;
use crate::primitives::statement::Statement;

#[derive(Clone)]
pub struct ExpressionStatement {
    pub expression: AstExpression
}

impl Node for ExpressionStatement {
    fn get_literal(&self) -> String {
        self.expression.get_literal()
    }
}

impl Statement for ExpressionStatement {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_expression_statement(self)
    }

    fn clone_boxed(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}