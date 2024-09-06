use color_print::{cprint, cprintln};
use ast::{Visitor};
use ast::nodes::boolean_literal::BooleanLiteral;
use ast::nodes::function_literal::FunctionLiteral;
use ast::nodes::integer_literal::IntegerLiteral;
use ast::misc::file::ParsedFile;
use ast::nodes::binary_expression::BinaryExpression;
use ast::nodes::block_statement::BlockStatement;
use ast::nodes::return_expression::ReturnExpression;
use ast::primitives::node::Node;

pub struct PrintVisitor {
    indent: usize,
}

impl PrintVisitor {
    pub fn new() -> Self {
        PrintVisitor { indent: 0 }
    }

    fn indent(&self) -> String {
        " ".repeat(self.indent)
    }
}

impl Visitor for PrintVisitor {
    fn visit_code_block(&mut self, code_block: &BlockStatement) {
        cprint!("{}<yellow>BlockStatement {{</>\n", self.indent());
        for statement in &code_block.statements {
            self.indent += 2;
            statement.accept(self);
            self.indent -= 2;
        }
        cprint!("{}<yellow>}}</>\n", self.indent());
    }

    fn visit_integer_literal(&mut self, integer: &IntegerLiteral) {
        cprintln!("{}<cyan>IntegerLiteral(value: {})</>", self.indent(), integer.value);
    }

    fn visit_boolean_literal(&mut self, boolean: &BooleanLiteral) {
        cprintln!("{}<cyan>BooleanLiteral(value: {})</>", self.indent(), boolean.val);
    }

    fn visit_function(&mut self, func: &FunctionLiteral) {
        cprintln!("{}<green>Function(", self.indent());
        self.indent += 1;
        cprintln!("{}<green>name:</> <cyan>\"{}\"</>", self.indent(), func.name);
        cprintln!("{}<green>ret type:</> <cyan>{:?}</>", self.indent(), func.return_type);
        cprintln!("{}<green>visibility:</> <cyan>{:?}</>", self.indent(), func.visibility);
        cprintln!("{}<green>body:</>", self.indent());
        self.indent -= 1;

        self.indent += 2;
        func.body.accept(self);
        self.indent -= 2;
        cprintln!("{}<green>))</>", self.indent());
    }

    fn visit_binary_expression(&mut self, expr: &BinaryExpression) {
        cprintln!("{}<blue>BinaryExpression {{</>", self.indent());
        self.indent += 2;

        cprintln!("{}<blue>left:</>", self.indent());
        self.indent += 2;
        expr.left.accept(self);
        self.indent -= 2;
        cprintln!("{}<blue>right:</>", self.indent());
        self.indent += 2;
        expr.right.accept(self);
        self.indent -= 2;
        cprintln!("{}<blue>operator:</> <cyan>{:?}</>", self.indent(), expr.operator);

        self.indent -= 2;
        cprintln!("{}<blue>}}</>", self.indent());
    }

    fn visit_return_expression(&mut self, expression: &ReturnExpression) {
        cprintln!("<red>{}Return {{</>", self.indent());
        self.indent += 2;

        if expression.expr.is_none() {
            cprintln!("{}<cyan>NONE</>", self.indent());
        } else {
            expression.clone().expr.unwrap().accept(self);
        }

        self.indent -= 2;
        cprintln!("{}<red>}}</>", self.indent());
    }
}

pub fn print_ast(file: &ParsedFile) {
    println!("File metadata:");
    cprint!(" - <yellow>Hash: {}</>\n", file.get_hash());
    cprint!(" - <yellow>Filename: {}</>\n", file.get_name());
    cprint!(" - <yellow>Path: {}\n</>", file.get_path());
    println!("----------------------------------------");

    let mut visitor = PrintVisitor::new();

    for expression in &file.expressions {
        expression.accept(&mut visitor)
    }
}