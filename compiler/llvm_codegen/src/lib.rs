use std::{collections::HashMap, path::PathBuf};

use hir::{HirExpr, HirExprKind, HirFile, HirId, HirModuleItem, HirVisibility};
use inkwell::{builder::Builder, context::{self, Context}, module::{self, Linkage, Module}, targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine, TargetTriple}, types::{BasicType, BasicTypeEnum}, values::{BasicMetadataValueEnum, BasicValue, BasicValueEnum, FunctionValue, PointerValue}, OptimizationLevel};
use middle::{ty::{LangType, Primitive}, GlobalCtx};

use crate::builder::build_llvm_binop;

pub mod builder;

struct VariableEnv<'ctx> {
    variables: HashMap<String, BasicValueEnum<'ctx>>,
}

impl<'ctx> VariableEnv<'ctx> {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn declare_variable(&mut self, name: &str, ptr: BasicValueEnum<'ctx>) {
        self.variables.insert(name.to_string(), ptr);
    }

    pub fn get_variable(&self, name: &str) -> Option<BasicValueEnum<'ctx>> {
        self.variables.get(name).copied()
    }
}

fn translate_to_llvm_ty<'input>(context: &'input Context, basic_type: &LangType) -> BasicTypeEnum<'input> {
    match basic_type {
        LangType::Primitives(Primitive::Int) => context.i64_type().as_basic_type_enum(),
        LangType::Primitives(Primitive::Float) => context.i8_type().as_basic_type_enum(),
        LangType::Primitives(Primitive::Bool) => context.f64_type().as_basic_type_enum(),
        LangType::Primitives(Primitive::Char) => context.bool_type().as_basic_type_enum(),
        
        _ => panic!("Unsupported type: {:?}", basic_type),
    }
}


struct ModuleCodeGenerator<'llvm, 'global: 'llvm> {
    llvm_mod: Module<'llvm>,
    builder: Builder<'llvm>,
    llvm_ctx: &'llvm Context,
    env_variables: VariableEnv<'llvm>,
    func_env: HashMap<String, FunctionValue<'llvm>>,

    global_ctx: &'global GlobalCtx<'global>,
}

