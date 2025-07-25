use std::{fs::File, io::Read, path::Path};

use ast_lowering::translate_to_hir;
use middle::GlobalCtx;
use parser::parse_file;
use types_lowering::TypeResolver;

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

    let mut ty_resolver = TypeResolver::new();

    ty_resolver.hir_type_resolution(global_ctx.get_module_ty_info(), &vec![hir]);
}
