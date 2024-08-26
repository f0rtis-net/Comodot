use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::BasicValue;
use ast::nodes::binary_expression::BinaryExpression;
use ast::nodes::block_statement::BlockStatement;
use ast::nodes::boolean_literal::BooleanLiteral;
use ast::nodes::function_literal::FunctionLiteral;
use ast::nodes::integer_literal::IntegerLiteral;
use ast::nodes::return_statement::ReturnStatement;
use ast::primitives::node::Node;
use ast::Visitor;

pub struct CodegenVisitor<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: &'ctx Builder<'ctx>,
    generated_value: Box<dyn BasicValue<'ctx>>
}

impl<'ctx> Visitor for CodegenVisitor<'ctx> {
    fn visit_code_block(&mut self, code_block: &BlockStatement) {
        for statement in &code_block.statements {
            statement.accept(self)
        }
    }

    fn visit_integer_literal(&mut self, integer: &IntegerLiteral) {

    }

    fn visit_boolean_literal(&mut self, boolean: &BooleanLiteral) {

    }

    fn visit_function(&mut self, func: &FunctionLiteral) {

    }

    fn visit_return_statement(&mut self, statement: &ReturnStatement) {

    }

    fn visit_binary_expression(&mut self, expression: &BinaryExpression) {

    }
}