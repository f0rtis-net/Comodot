use std::{fs::File, io::Read, path::Path};

use ast_lowering::translate_to_hir;
use hir_resolver::NamesResolver;
use llvm_codegen::test_gen;
use middle::GlobalCtx;
use parser::parse_file;
use types_lowering::hir_type_resolution;

pub struct BuildingModule<'a> {
    pub name: &'a str,
    pub path: &'a str,
    pub files: Vec<&'a str>
}

pub fn build_module(module: &BuildingModule) {
    let mut file = File::open(module.path).unwrap();
    
    let file_name = Path::new(module.path)
            .file_stem()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or("unknown");
    
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let boxed_content = Box::leak(content.into_boxed_str());
    
    let data = parse_file(file_name, boxed_content);

    let mut global_ctx = GlobalCtx::new(module.name.to_string(), String::from("x86_64"), middle::BuildType::Executable);

    let hir = translate_to_hir(global_ctx.get_module_ty_info(), &data);

    let hir_files = vec![hir];

    let mut name_resolver = NamesResolver::new();
    name_resolver.resolve_names(&hir_files);

    hir_type_resolution(&name_resolver, global_ctx.get_module_ty_info(), &hir_files);

    test_gen(&mut global_ctx, &hir_files);
}
