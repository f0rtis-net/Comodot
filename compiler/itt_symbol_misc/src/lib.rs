use std::cell::RefCell;

use func_table::{FunctionSymbolTable, TableFunction};
use mir::{IttDefinitions, TypedUnit};

pub mod func_table;
pub mod local_env;
pub mod name_mangler;

pub fn function_table_builder<'input>(unit: &TypedUnit<'input>, table: &RefCell<FunctionSymbolTable<'input>>) {
    unit.unit_content.iter().for_each(|expr| {
        match expr {
            IttDefinitions::Function(func) => table.borrow_mut().define(&unit.unit_name, TableFunction {
                name: func.name,
                args: func.args.clone(),
                return_type: func.return_type,
                is_extern: false
            }).unwrap(),
            
            IttDefinitions::Extern(ext_fn) => table.borrow_mut().define(&unit.unit_name, TableFunction {
                name: ext_fn.name,
                args: ext_fn.args.clone(),
                return_type: ext_fn.return_type,
                is_extern: true
            }).unwrap(),
            
            _ => ()
        };
    });
}