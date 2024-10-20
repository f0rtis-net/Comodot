use clap::Arg;
use codegen::test_gen;
use itt::{IttTreeBuilder, IttType, IttVisibility};
use itt_resolver::IttTreeTypeResolver;
use itt_symbol_misc::{func_table::{FunctionSymbolTable, TableFunction}, function_table_builder};
use itt_validator::IttTreeValidator;
use parser::Parser;
use std::cell::RefCell;

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
    
    /*
    //test comment 

    pub fn main() -> Int {      
        println("Hello! This is first program!!!");
        print("Enter the some string: ");
        print(readLine());
        ret 0;
    }
    */
    
    let input = r#"
        //test comment 
        
        fn testFunction(n: Int) -> Int {
            if ((n + 1) / 10) == 0 {
                ret 1;
            } else {
                ret 0;
            }
        }
        
        pub fn main() -> Int {      
            print("Input guess number: ");
            
            Int input = readInt();
            
            ret testFunction(input);
        }
    "#;

    let parse_result = Parser::generate_parsed_unit_from_input("test_unit", input);
    
    let translator = IttTreeBuilder::new();
    
    let mut unit = translator.translate(&parse_result);
    
    let functions_table = RefCell::new(FunctionSymbolTable::new());
    
    functions_table.borrow_mut().define(TableFunction {
        name: "print",
        args: vec![("arg0", IttType::String)],
        return_type: IttType::Int,
        visibility: IttVisibility::GLOBAL
    }).unwrap();
    
    functions_table.borrow_mut().define(TableFunction {
        name: "println",
        args: vec![("arg0", IttType::String)],
        return_type: IttType::Int,
        visibility: IttVisibility::GLOBAL
    }).unwrap();
    
    functions_table.borrow_mut().define(TableFunction {
        name: "readInt",
        args: vec![],
        return_type: IttType::Int,
        visibility: IttVisibility::GLOBAL
    }).unwrap();
    
    functions_table.borrow_mut().define(TableFunction {
        name: "readFloat",
        args: vec![],
        return_type: IttType::Float,
        visibility: IttVisibility::GLOBAL
    }).unwrap();
    
    functions_table.borrow_mut().define(TableFunction {
        name: "readLine",
        args: vec![],
        return_type: IttType::String,
        visibility: IttVisibility::GLOBAL
    }).unwrap();
    
    function_table_builder(&unit, &functions_table);
    
    let mut types_resolver = IttTreeTypeResolver::new(&functions_table);
    
    types_resolver.process_tree(&mut unit);
    
    let validator = IttTreeValidator::new(&functions_table, &unit);
    
    validator.validate_tree();
    
    test_gen(&unit, &functions_table.borrow());
}