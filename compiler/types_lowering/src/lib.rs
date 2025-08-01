use hir::{HirExpr, HirExprKind, HirModuleItem, HirTyHint};
use middle::{ty::{LangType, Primitive}, GlobalCtx, TypeInfo};

fn translate_hint_to_type<'a>(hint: &HirTyHint<'a>) -> LangType {
    match hint {
        HirTyHint::Primitive(hint) => {
            match *hint {
                "Bool" => LangType::Primitives(Primitive::Bool),
                "Int" => LangType::Primitives(Primitive::Int),
                "Float" => LangType::Primitives(Primitive::Float),
                "Char" => LangType::Primitives(Primitive::Char),
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
        }),

        HirExprKind::Int(_) => ctx.module_ty_info.borrow_mut().insert_type(expr.id, TypeInfo {
            ty: LangType::Primitives(middle::ty::Primitive::Int),
        }),

        HirExprKind::Float(_) => ctx.module_ty_info.borrow_mut().insert_type(expr.id, TypeInfo {
            ty: LangType::Primitives(middle::ty::Primitive::Float),
        }),

        HirExprKind::Char(_) => ctx.module_ty_info.borrow_mut().insert_type(expr.id, TypeInfo {
            ty: LangType::Primitives(middle::ty::Primitive::Char),
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
            });
        }

        HirExprKind::Return(Some(expr_ret)) => {
            infer_expr(ctx, expr_ret);
            
            let ret_expr_ty = ctx.module_ty_info.borrow_mut().get_type(&expr_ret.id).unwrap().clone();
            ctx.module_ty_info.borrow_mut().insert_type(expr.id, ret_expr_ty.clone());
        }

        HirExprKind::Return(None) => {}

        HirExprKind::Call { name: _, args} => {
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

        HirExprKind::VarDef { name: _, value, ty} => {
            infer_expr(ctx, value);

            if ty.is_some() {
                let conv_ty = translate_hint_to_type(&ty.clone().unwrap()).clone();
                ctx.module_ty_info.borrow_mut().insert_type(expr.id, TypeInfo { ty: conv_ty });
                return;
            }

            let content_type = ctx.module_ty_info.borrow_mut().get_type(&value.id).unwrap().clone();
            ctx.module_ty_info.borrow_mut().insert_type(expr.id, content_type.clone());
        }

        HirExprKind::Id(id) => {
            let def_id = ctx.module_symbols.borrow().get_pair(&expr.id).unwrap_or_else(|| panic!("Could not find definition for {}", id)).clone();

            let old_ty = ctx.module_ty_info.borrow_mut().get_type(&def_id.id).unwrap().clone();
            ctx.module_ty_info.borrow_mut().insert_type(expr.id, old_ty);
        }
    }
}

pub fn type_hir_module(ctx: &mut GlobalCtx) {
    for file in &ctx.module_files {
        for elem in file.items.iter() {
            match elem {
                HirModuleItem::Func { id, name: _, args, ret_ty, .. } => {
                    let conv_ty = if ret_ty.is_some() {
                        translate_hint_to_type(&ret_ty.clone().unwrap()).clone() 
                    } else {
                        LangType::Primitives(middle::ty::Primitive::Unit)
                    };

                    ctx.module_ty_info.borrow_mut().insert_type(id.clone(), TypeInfo { ty: conv_ty });

                    for arg in args {
                        let arg_ty = translate_hint_to_type(&arg.2.clone()).clone();
                        ctx.module_ty_info.borrow_mut().insert_type(arg.1.clone(), TypeInfo { ty: arg_ty });
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