use func_table::{GlobalFunctionSymbolTable, TableFunction};
use itt::{IttDefinitions, IttVisibility, TypedUnit};

pub mod func_table;
pub mod local_env;
pub mod name_mangler;

pub fn function_table_builder<'input>(unit: &TypedUnit<'input>, global_table: &mut GlobalFunctionSymbolTable<'input>) {
    let unit_table = global_table.define_module(&unit.unit_name).unwrap();

    unit.unit_content.iter().for_each(|expr| {
        match expr {
            IttDefinitions::Function(func) => {
            
                unit_table.borrow_mut().define(&unit.unit_name, TableFunction {
                    name: func.name.clone(),
                    args: func.args.clone(),
                    return_type: func.return_type,
                    visibility: func.visibility
                }).unwrap()
            },
            
            IttDefinitions::Extern(ext_fn) => unit_table.borrow_mut().define(&unit.unit_name, TableFunction {
                name: ext_fn.name.to_string(),
                args: ext_fn.args.clone(),
                return_type: ext_fn.return_type,
                visibility: IttVisibility::EXTERN
            }).unwrap(),
        };
    });
}