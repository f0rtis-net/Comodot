use crate::expressions::integer_literal::IntegerLiteral;
use crate::primitives::expression::Expression;
use crate::primitives::statement::Statement;
use crate::statements::block_statement::BlockStatement;
use crate::statements::expression_statement::ExpressionStatement;

pub mod primitives;
pub mod expressions;
pub mod statements;

pub type AstStatement = Box<dyn Statement>;
pub type AstExpression = Box<dyn Expression>;

pub struct CodeBlock {
    pub expressions: Vec<AstStatement>,
}

pub trait Visitor {
    fn visit_code_block(&mut self, code_block: &BlockStatement);
    fn visit_integer_literal(&mut self, integer: &IntegerLiteral);
    fn visit_expression_statement(&mut self, statement: &ExpressionStatement);
}