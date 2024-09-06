use tokens::Token;
use crate::primitives::node::Node;
use crate::nodes::block_statement::BlockStatement;
use crate::Visitor;

#[derive(Clone)]
pub struct FunctionLiteral {
    pub name: String,
    pub return_type: Token,
    pub visibility: Token,
    pub body: BlockStatement,
}

impl Node for FunctionLiteral {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_function(self)
    }

    fn clone_boxed(&self) -> Box<dyn Node> {
        Box::new(self.clone())
    }

    fn get_literal(&self) -> String {
        format!("Function {}", self.name)
    }

    fn get_type(&self) -> String {
        String::from("Function")
    }
}
