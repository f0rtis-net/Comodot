use hir::{HirExpr, HirExprKind, HirModuleItem};
use middle::GlobalCtx;

pub fn check_inner_expressions<'a>(ctx: &GlobalCtx<'a>, expr: &HirExpr<'a>) {
    match &expr.kind {
        hir::HirExprKind::VarDef { name, value , ty: _} => {
            let val_ty = ctx.module_ty_info.borrow().get_type(&value.id).unwrap().ty.clone();
            
            let val_expr_ty = ctx.module_ty_info.borrow().get_type(&expr.id).unwrap().ty.clone();

            if val_ty != val_expr_ty {
                panic!("variable {} has type {:?} but r-value has type {:?}", name, val_expr_ty, val_ty);
            }
        }

        HirExprKind::Binary { op: _, lhs, rhs } => {
            check_inner_expressions(ctx, lhs);
            check_inner_expressions(ctx, rhs);

            let lhs_ty = ctx.module_ty_info.borrow().get_type(&lhs.id).unwrap().ty.clone();
            let rhs_ty = ctx.module_ty_info.borrow().get_type(&rhs.id).unwrap().ty.clone();

            if lhs_ty != rhs_ty {
                panic!("In binary operation left hand side has type {:?} but right hand side has type {:?}", lhs_ty, rhs_ty);
            }
        }

        HirExprKind::Call { name: _, args } => {
            for arg in args {
                check_inner_expressions(ctx, arg);
            }
            //to do - create fn type, for checking match of fn type with call args
        }

        HirExprKind::If { cond, then, _else } => {
            check_inner_expressions(ctx, cond);
            check_inner_expressions(ctx, then);

            if _else.is_none() {
                return;
            }

            check_inner_expressions(ctx, _else.as_ref().unwrap());
        }

        HirExprKind::Block(exprs) => {
            for expr in exprs {
                check_inner_expressions(ctx, expr);
            }
        }

        _ => ()
    }
}

pub fn validate_hir<'a>(ctx: &GlobalCtx<'a>) {
    for file in ctx.module_files.iter() {
        for decl in &file.items {
            match decl {
                HirModuleItem::Func { id, name, args: _, body, visibility: _, ret_ty: _ } => {
                    let fn_type = ctx.module_ty_info.borrow().get_type(&id).unwrap().clone();

                    let body_type = ctx.module_ty_info.borrow().get_type(&body.id).unwrap().clone();

                    if fn_type.ty != body_type.ty {
                        panic!("function {} has type {:?} but body has type {:?}", name, fn_type.ty, body_type.ty);
                    }

                    check_inner_expressions(ctx, body);
                }
            }
        }
    }
}