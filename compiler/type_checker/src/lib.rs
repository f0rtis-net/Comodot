use hir::HirModuleItem;
use middle::GlobalCtx;

//todo - type checking of variables, conditions, binary operations. Part of ty checking of bin ops need be placed here - type unification

pub fn validate_hir<'a>(ctx: &GlobalCtx<'a>) {
    for file in ctx.module_files.iter() {
        for decl in &file.items {
            match decl {
                HirModuleItem::Func { id, name, args, body, visibility } => {
                    let fn_type = ctx.module_ty_info.borrow().get_type(&id).unwrap().clone();

                    let body_type = ctx.module_ty_info.borrow().get_type(&body.id).unwrap().clone();

                    if fn_type.ty != body_type.ty {
                        panic!("function {} has type {:?} but body has type {:?}", name, fn_type.ty, body_type.ty);
                    }
                }
            }
        }
    }
}