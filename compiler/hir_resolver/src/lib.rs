use std::{collections::HashMap};

use hir::{HirExpr, HirExprKind, HirModuleItem};
use middle::{GlobalCtx, SymbolInfo};

struct Env<'a> {
    scopes: Vec<HashMap<&'a str, SymbolInfo>>
}

impl <'a> Env<'a> {
    pub fn new() -> Self {
        let mut global = Self {
            scopes: vec![]
        };
        
        global.push_scope();
        
        global
    }
    
    pub fn push_scope(&mut self) { self.scopes.push(HashMap::new()); }
    
    pub fn pop_scope(&mut self) { self.scopes.pop(); }
    
    pub fn define(&mut self, name: &'a str, value: SymbolInfo) -> Result<(), String> {
        
        if self.scopes.last_mut().unwrap().insert(name, value).is_some() {
            Err(String::from("Can not define symbol in enviroment"))
        } else {
            Ok(())
        }
    }
    
    pub fn lookup(&self, name: &'a str) -> Option<SymbolInfo> {

        for scope in self.scopes.iter().rev() {
            match scope.get(name) {
                Some(value) => return Some(value.clone()),
                None => continue
            }
        }
        
        None
    }
}

fn track_global_names<'a>(env: &mut Env<'a>, ctx: &GlobalCtx<'a>) {
    for file in &ctx.module_files {
        for item in &file.items {
            match item {
                HirModuleItem::Func { id, name, .. } => {
                    env.define(name, SymbolInfo { 
                        id: id.clone(), 
                        is_external_name: false 
                    }).unwrap();
                },
            }
        }
    }
}

fn link_local_names<'a>(env: &mut Env<'a>, ctx: &GlobalCtx<'a>, expr: &HirExpr<'a>) {
    match &expr.kind {
        HirExprKind::Block(block) => {
            for expr in block {
                env.push_scope();
                link_local_names(env, ctx, expr);
                env.pop_scope();
            }
        }

        HirExprKind::Binary { op, lhs, rhs } => {
            link_local_names(env, ctx, lhs);
            link_local_names(env, ctx, rhs);
        }

        HirExprKind::Return(Some(ret_expr)) => {
            link_local_names(env, ctx, ret_expr);
        }

        HirExprKind::If { cond, then, _else } => {
            link_local_names(env, ctx, cond);
            link_local_names(env, ctx, then);
           
            if _else.is_some() {
                link_local_names(env, ctx, _else.as_ref().unwrap());
            }
        }

        HirExprKind::Id(id) => {
            if let Some(symbol) = env.lookup(id) {
                ctx.module_symbols.borrow_mut().add_pair(expr.id, SymbolInfo {
                    id: symbol.id,
                    is_external_name: symbol.is_external_name
                });
            }
        }

        HirExprKind::Call { name, args } => {
            //todo find in top scopes
            for arg in args {
                link_local_names(env, ctx, arg);
            }

            //temporally, next step - find in imports
            if let Some(symbol) = env.lookup(name) {
                ctx.module_symbols.borrow_mut().add_pair(expr.id, SymbolInfo {
                    id: symbol.id,
                    is_external_name: symbol.is_external_name 
                });
            }
        }

        _ => ()
    }
}


fn try_to_resolve_locals<'a>(env: &mut Env<'a>, ctx: &GlobalCtx<'a>) {
    for file in &ctx.module_files {
        for item in &file.items {
            match item {
                HirModuleItem::Func { name, id, args, body, .. } => {
                    env.push_scope();

                    for arg in args {
                        env.define(arg.0, SymbolInfo { 
                            id: id.clone(), 
                            is_external_name: false 
                        }).unwrap();
                    }

                    link_local_names(env, ctx,body);

                    env.pop_scope();
                }
            }
        }
    }
}

pub fn resolve_module<'a>(ctx: &mut GlobalCtx<'a>) {
    let mut env = Env::new();

    track_global_names(&mut env, ctx);

    try_to_resolve_locals(&mut env, ctx);
}