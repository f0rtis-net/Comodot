use crate::nodes::boolean_literal::BooleanLiteral;
use crate::nodes::function_literal::FunctionLiteral;
use crate::nodes::integer_literal::IntegerLiteral;
use crate::primitives::node::Node;
use nodes::block_statement::BlockStatement;
use crate::nodes::binary_expression::BinaryExpression;
use crate::nodes::return_expression::ReturnExpression;

pub mod primitives;
pub mod nodes;
pub mod misc;

pub type AstNode = Box<dyn Node>;

pub trait Visitor {
    fn visit_code_block(&mut self, code_block: &BlockStatement);
    fn visit_integer_literal(&mut self, integer: &IntegerLiteral);
    fn visit_boolean_literal(&mut self, boolean: &BooleanLiteral);
    fn visit_function(&mut self, func: &FunctionLiteral);
    fn visit_binary_expression(&mut self, expression: &BinaryExpression);
    fn visit_return_expression(&mut self, expr: &ReturnExpression);
}