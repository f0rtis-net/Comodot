use std::collections::HashMap;
use std::cell::RefCell;
use itt::{IttType, IttVisibility};

use crate::name_mangler::mangle_function_name;

#[derive(Debug)]
pub struct TableFunction<'input> {
    pub name: String,
    pub args: Vec<(&'input str, IttType)>,
    pub return_type: IttType,
    pub visibility: IttVisibility
}

#[derive(Debug)]
pub struct GlobalFunctionSymbolTable<'input> {
    modules: HashMap<String, RefCell<FunctionSymbolTable<'input>>>
}

impl<'input> GlobalFunctionSymbolTable<'input> {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new()
        }
    }
    
    pub fn define_module(&mut self, name: &str) -> Result<&RefCell<FunctionSymbolTable<'input>>, String> {
        self.modules.insert(name.to_string(), RefCell::new(FunctionSymbolTable::new()));
        Ok(self.lookup_module(name).unwrap())
    }
    
    pub fn lookup_module(&self, name: &str) -> Option<&RefCell<FunctionSymbolTable<'input>>> {
        self.modules.get(name)
    }
}

#[derive(Debug)]
pub struct FunctionSymbolTable<'input> {
    symbols: HashMap<String, TableFunction<'input>>,
}

impl<'input> FunctionSymbolTable<'input> {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }
    
    pub fn lookup(&self, module_name: &str, name: &'input str, args: &Vec<IttType>) -> Option<&TableFunction> {
        let mangled_name = mangle_function_name(module_name, name, args).unwrap();
        
        if let Some(symbol) = self.symbols.get(mangled_name.as_str()) {
            return Some(symbol);
        }
        
        None
    }
    
    pub fn define(&mut self, module_name: &str, _fn: TableFunction<'input>) -> Result<(), String> {
        let translated_args = _fn.args.iter().map(|arg| arg.1).collect();
        let mangled_name = mangle_function_name(module_name, &_fn.name, &translated_args).unwrap();
        
        if self.symbols.insert(mangled_name, _fn).is_some() {
            Err(String::from("Can not define symbol in table"))
        } else {
            Ok(())
        }
    }
}
