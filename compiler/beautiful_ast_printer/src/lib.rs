use ast::{Visitor};
use ast::expressions::integer_literal::IntegerLiteral;
use ast::primitives::expression::Expression;
use ast::primitives::node::Node;
use ast::primitives::statement::Statement;
use ast::statements::block_statement::BlockStatement;
use ast::statements::expression_statement::ExpressionStatement;

struct Printer {
    representation: String,
}

impl Visitor for Printer {
    fn visit_code_block(&mut self, code_block: &BlockStatement) {
        self.representation.push_str("Code block: \n");

        let mut statements = String::new();

        for statement in code_block.statements.iter() {
            statements.push_str(format!("  - Integer literal: {}", statement.get_literal()).as_str());
            statements.push_str("\n");
        }

        self.representation.push_str(statements.as_str());
    }

    fn visit_integer_literal(&mut self, integer: &IntegerLiteral) {
        self.representation.push_str(format!("Integer literal: {}", integer.value).as_str());
    }

    fn visit_expression_statement(&mut self, statement: &ExpressionStatement) {
        self.representation.push_str(&statement.expression.get_literal())
    }
}

pub fn print_ast() {
    let int = IntegerLiteral{value: 1};

    let int_expr = ExpressionStatement{expression: int.clone_boxed()};

    let block = BlockStatement{statements: vec![int_expr.clone_boxed()]};

    let mut visitor = Printer {representation: String::new()};

    block.accept(&mut visitor);

    println!("{}", visitor.representation)
}