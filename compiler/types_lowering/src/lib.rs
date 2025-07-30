use std::cell::RefCell;

use hir::{HirExpr, HirExprKind, HirId, HirModuleItem};
use middle::{ty::{LangType, Primitive}, GlobalCtx, HirModuleTypeTable, TypeInfo};

fn update_type(ty_table: &RefCell<HirModuleTypeTable>, hir_id: HirId, ty: LangType) {
    ty_table.borrow_mut().insert_type(hir_id, TypeInfo { ty, inferred: false });
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

fn infer_expr<'a>(ctx: &GlobalCtx<'a>, expr: &HirExpr) {
    match &expr.kind {
        HirExprKind::Bool(_) => ctx.module_ty_info.borrow_mut().insert_type(expr.id, TypeInfo {
            ty: LangType::Primitives(middle::ty::Primitive::Bool),
            inferred: false
        }),

        HirExprKind::Int(_) => ctx.module_ty_info.borrow_mut().insert_type(expr.id, TypeInfo {
            ty: LangType::Primitives(middle::ty::Primitive::Int),
            inferred: false
        }),

        HirExprKind::Float(_) => ctx.module_ty_info.borrow_mut().insert_type(expr.id, TypeInfo {
            ty: LangType::Primitives(middle::ty::Primitive::Float),
            inferred: false
        }),

        HirExprKind::Char(_) => ctx.module_ty_info.borrow_mut().insert_type(expr.id, TypeInfo {
            ty: LangType::Primitives(middle::ty::Primitive::Char),
            inferred: false
        }),

        HirExprKind::Block(block) => {
            for block_expr in block {
                infer_expr(ctx, block_expr);
            }

            let last_ty = ctx.module_ty_info.borrow_mut().get_type(&block.last().unwrap().id).cloned();
            ctx.module_ty_info.borrow_mut().insert_type(expr.id, last_ty.unwrap().clone());
        }

        HirExprKind::Binary { op: _, lhs, rhs } => {
            infer_expr(ctx, lhs);
            infer_expr(ctx, rhs);

            let lhs_ty = ctx.module_ty_info.borrow_mut().get_type(&lhs.id).unwrap().clone();
            //let rhs_ty = ty_table.get_type(rhs.id).unwrap(); - late: todo inference type bounds

            ctx.module_ty_info.borrow_mut().insert_type(expr.id, TypeInfo {
                ty: lhs_ty.ty.clone(),
                inferred: false
            });
        }

        HirExprKind::Return(Some(expr_ret)) => {
            infer_expr(ctx, expr_ret);
            
            let ret_expr_ty = ctx.module_ty_info.borrow_mut().get_type(&expr_ret.id).unwrap().clone();
            ctx.module_ty_info.borrow_mut().insert_type(expr.id, ret_expr_ty.clone());
        }

        HirExprKind::Return(None) => {}

        HirExprKind::Call { name, args} => {
            for arg in args {
                infer_expr(ctx, arg);
            }

            let in_scope = ctx.module_symbols.borrow().get_pair(&expr.id).unwrap().clone();
            
            let in_scope_ty = ctx.module_ty_info.borrow_mut().get_type(&in_scope.id).unwrap().clone();
            ctx.module_ty_info.borrow_mut().insert_type(expr.id, in_scope_ty.clone());
        }

        HirExprKind::If { cond, then, _else } => {
            infer_expr(ctx, cond);
            infer_expr(ctx, then);

            if _else.is_some() {
                infer_expr(ctx, _else.as_ref().unwrap());
            }

            // late todo: inference type bounds

            let then_expr_ty = ctx.module_ty_info.borrow_mut().get_type(&then.id).unwrap().clone();
            ctx.module_ty_info.borrow_mut().insert_type(expr.id, then_expr_ty.clone());
        }

        HirExprKind::VarDef { name: _, value } => {
            infer_expr(ctx, value);

            ctx.module_ty_info.borrow_mut().insert_type(expr.id, TypeInfo { ty: LangType::Primitives(Primitive::Unit), inferred: false });
        }

        HirExprKind::Id(id) => {
            let def_id = ctx.module_symbols.borrow().get_pair(&expr.id).unwrap_or_else(|| panic!("Could not find definition for {}", id)).clone();

            let old_ty = ctx.module_ty_info.borrow_mut().get_type(&def_id.id).unwrap().clone();
            ctx.module_ty_info.borrow_mut().insert_type(expr.id, old_ty);
        }

        _ => panic!("Not implemented: {:?}", expr.kind)
    }
}

pub fn type_hir_module(ctx: &mut GlobalCtx) {
    for file in &ctx.module_files {
        for elem in file.items.iter() {
            match elem {
                HirModuleItem::Func { id, name: _, args, body, .. } => {
                    let old_ty = ctx.module_ty_info.borrow_mut().get_type(id).unwrap().clone();
                    update_type(&ctx.module_ty_info, *id, translate_hint_to_type(old_ty.ty.clone()));

                    for arg in args {
                        let arg_old_ty = ctx.module_ty_info.borrow_mut().get_type(&arg.1).unwrap().clone();
                        update_type(&ctx.module_ty_info, arg.1, translate_hint_to_type(arg_old_ty.ty.clone()));
                    } 
                }
            }
        }
    }

    for file in &ctx.module_files {
        for elem in file.items.iter() {
            match elem {
                HirModuleItem::Func { id: _, name: _, args: _, body, .. } => {
                    match &body.kind {
                        HirExprKind::Block(exprs) => {
                            for expr in exprs {
                                infer_expr(ctx, expr);
                            }

                            let last_ty =  ctx.module_ty_info.borrow_mut().get_type(&exprs.last().unwrap().id).cloned();
                            ctx.module_ty_info.borrow_mut().insert_type(body.id, last_ty.unwrap().clone());
                        },

                        _ => panic!("Unsupported body type for function")
                    }; 
                }
            }
        }
    }

    //ctx.module_ty_info.borrow_mut().dump();
}