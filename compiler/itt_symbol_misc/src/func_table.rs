use std::collections::HashMap;

use mir::IttType;

use crate::name_mangler::mangle_function_name;

pub struct TableFunction<'input> {
    pub name: &'input str,
    pub args: Vec<(&'input str, IttType)>,
    pub return_type: IttType,
    pub is_extern: bool
}

pub struct FunctionSymbolTable<'input> {
    symbols: HashMap<String, TableFunction<'input>>,
}

impl<'input> FunctionSymbolTable<'input> {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }
    
    pub fn lookup(&self, module_name: &str, name: &'input str, arguments: &Vec<IttType>) -> Option<&TableFunction> {
        let masked_name = mangle_function_name(module_name, name, arguments).unwrap();
        
        if let Some(symbol) = self.symbols.get(&masked_name) {
            return Some(symbol);
        }
        
        None
    }
    
    pub fn define(&mut self, module_name: &str, _fn: TableFunction<'input>) -> Result<(), String>{
        let arg_types = _fn.args.iter().map(|arg| {
            arg.1
        }).collect();
        
        let masked_name = mangle_function_name(module_name, _fn.name, &arg_types).unwrap();
        
        if self.symbols.insert(masked_name, _fn).is_some() {
            Err(String::from("Can not define symbol in table"))
        } else {
            Ok(())
        }
    }
}
