use crate::{AstStatement, Visitor};
use crate::primitives::node::Node;
use crate::primitives::statement::Statement;

#[derive(Clone)]
pub struct BlockStatement {
    pub statements: Vec<AstStatement>,
}

impl Node for BlockStatement {
    fn get_literal(&self) -> String {
        let mut statements = String::new();

        for statement in &self.statements {
            statements.push_str(&statement.get_literal());
        }

        format!("{{\n{}\n}}", statements)
    }
}

impl Statement for BlockStatement {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_code_block(self)
    }

    fn clone_boxed(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}