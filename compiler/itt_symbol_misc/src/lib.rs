use std::{cell::RefCell};

use func_table::{FunctionSymbolTable, TableFunction};
use itt::{IttDefinitions, IttVisibility, TypedUnit};

pub mod func_table;
pub mod local_env;

pub fn function_table_builder<'input>(unit: &TypedUnit<'input>, table: &RefCell<FunctionSymbolTable<'input>>) {
    unit.unit_content.iter().for_each(|expr| {
        match expr {
            IttDefinitions::Function(func) => table.borrow_mut().define(TableFunction {
                name: func.name,
                args: func.args.clone(),
                return_type: func.return_type,
                visibility: func.visibility
            }).unwrap(),
            
            _ => ()
        };
    });
}