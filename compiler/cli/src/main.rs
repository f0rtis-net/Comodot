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

    let input = r#"
        //test comment 
        
        func test(n: Int) > Int {
            if n == 40 {
                Int hui = 10;
                
                ret hui - 6.0;
            } else if n == 20 {
                ret 2;
            } else {
                if n == 0 {
                    ret 1;
                } else {
                     ret 0;
                };
            };
        }
        
        pub func main() > Int {      
            printf("loh");
            ret test(0);
        }
    "#;

    let parse_result = Parser::generate_parsed_unit_from_input("test_unit", input);
    
    let translator = IttTreeBuilder { name: "test" };
    
    let mut unit = translator.translate(&parse_result);
    
    let functions_table = RefCell::new(FunctionSymbolTable::new());
    
    functions_table.borrow_mut().define(TableFunction {
        name: "printf",
        args: vec![("arg0", IttType::String)],
        return_type: IttType::Int,
        visibility: IttVisibility::GLOBAL
    }).unwrap();
    
    function_table_builder(&unit, &functions_table);
    
    let mut types_resolver = IttTreeTypeResolver::new(&functions_table);
    
    types_resolver.process_tree(&mut unit);
    
    let validator = IttTreeValidator::new(&functions_table, &unit);
    
    validator.validate_tree();
    
    test_gen(&unit, &functions_table.borrow());
}