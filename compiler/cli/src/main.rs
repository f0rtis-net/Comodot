use clap::Arg;
use codegen::test_gen;
use mir::IttTreeBuilder;
use itt_resolver::IttTreeTypeResolver;
use itt_symbol_misc::{func_table::FunctionSymbolTable, function_table_builder};
use itt_validator::IttTreeValidator;
use parser::Parser;
use std::{cell::RefCell, fs::File, io::Read};

fn main() {
    let app = clap::Command::new("comodot_cli")
        .version("v0.0.1")
        .about("comodot language compiler")
        .arg(
            Arg::new("project")
                .required(false)
        )
        .arg(
            Arg::new("name")
                .short('n')
                .required(false)
        )
        .get_matches();
    
    /*if !app.args_present() {
        println!("Invalid usage. type --help to get command list");
    }*/
    
    let mut file = File::open("main.cd").unwrap();
    
    let mut content = String::new();
    
    file.read_to_string(&mut content).unwrap();

    let parse_result = Parser::generate_parsed_unit_from_input("test_unit", content.as_str());
    
    let translator = IttTreeBuilder::new();
    
    let mut unit = translator.translate(&parse_result);
    
    let functions_table = RefCell::new(FunctionSymbolTable::new());
    
    function_table_builder(&unit, &functions_table);
    
    let mut types_resolver = IttTreeTypeResolver::new(&functions_table);
    
    types_resolver.process_tree(&mut unit);
    
    let validator = IttTreeValidator::new(&functions_table, &unit);
    
    validator.validate_tree();
    
    test_gen(&unit, &functions_table.borrow());
}