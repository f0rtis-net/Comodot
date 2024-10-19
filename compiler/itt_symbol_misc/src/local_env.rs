use std::collections::HashMap;

pub struct LocalEnv<'input, T: Clone> {
    scopes: Vec<HashMap<&'input str, T>>
}

impl <'input, T: Clone> LocalEnv<'input, T> {
    pub fn new() -> Self {
        let mut global = Self {
            scopes: vec![]
        };
        
        global.push_scope();
        
        global
    }
    
    pub fn push_scope(&mut self) { self.scopes.push(HashMap::new()); }
    
    pub fn pop_scope(&mut self) { self.scopes.pop(); }
    
    pub fn define(&mut self, name: &'input str, value: T) -> Result<(), String> {
        
        if self.scopes.last_mut().unwrap().insert(name, value).is_some() {
            Err(String::from("Can not define symbol in enviroment"))
        } else {
            Ok(())
        }
    }
    
    pub fn lookup(&self, name: &'input str) -> Option<T> {

        for scope in self.scopes.iter().rev() {
            match scope.get(name) {
                Some(value) => return Some(value.clone()),
                None => continue
            }
        }
        
        None
    }
}