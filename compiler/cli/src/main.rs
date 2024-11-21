use ast::ParsedUnit;
use clap::Arg;
use codegen::test_gen;
use itt::{IttTreeBuilder, TypedUnit};
use itt_resolver::IttTreeTypeResolver;
use itt_symbol_misc::{func_table::GlobalFunctionSymbolTable, function_table_builder};
use parser::generate_parsed_unit_from_input;
use std::{fs::File, io::Read, path::Path};

fn parse_file<'input>(file_path: &'input str) -> ParsedUnit<'input> {
    let mut file = File::open(file_path).unwrap();
    
    let file_name = Path::new(file_path)
            .file_stem()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or("unknown");
    
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let boxed_content = Box::leak(content.into_boxed_str());
    
    generate_parsed_unit_from_input(file_name, boxed_content)
}

fn main() {
    let app = clap::Command::new("comodotc")
        .version("v0.0.1")
        .about("comodot language compiler")
        .arg(
            Arg::new("output")
                .short('n')
                .long("output")
                .help("Specify the output file")
                .required(false),
        )
        .arg(
            Arg::new("files")
                .short('f')
                .long("files")
                .help("Specify the input files")
                .required(true)
                .num_args(1..)
        )
        .get_matches();

    if !app.args_present() {
        println!("Invalid usage. Type --help to get command list");
        return;
    }

    let files: Vec<_> = app
        .get_many::<String>("files")
        .unwrap()
        .map(|s| s.as_str())
        .collect();
    
    let mut translated_units: Vec<TypedUnit> = Vec::new();
    
    let mut global_table = GlobalFunctionSymbolTable::new();
    let translator = IttTreeBuilder::new();
     
    for file in files {
        let ast = parse_file(file);
        let unit = translator.translate(&ast);
        function_table_builder(&unit, &mut global_table);
        translated_units.push(unit.clone());
    }
    
    let mut types_resolver = IttTreeTypeResolver::new();
    
    for translated_unit in &mut translated_units {
        types_resolver.process_tree(&global_table, translated_unit);
    }
    
    let mut path_to_generated_obj_files: Vec<String> = Vec::new();
    
    for ready_to_compile in translated_units {
        path_to_generated_obj_files.push(test_gen(&ready_to_compile));
    }
    
    // linker module in future ...
}