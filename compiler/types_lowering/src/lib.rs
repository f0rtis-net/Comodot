use std::{cell::RefCell, collections::HashMap, rc::Rc};

use hir::{HirExpr, HirExprKind, HirFile, HirId, HirModuleItem};
use middle::{ty::{LangType, Primitive}, HirModuleTypeTable, TypeInfo};

#[derive(Debug)]
pub struct Scope {
    parent: Option<Rc<RefCell<Scope>>>,
    definitions: HashMap<String, HirId>,
}

impl Scope {
    pub fn new(parent: Option<Rc<RefCell<Scope>>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            parent,
            definitions: HashMap::new(),
        }))
    }

    pub fn add_definition(&mut self, name: String, hir_id: HirId) {
        self.definitions.insert(name, hir_id);
    }

    pub fn lookup(&self, name: &str) -> Option<HirId> {
        self.definitions.get(name).copied().or_else(|| {
            self.parent.as_ref().and_then(|parent| parent.borrow().lookup(name))
        })
    }
}

pub struct TypeResolver {
    top_scope: Rc<RefCell<Scope>>
}

impl TypeResolver {
    pub fn new() -> Self {
        Self { top_scope: Scope::new(None) }
    }

    fn translate_hint_to_type(&self, hint: LangType) -> LangType {
        match hint {
            LangType::HINT(hint) => {
                match hint.as_str() {
                    "Int" => LangType::Primitives(Primitive::Int),
                    "Float" => LangType::Primitives(Primitive::Float),
                    "Char" => LangType::Primitives(Primitive::Char),
                    "Bool" => LangType::Primitives(Primitive::Bool),
                    _ => LangType::UNRESOLVED    
                }
            },
            _ => LangType::UNRESOLVED,
        }
    }

    fn infer_expr(&mut self, ty_table: &mut HirModuleTypeTable, expr: &HirExpr) {
        match &expr.kind {
            HirExprKind::Bool(_) => ty_table.insert_type(expr.id, TypeInfo {
                ty: LangType::Primitives(middle::ty::Primitive::Bool),
                inferred: false
            }),

            HirExprKind::Int(_) => ty_table.insert_type(expr.id, TypeInfo {
                ty: LangType::Primitives(middle::ty::Primitive::Int),
                inferred: false
            }),

            HirExprKind::Float(_) => ty_table.insert_type(expr.id, TypeInfo {
                ty: LangType::Primitives(middle::ty::Primitive::Float),
                inferred: false
            }),

            HirExprKind::Char(_) => ty_table.insert_type(expr.id, TypeInfo {
                ty: LangType::Primitives(middle::ty::Primitive::Char),
                inferred: false
            }),

            HirExprKind::Block(block) => {
                self.open_scope();
                for block_expr in block {
                    self.infer_expr(ty_table, block_expr);
                }
                let last_ty = ty_table.get_type(block.last().unwrap().id);
                ty_table.insert_type(expr.id, last_ty.unwrap().clone());
                self.close_scope();
            }

            HirExprKind::Binary { op, lhs, rhs } => {
                self.infer_expr(ty_table, lhs);
                self.infer_expr(ty_table, rhs);

                let lhs_ty = ty_table.get_type(lhs.id).unwrap();
                //let rhs_ty = ty_table.get_type(rhs.id).unwrap(); - late: todo inference type bounds

                ty_table.insert_type(expr.id, TypeInfo {
                    ty: lhs_ty.ty.clone(),
                    inferred: false
                });
            }

            HirExprKind::Return(Some(expr_ret)) => {
                self.infer_expr(ty_table, expr_ret);
                
                let ret_expr_ty = ty_table.get_type(expr_ret.id).unwrap();
                ty_table.insert_type(expr.id, ret_expr_ty.clone());
            }

            HirExprKind::Return(None) => {}

            HirExprKind::Call { name, args } => {
                for arg in args {
                    self.infer_expr(ty_table, arg);
                }

                let in_scope = self.top_scope.borrow().lookup(name).unwrap();
                let in_scope_ty = ty_table.get_type(in_scope).unwrap();
                ty_table.insert_type(expr.id, in_scope_ty.clone());
            }

            HirExprKind::If { cond, then, _else } => {
                self.infer_expr(ty_table, cond);
                self.infer_expr(ty_table, then);

                if _else.is_some() {
                    self.infer_expr(ty_table, _else.as_ref().unwrap());
                }

                // late todo: inference type bounds

                let then_expr_ty = ty_table.get_type(then.id).unwrap();
                ty_table.insert_type(expr.id, then_expr_ty.clone());
            }

            HirExprKind::VarDef { name, value } => {
                self.infer_expr(ty_table, value);

                self.top_scope.borrow_mut().add_definition(name.to_string(), value.id);

                ty_table.insert_type(expr.id, TypeInfo { ty: LangType::Primitives(Primitive::Unit), inferred: false });
            }

            HirExprKind::Id(id) => {
                let def_id = self.top_scope.borrow().lookup(id).unwrap_or_else(|| panic!("Could not find definition for {}", id));

                ty_table.insert_type(expr.id, ty_table.get_type(def_id).unwrap().clone());
            }

            _ => panic!("Not implemented: {:?}", expr.kind)
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


    pub fn hir_type_resolution(&mut self, ty_table: &mut HirModuleTypeTable, files: &Vec<HirFile>) {
        for file in files {
            for declaration in &file.items {
                match declaration {
                    HirModuleItem::Func { id, name, .. } => {
                        self.top_scope.borrow_mut().add_definition(name.to_string(), *id);
                    }
                }
            }
        }


        for file in files {
            for declaration in &file.items {
                match declaration {
                    HirModuleItem::Func { id, name, args, body, visibility } => {
                        
                        self.infer_expr(ty_table, body);
                    }
                }
            }
        }

        ty_table.dump();
    }
}