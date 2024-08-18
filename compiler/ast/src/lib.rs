use crate::expressions::boolean_literal::BooleanLiteral;
use crate::expressions::function_literal::FunctionLiteral;
use crate::expressions::integer_literal::IntegerLiteral;
use crate::primitives::expression::Expression;
use crate::primitives::statement::Statement;
use crate::statements::block_statement::BlockStatement;
use crate::statements::expression_statement::ExpressionStatement;
use crate::statements::return_statement::ReturnStatement;

pub mod primitives;
pub mod expressions;
pub mod statements;
pub mod misc;

pub type AstStatement = Box<dyn Statement>;
pub type AstExpression = Box<dyn Expression>;

pub trait Visitor {
    fn visit_code_block(&mut self, code_block: &BlockStatement);
    fn visit_integer_literal(&mut self, integer: &IntegerLiteral);
    fn visit_expression_statement(&mut self, statement: &ExpressionStatement);
    fn visit_boolean_literal(&mut self, boolean: &BooleanLiteral);
    fn visit_function(&mut self, func: &FunctionLiteral);
    fn visit_return_statement(&mut self, statement: &ReturnStatement);
}