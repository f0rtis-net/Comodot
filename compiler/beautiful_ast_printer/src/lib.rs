use color_print::cprint;
use ast::{Visitor};
use ast::expressions::boolean_literal::BooleanLiteral;
use ast::expressions::function_literal::FunctionLiteral;
use ast::expressions::integer_literal::IntegerLiteral;
use ast::misc::file::ParsedFile;
use ast::primitives::expression::Expression;
use ast::primitives::statement::Statement;
use ast::statements::block_statement::BlockStatement;
use ast::statements::expression_statement::ExpressionStatement;
use ast::statements::return_statement::ReturnStatement;

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
        println!("{}IntegerLiteral(value: {})", self.indent(), integer.value);
    }

    fn visit_expression_statement(&mut self, statement: &ExpressionStatement) {
        println!("{}ExpressionStatement(", self.indent());
        self.indent += 2;
        statement.expression.accept(self);
        self.indent -= 2;
        println!("{}))", self.indent());
    }

    fn visit_boolean_literal(&mut self, boolean: &BooleanLiteral) {
        println!("{}BooleanLiteral(value: {})", self.indent(), boolean.val);
    }

    fn visit_function(&mut self, func: &FunctionLiteral) {
        cprint!("{}<green>FunctionLiteral(name: \"{}\", visible: {}, body: </>\n", self.indent(), func.name, func.visibility);
        self.indent += 2;
        func.body.accept(self);
        self.indent -= 2;
        cprint!("{}<green>))</>", self.indent());
    }

    fn visit_return_statement(&mut self, statement: &ReturnStatement) {
        cprint!("{} Return statement: ", self.indent());

        let saved_ident = self.indent;
        self.indent = 0;

        if statement.value.is_none() {
            cprint!("<red>None</>");
        } else {
            statement.value.as_ref().unwrap().accept(self);
        }

        self.indent = saved_ident;
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