impl<'llvm, 'global: 'llvm> ModuleCodeGenerator<'llvm, 'global> {
    pub fn new(context: &'llvm Context, global_ctx: &'global GlobalCtx<'global>) -> Self {
        Self {
            llvm_mod: context.create_module(&global_ctx.module_name),
            builder: context.create_builder(),
            global_ctx,
            llvm_ctx: context,
            env_variables: VariableEnv::new(),
            func_env: HashMap::new(),
        }
    }

    fn translate_to_function_sig( &mut self, 
        name: &str, 
        arg_types: &Vec<LangType>, 
        is_global: bool, 
        return_type: &LangType,
    ) -> FunctionValue<'llvm>{

        if let Some(cached_fn) = self.func_env.get(name) {
            return cached_fn.clone();
        }

        let argument_types: Vec<_> = arg_types
                .iter()
                .map(|arg| translate_to_llvm_ty(self.llvm_ctx, arg).into())
                .collect();
        
        let linkage = if is_global {
            Linkage::External
        } else {
            Linkage::Internal
        };

        let func_signature = match return_type {
            LangType::Primitives(Primitive::Unit) => self.llvm_ctx.void_type().fn_type(&argument_types, false),
            _ => 
                translate_to_llvm_ty(self.llvm_ctx, &return_type)
                    .fn_type(&argument_types, false),
        };

        let func = self.llvm_mod.add_function(name, func_signature, Some(linkage));

        self.func_env.insert(name.to_string(), func);

        func
    }

    fn generate_inner_decls_ir(&mut self, node: &'llvm HirExpr) -> BasicValueEnum<'llvm> {
        match &node.kind {
            HirExprKind::Int(val) => BasicValueEnum::IntValue(
                self.llvm_ctx.i64_type().const_int(unsafe { std::mem::transmute(*val) }, false)
            ),
            
            HirExprKind::Float(val) => BasicValueEnum::FloatValue(
                self.llvm_ctx.f64_type().const_float(*val)
            ),
            
            HirExprKind::Bool(val) => {
                if *val == true {
                    self.llvm_ctx.bool_type().const_all_ones().as_basic_value_enum()
                } else {
                    self.llvm_ctx.bool_type().const_zero().as_basic_value_enum()
                }
            }

            HirExprKind::Id(id) =>  {
                self.env_variables.get_variable(&id).unwrap()
            }
            
            HirExprKind::Binary { op, lhs, rhs } => {
                let lhs = self.generate_inner_decls_ir(lhs);

                let rhs = self.generate_inner_decls_ir(rhs);
                
                let ty = self.global_ctx.module_ty_info.borrow().get_type(&node.id).unwrap().ty.clone();

                build_llvm_binop(&self.builder, lhs, rhs, op, &ty)
            }

            HirExprKind::Block(block) => {
                //self.env_variables.push_scope();
                
                let mut return_expr: Option<BasicValueEnum<'llvm>> = None;
                
                block.iter().for_each(|f| {
                    return_expr = Some(self.generate_inner_decls_ir(f));
                });
                
                //self.env_variables.pop_scope();
                
                return_expr.unwrap()
            }

            HirExprKind::Return(expr) => {
                if let Some(expr) = expr {
                    let value = self.generate_inner_decls_ir(expr);
                    self.builder.build_return(Some(&value)).unwrap();
                } else {
                    self.builder.build_return(None).unwrap();
                }
                
                self.default_val()
            }

            HirExprKind::Call { name, args} => {
                let arg_types: Vec<LangType> = args.iter()
                    .map(|arg| {
                        self.global_ctx.module_ty_info.borrow()
                            .get_type(&arg.id)
                            .map(|ti| ti.ty.clone())
                            .unwrap()
                    })
                    .collect();

                let llvm_args: Vec<BasicMetadataValueEnum<'llvm>> = args.iter()
                    .map(|arg| self.generate_inner_decls_ir(arg).into())
                    .collect();

                let func_ty = self.global_ctx.module_ty_info.borrow().get_type(&node.id).unwrap().clone();
                let return_ty = func_ty.ty.clone();
            
                let function = self.translate_to_function_sig(
                    name,
                    &arg_types,
                    false, 
                    &return_ty,
                );

                let call = self.builder.build_call(
                    function,
                    llvm_args.as_slice(),
                    name,
                ).unwrap();

                match call.try_as_basic_value().left() {
                    Some(value) => value,
                    None => self.default_val(),     
                }
            }

            _ => panic!("Unsupported node: {:?}", node),
        }
    }

    fn default_val(&self) -> BasicValueEnum<'llvm> {
        BasicValueEnum::IntValue(self.llvm_ctx.i64_type().const_zero())
    }

    fn generate_ir(&mut self) {
        for file in &self.global_ctx.module_files {
            for decl in &file.items {
                match decl {
                    HirModuleItem::Func { id, name, args, body, visibility } => {
                        let is_global = match visibility {
                            HirVisibility::Public => true,
                            HirVisibility::Private => false
                        };

                        let ret_ty = self.global_ctx.module_ty_info.borrow().get_type(id).cloned().unwrap();

                        let arg_types: Vec<_> = args.iter()
                            .map(|(_, arg_id)| {
                                self.global_ctx.module_ty_info.borrow().get_type(arg_id).unwrap().ty.clone()
                            })
                            .collect();

                        let signature = self.translate_to_function_sig(
                            &name, 
                            &arg_types, 
                            is_global, 
                            &ret_ty.ty
                        );

                        for i in 0..args.len() {
                            let param = signature.get_nth_param(i as u32).unwrap();
                            param.set_name(args[i].0);
                            self.env_variables.declare_variable(args[i].0, param);
                        }

                        let llvm_block = self.llvm_ctx.append_basic_block(signature, "start");
                        self.builder.position_at_end(llvm_block);

                        self.generate_inner_decls_ir(&body);

                        if llvm_block.get_terminator().is_none() {
                            self.builder.build_return(None).unwrap();
                        }
                    }
                }
            }
        }

        self.llvm_mod.print_to_stderr();
    }
}

pub fn generate_object_code<'a>(ctx: &'a GlobalCtx<'a>) {
    let llvm_ctx = Context::create();

    Target::initialize_all(&Default::default());

    let mut module_gen = ModuleCodeGenerator::new(&llvm_ctx, ctx);

    module_gen.generate_ir();
}