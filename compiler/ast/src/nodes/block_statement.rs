use crate::{AstNode, Visitor};
use crate::primitives::node::Node;

#[derive(Clone)]
pub struct BlockStatement {
    pub statements: Vec<AstNode>,
}

impl Node for BlockStatement {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_code_block(self)
    }

    fn clone_boxed(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }

    fn get_literal(&self) -> String {
        let mut statements = String::new();

        for statement in &self.statements {
            statements.push_str(&statement.get_literal());
        }

        format!("{{\n{}\n}}", statements)
    }

    fn get_type(&self) -> String {
        String::from("BlockStatement")
    }
}