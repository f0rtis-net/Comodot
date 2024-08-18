use tokens::TokenType;
use crate::primitives::expression::Expression;
use crate::primitives::node::Node;
use crate::statements::block_statement::BlockStatement;
use crate::Visitor;

#[derive(Clone)]
pub struct FunctionLiteral {
    pub name: String,
    pub body: BlockStatement,
    pub return_type: TokenType,
    pub visibility: bool
}

impl Node for FunctionLiteral {
    fn get_literal(&self) -> String {
        format!("Function {}", self.name)
    }
}

impl Expression for FunctionLiteral {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_function(self)
    }

    fn clone_boxed(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}