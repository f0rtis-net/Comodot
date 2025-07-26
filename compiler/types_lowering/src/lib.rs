use hir::{HirExpr, HirExprKind, HirFile, HirId, HirModuleItem};
use hir_resolver::NamesResolver;
use middle::{ty::{LangType, Primitive}, HirModuleTypeTable, TypeInfo};

fn update_type(ty_table: &mut HirModuleTypeTable, hir_id: HirId, ty: LangType) {
    ty_table.insert_type(hir_id, TypeInfo { ty, inferred: false });
}

fn translate_hint_to_type(hint: LangType) -> LangType {
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

fn infer_expr(names_ctx: &NamesResolver, ty_table: &mut HirModuleTypeTable, expr: &HirExpr) {
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
            for block_expr in block {
                infer_expr(names_ctx, ty_table, block_expr);
            }
            let last_ty = ty_table.get_type(&block.last().unwrap().id);
            ty_table.insert_type(expr.id, last_ty.unwrap().clone());
        }

        HirExprKind::Binary { op, lhs, rhs } => {
            infer_expr(names_ctx, ty_table, lhs);
            infer_expr(names_ctx, ty_table, rhs);

            let lhs_ty = ty_table.get_type(&lhs.id).unwrap();
            //let rhs_ty = ty_table.get_type(rhs.id).unwrap(); - late: todo inference type bounds

            ty_table.insert_type(expr.id, TypeInfo {
                ty: lhs_ty.ty.clone(),
                inferred: false
            });
        }

        HirExprKind::Return(Some(expr_ret)) => {
            infer_expr(names_ctx, ty_table, expr_ret);
            
            let ret_expr_ty = ty_table.get_type(&expr_ret.id).unwrap();
            ty_table.insert_type(expr.id, ret_expr_ty.clone());
        }

        HirExprKind::Return(None) => {}

        HirExprKind::Call { name, args} => {
            for arg in args {
                infer_expr(names_ctx, ty_table, arg);
            }
            let in_scope = names_ctx.find_match(&expr.id).unwrap();

            let in_scope_ty = ty_table.get_type(&in_scope.id).unwrap();
            ty_table.insert_type(expr.id, in_scope_ty.clone());
        }

        HirExprKind::If { cond, then, _else } => {
            infer_expr(names_ctx, ty_table, cond);
            infer_expr(names_ctx, ty_table, then);

            if _else.is_some() {
                infer_expr(names_ctx, ty_table, _else.as_ref().unwrap());
            }

            // late todo: inference type bounds

            let then_expr_ty = ty_table.get_type(&then.id).unwrap();
            ty_table.insert_type(expr.id, then_expr_ty.clone());
        }

        HirExprKind::VarDef { name, value } => {
            infer_expr(names_ctx, ty_table, value);

            ty_table.insert_type(expr.id, TypeInfo { ty: LangType::Primitives(Primitive::Unit), inferred: false });
        }

        HirExprKind::Id(id) => {
            let def_id = names_ctx.find_match(&expr.id).unwrap_or_else(|| panic!("Could not find definition for {}", id));

            ty_table.insert_type(expr.id, ty_table.get_type(&def_id.id).unwrap().clone());
        }

        _ => panic!("Not implemented: {:?}", expr.kind)
    }
}

pub fn hir_type_resolution(names_ctx: &NamesResolver, ty_table: &mut HirModuleTypeTable, files: &Vec<HirFile>) {
    for file in files {
        for declaration in &file.items {
            match declaration {
                HirModuleItem::Func { id, name, args, body, visibility } => {
                    let old_ty = ty_table.get_type(id).unwrap();
                    update_type(ty_table, *id, translate_hint_to_type(old_ty.ty.clone()));

                    for arg in args {
                        let arg_old_ty = ty_table.get_type(&arg.1).unwrap();
                        update_type(ty_table, arg.1, translate_hint_to_type(arg_old_ty.ty.clone()));
                    }

                    match &body.kind {
                        HirExprKind::Block(exprs) => {
                            for expr in exprs {
                                infer_expr(names_ctx, ty_table, expr);
                            }

                            let last_ty = ty_table.get_type(&exprs.last().unwrap().id);
                            ty_table.insert_type(body.id, last_ty.unwrap().clone());
                        },

                        _ => panic!("Unsupported body type for function")
                    }; 
                }
            }
        }
    }
}