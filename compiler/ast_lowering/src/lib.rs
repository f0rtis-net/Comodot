use core::panic;

use ast::{AstDefinitions, AstExpr, ExprTy, ParsedFile};
use hir::{HirBinOps, HirExpr, HirExprKind, HirFile, HirId, HirModuleItem, HirTyHint, HirVisibility};
use tokens::Token;

fn remap_visibility(visibility: &Token) -> HirVisibility {
    match visibility {
        Token::PUBLIC => HirVisibility::Public,
        Token::PRIVATE => HirVisibility::Private,
        _ => panic!("Unsupported visibility token")
    }
}

fn remap_bin_op(bin_op: &Token) -> HirBinOps {
    match bin_op {
        Token::PLUS => HirBinOps::SUM,
        Token::MINUS => HirBinOps::SUB,
        Token::STAR => HirBinOps::MUL,
        Token::SLASH => HirBinOps::DIV,
        Token::AND => HirBinOps::AND,
        Token::OR => HirBinOps::OR,
        Token::LT => HirBinOps::LT,
        Token::GT => HirBinOps::GT,
        Token::EQ => HirBinOps::EQ,
        _ => panic!("invalid binary operation")
    }
}

fn remap_to_hir_ty_hint<'a>(ast_ty: &ExprTy<'a>) -> HirTyHint<'a> {
    match ast_ty {
        ExprTy::Simple(ty) => HirTyHint::Primitive(ty),
        _ => panic!("Unsupported ast type")
    }
}

fn translate_decls<'a>(expr: &AstExpr<'a>) -> HirExpr<'a> {
    match expr {
        AstExpr::Identifier(id) => HirExpr { 
            id: HirId::new(), 
            kind: HirExprKind::Id(*id),
        },

        AstExpr::Integer(num) => HirExpr { 
            id: HirId::new(), 
            kind: HirExprKind::Int(*num),
        },

        AstExpr::Bool(val) => HirExpr { 
            id: HirId::new(), 
            kind: HirExprKind::Bool(*val),
        },

        AstExpr::Float(val) => HirExpr { 
            id: HirId::new(), 
            kind: HirExprKind::Float(*val),
        },

        AstExpr::Block(val) => HirExpr { 
            id: HirId::new(), 
            kind: HirExprKind::Block(val.iter().map(|expr| translate_decls(expr)).collect()),
        },

        AstExpr::Binary(val) => HirExpr { 
            id: HirId::new(), 
            kind: HirExprKind::Binary {
                op: remap_bin_op(&val.operator),
                lhs: Box::new(translate_decls(&val.lhs)),
                rhs: Box::new(translate_decls(&val.rhs))
            },
        },

        AstExpr::Return(val) => HirExpr { 
            id: HirId::new(), 
            kind: HirExprKind::Return(val.clone().map(|expr| Box::new(translate_decls(&expr)))),
        },

        AstExpr::Call(val) => HirExpr { 
            id: HirId::new(), 
            kind: HirExprKind::Call {
                name: val.name,
                args: val.args.iter().map(|expr| translate_decls(expr)).collect(),
            },
        },

        AstExpr::VarDef(val) => { 
            let var_id = HirId::new();

            let mut val_ty: Option<HirTyHint<'_>> = None;

            if val.ty.is_some() {
                val_ty = Some(remap_to_hir_ty_hint(&val.ty.clone().unwrap()));
            }
            
            HirExpr{
                id: var_id, 
                kind: HirExprKind::VarDef {
                    name: val.name,
                    value: Box::new(translate_decls(&val.content)),
                    ty: val_ty
                }
            }
        },

        AstExpr::IfExpr(val) => HirExpr { 
            id: HirId::new(), 
            kind: HirExprKind::If { 
                cond: Box::new(translate_decls( &val.logic_condition)), 
                then: Box::new(translate_decls(&val.if_block)),
                _else: val.else_block.clone().map(|expr| Box::new(translate_decls(&expr)))
            },
        },

        _ => panic!("Unsupported expression: {:?}", expr)
    }
}

pub fn translate_to_hir<'a>(ast: &'a ParsedFile<'a>) -> HirFile<'a> {
    let mut hir = HirFile {
        name: ast.name,
        items: vec![],
        imports: vec![]
    };

    for decl in &ast.content {
        match decl {
            AstDefinitions::Function(fun) => {
                let args = fun.args.iter().map(|arg| {
                    (arg.0, HirId::new(), remap_to_hir_ty_hint(&arg.1))
                }).collect();

                let func_id = HirId::new();

                hir.items.push(HirModuleItem::Func {
                    id: func_id,
                    name: &fun.name,
                    args,
                    body: translate_decls(&fun.body),
                    visibility: remap_visibility(&fun.visibility),
                    ret_ty: fun.return_type.as_ref().map(|ret_t|remap_to_hir_ty_hint(ret_t))
                });
            }
            _=> panic!("Unsupported declaration")
        }
    }
    
    hir
}