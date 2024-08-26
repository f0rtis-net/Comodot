mod visitor;

use std::error::Error;
use std::path::Path;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::{Module};
use inkwell::OptimizationLevel;
use inkwell::targets::{CodeModel, FileType, RelocMode, Target, TargetMachine};
use inkwell::values::{BasicValueEnum};
use ast::misc::file::ParsedFile;

struct CodegenVisitor<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: &'ctx Builder<'ctx>,
    pub current_value: Option<BasicValueEnum<'ctx>>,
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