use std::error::Error;
use std::path::Path;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::{Linkage, Module};
use inkwell::OptimizationLevel;
use inkwell::targets::{CodeModel, FileType, RelocMode, Target, TargetMachine};
use inkwell::values::{BasicValue, BasicValueEnum};
use ast::expressions::boolean_literal::BooleanLiteral;
use ast::expressions::function_literal::FunctionLiteral;
use ast::expressions::integer_literal::IntegerLiteral;
use ast::misc::file::ParsedFile;
use ast::primitives::statement::Statement;
use ast::statements::block_statement::BlockStatement;
use ast::statements::expression_statement::ExpressionStatement;
use ast::statements::return_statement::ReturnStatement;
use ast::Visitor;

struct CodegenVisitor<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: &'ctx Builder<'ctx>,
    pub current_value: Option<BasicValueEnum<'ctx>>,
}

impl<'ctx> Visitor for CodegenVisitor<'ctx> {
    fn visit_code_block(&mut self, code_block: &BlockStatement) {
        for statement in &code_block.statements {
            statement.accept(self)
        }
    }

    fn visit_integer_literal(&mut self, integer: &IntegerLiteral) {
        let int_value = self.context.i64_type().const_int(integer.value as u64, false);
        self.current_value = Some(BasicValueEnum::from(int_value));
    }

    fn visit_expression_statement(&mut self, statement: &ExpressionStatement) {

    }

    fn visit_boolean_literal(&mut self, boolean: &BooleanLiteral) {

    }

    fn visit_function(&mut self, func: &FunctionLiteral) {
        let function_type = self.context.i64_type().fn_type(&[], false);
        let func_declaration = self.module.add_function(func.name.as_str(), function_type, None);

        let func_body = self.context.append_basic_block(func_declaration, "entry");
        self.builder.position_at_end(func_body);

        func.body.accept(self)
    }

    fn visit_return_statement(&mut self, statement: &ReturnStatement) {
        if let Some(value) = &statement.value {
            value.accept(self);
            let return_value = self.current_value.as_ref().map(|v| v as &dyn BasicValue);
            self.builder.build_return(return_value).unwrap();
        } else {
            self.builder.build_return(None).unwrap();
        }
    }
}

pub fn test_generation(file: &ParsedFile) -> Result<(), Box<dyn Error>> {
    //init environment
    let context = Context::create();
    let module = context.create_module(file.get_name().as_str());
    let builder = context.create_builder();

    let mut visitor = CodegenVisitor {
        context: &context,
        module,
        builder: &builder,
        current_value: None,
    };

    for expr in &file.expressions {
        expr.accept(&mut visitor);
    }

    //generate object file
    Target::initialize_all(&Default::default());

    let triple = TargetMachine::get_default_triple();

    visitor.module.set_triple(&triple);

    let path = Path::new("test.ll");

    visitor.module.print_to_file(path).unwrap();

    let target = Target::from_triple(&triple).unwrap();

    let machine = target.create_target_machine(
        &triple,
        "generic",
        "",
        OptimizationLevel::Default,
        RelocMode::Default,
        CodeModel::Default
    ).expect("Could not create target machine");

    let path = Path::new("output.o");

    machine.write_to_file(&visitor.module, FileType::Object, path).expect("Failed to write object file");

    Ok(())
}