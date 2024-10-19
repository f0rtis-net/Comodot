use std::collections::HashMap;

use itt::{IttType, IttVisibility};

pub struct TableFunction<'input> {
    pub name: &'input str,
    pub args: Vec<(&'input str, IttType)>,
    pub return_type: IttType,
    pub visibility: IttVisibility
}

type ManglerFunction = Box<dyn Fn(&str, &Vec<IttType>) -> Result<String, String>>;

pub struct FunctionSymbolTable<'input> {
    symbols: HashMap<String, TableFunction<'input>>,
}

impl<'input> FunctionSymbolTable<'input> {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }
    
    pub fn type_to_char_translator(&self, _type: &IttType) -> Result<char, String> {
        
        match _type {
            IttType::Int => Ok('i'),
            IttType::Bool => Ok('b'),
            IttType::Char => Ok('c'),
            IttType::Custom => Ok('u'),
            IttType::Void => Ok('v'),
            IttType::Float => Ok('f'),
            IttType::String => Ok('s'),
            _ => Err(String::from("Invalid type to short"))
        }
    }
    
    pub fn function_table_name_mangler(&self, name: &str, arguments: &Vec<IttType>) -> Result<String, String> {

        let mut arg_prefix = String::new();
    
        for arg in arguments.iter() {
            arg_prefix.push('_');
            arg_prefix.push(self.type_to_char_translator(arg)?);
        }
        
        Ok(format!("{name}{arg_prefix}"))
    }
    
    pub fn lookup(&self, name: &'input str, arguments: &Vec<IttType>) -> Option<&TableFunction> {
        let masked_name = self.function_table_name_mangler(name, arguments).unwrap();
        
        if let Some(symbol) = self.symbols.get(&masked_name) {
            return Some(symbol);
        }
        
        None
    }
    
    pub fn define(&mut self, _fn: TableFunction<'input>) -> Result<(), String>{
        let arg_types = _fn.args.iter().map(|arg| {
            arg.1
        }).collect();
        
        let masked_name = self.function_table_name_mangler(_fn.name, &arg_types).unwrap();
        
        if self.symbols.insert(masked_name, _fn).is_some() {
            Err(String::from("Can not define symbol in table"))
        } else {
            Ok(())
        }
    }
}
