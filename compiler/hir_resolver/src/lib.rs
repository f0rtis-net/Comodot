use std::{cell::RefCell, collections::HashMap, rc::Rc};

use hir::{HirExpr, HirExprKind, HirFile, HirId, HirModuleItem};

#[derive(Debug, Clone, Copy)]
pub struct SymbolInfo {
    pub id: HirId,
    pub is_external_name: bool
}

#[derive(Debug)]
struct Scope {
    parent: Option<Rc<RefCell<Scope>>>,
    definitions: HashMap<String, SymbolInfo>,
}

impl Scope {
    pub fn new(parent: Option<Rc<RefCell<Scope>>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            parent,
            definitions: HashMap::new(),
        }))
    }

    pub fn add_definition(&mut self, name: String, hir_id: SymbolInfo) {
        self.definitions.insert(name, hir_id);
    }

    pub fn lookup(&self, name: &str) -> Option<SymbolInfo> {
        self.definitions.get(name).copied().or_else(|| {
            self.parent.as_ref().and_then(|parent| parent.borrow().lookup(name))
        })
    }
}

pub struct NamesResolver {
    nominal_matches: HashMap<HirId, SymbolInfo>,
    top_scope: Rc<RefCell<Scope>>
}

impl NamesResolver {
    pub fn new() -> Self {
        Self {
            nominal_matches: HashMap::new(),
            top_scope: Scope::new(None),
        }
    }

    fn open_scope(&mut self) {
        let new_scope = Scope::new(Some(self.top_scope.clone()));

        self.top_scope = new_scope;
    }

    fn close_scope(&mut self) {
        let parent = self.top_scope.borrow().parent.clone();
        self.top_scope = parent.expect("No parent scope");
    }

    fn find_local_names(&mut self, expr: &HirExpr) {
        match &expr.kind {
            HirExprKind::Block(block) => {
                for expr in block {
                    self.open_scope();
                    self.find_local_names(expr);
                    self.close_scope();
                }
            }

            HirExprKind::Binary { op, lhs, rhs } => {
                self.find_local_names(lhs);
                self.find_local_names(rhs);
            }

            HirExprKind::Return(Some(ret_expr)) => {
                self.find_local_names(ret_expr);
            }

            HirExprKind::If { cond, then, _else } => {
                self.find_local_names(&cond);
                self.find_local_names(then);
                if _else.is_some() {
                    self.find_local_names(_else.as_ref().unwrap());
                }
            }

            HirExprKind::Id(id) => {
                if let Some(symbol) = self.top_scope.borrow().lookup(id) {
                    self.nominal_matches.insert(expr.id, SymbolInfo {
                        id: symbol.id,
                        is_external_name: symbol.is_external_name
                    });
                }
            }

            HirExprKind::Call { name, args } => {
                //todo find in top scopes
                for arg in args {
                    self.find_local_names(arg);
                }

                //temporally, next step - find in imports
                if let Some(symbol) = self.top_scope.borrow().lookup(name) {
                    self.nominal_matches.insert(expr.id, SymbolInfo {
                        id: symbol.id,
                        is_external_name: symbol.is_external_name 
                    });
                }
            }

            _ => ()
        }
    }

    fn try_to_link_local_names(&mut self, hir: &Vec<HirFile>) { 
        for file in hir {
            for item in &file.items {
                match item {
                    HirModuleItem::Func { name, id, args, body, .. } => {
                        self.open_scope();
                        for arg in args {
                            self.top_scope.borrow_mut().add_definition(arg.0.to_string(), SymbolInfo { 
                                id: id.clone(), 
                                is_external_name: false 
                            });
                        }

                        self.find_local_names(body);
                        self.close_scope();
                    },
                }
            }
        }
    }

    fn track_global_names(&self, hir: &Vec<HirFile>) {
        for file in hir {
            for item in &file.items {
                match item {
                    HirModuleItem::Func { id, name, .. } => self.top_scope.borrow_mut().add_definition(name.to_string(), SymbolInfo { 
                        id: id.clone(), 
                        is_external_name: false 
                    }),
                }
            }
        }
    }

    pub fn resolve_names(&mut self, mod_files: &Vec<HirFile>) {
        //open global scope
        self.open_scope();

        //collect top level definitions
        self.track_global_names(mod_files);

        self.try_to_link_local_names(mod_files);

        self.close_scope();
    }


    pub fn find_match(&self, child: &HirId) -> Option<&SymbolInfo> {
        self.nominal_matches.get(child)
    }
}