use crate::{IttFunction, IttVariable};
use std::collections::HashMap;

#[derive(Clone)]
pub enum SymbolTableTypes {
    Function(IttFunction),
    Variable(IttVariable),
}

#[derive(Clone)]
pub enum ScopeType {
    Global,
    Local(String),
}

#[derive(Clone)]
pub struct ModuleSymbolTable {
    scopes: Vec<HashMap<String, SymbolTableTypes>>,
    enclosure_index: usize,
}

impl ModuleSymbolTable {
    pub fn new() -> Self {
        let mut scope = ModuleSymbolTable {
            scopes: Vec::new(),
            enclosure_index: 0,
        };

        //create global module scope
        scope.declare_scope();

        scope
    }

    pub fn declare_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn undeclare_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn add_to_current_scope(&mut self, marker: String, symbol_def: SymbolTableTypes) {
        if self
            .scopes
            .last_mut()
            .unwrap()
            .insert(marker, symbol_def)
            .is_some()
        {
            panic!("Already exists symbol in this scope")
        }
    }

    pub fn enter_to_scope(&mut self) {
        self.enclosure_index += 1;
    }

    pub fn exit_from_scope(&mut self) {
        self.enclosure_index -= 1;
    }

    pub fn try_to_find_in_scopes(&mut self, name: &str) -> Option<&SymbolTableTypes> {
        for level in 0..=self.enclosure_index {
            for scope in self.scopes.get(level) {
                match scope.get(name) {
                    Some(value) => return Some(value),
                    None => continue,
                };
            }
        }

        None
    }

    pub fn try_to_find_in_global_scope(&mut self, name: &str) -> &SymbolTableTypes {
        if let Some(symbol) = self.scopes.first_mut().unwrap().get(name) {
            return symbol;
        }

        panic!("Symbol not found")
    }

    pub fn get_scopes_count(&self) -> usize {
        self.scopes.len()
    }

    pub fn get_current_enclosure_index(&self) -> usize {
        self.enclosure_index
    }
}

#[derive(Clone)]
pub struct GlobalSymbolTable {
    modules: HashMap<String, ModuleSymbolTable>,
}

impl GlobalSymbolTable {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }

    pub fn define_module(&mut self, name: String) {
        self.modules.insert(name, ModuleSymbolTable::new());
    }

    pub fn try_to_find_in_module_globals(
        &mut self,
        module_name: &str,
        symbol_name: &str,
    ) -> &SymbolTableTypes {
        if let Some(module) = self.modules.get_mut(module_name) {
            return module.try_to_find_in_global_scope(symbol_name);
        }

        panic!("Symbol not found")
    }

    pub fn get_module_table(&mut self, module_name: &str) -> &mut ModuleSymbolTable {
        if let Some(module) = self.modules.get_mut(module_name) {
            return module;
        }

        panic!("module not found")
    }
}
