use std::{fs::File, io::Read, path::Path};

use ast_lowering::translate_to_hir;
use hir_resolver::resolve_module;
use llvm_codegen::generate_object_code;
use middle::GlobalCtx;
use parser::parse_file;
use type_checker::validate_hir;
use types_lowering::type_hir_module;

pub struct BuildingModule<'a> {
    pub name: &'a str,
    pub path: &'a str,
    pub files: Vec<&'a str>
}

pub fn build_module(module: &BuildingModule) {
    let mut ctx = GlobalCtx::new(
        module.name.to_string(), 
        "x86_64".to_string(), 
        middle::BuildType::Executable
    );

    let mut parsed = Vec::new();

    for file in &module.files {
        let file_name = Path::new(file)
            .file_stem()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or("unknown");

        let mut file = File::open(file).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let boxed_content = Box::leak(content.into_boxed_str());

        parsed.push(parse_file(file_name, boxed_content));
    }

    for ast in &parsed {
        ctx.module_files.push(
            translate_to_hir(ast)
        );
    }
    
    resolve_module(&mut ctx);

    type_hir_module(&mut ctx);

    validate_hir(&ctx);

    generate_object_code(&ctx);
}
