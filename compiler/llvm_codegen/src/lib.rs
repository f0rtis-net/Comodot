use std::{collections::HashMap};

use hir::{HirExpr, HirExprKind, HirModuleItem, HirVisibility};
use inkwell::{builder::Builder, context::Context, module::{Linkage, Module}, passes::PassBuilderOptions, targets::{InitializationConfig, Target, TargetMachine}, types::{BasicType, BasicTypeEnum}, values::{BasicMetadataValueEnum, BasicValue, BasicValueEnum, FunctionValue, PointerValue}};
use middle::{ty::{LangType, Primitive}, GlobalCtx};

use crate::builder::build_llvm_binop;

pub mod builder;

#[derive(Debug, Clone)]
struct PtrValue<'ctx> {
    ptr: PointerValue<'ctx>,
    value_type: BasicTypeEnum<'ctx>
}

struct VariableEnv<'ctx> {
    scopes: Vec<HashMap<&'ctx str, PtrValue<'ctx>>>
}

impl<'ctx> VariableEnv<'ctx> {
    fn new() -> Self {
        let mut global = Self {
            scopes: vec![]
        };
        
        global.push_scope();
        
        global
    }

    fn push_scope(&mut self) { self.scopes.push(HashMap::new()); }
    
    fn pop_scope(&mut self) { self.scopes.pop(); }

    fn declare_variable(&mut self, name: &'ctx str, value: PtrValue<'ctx>) -> Result<(), String> {
        if self.scopes.last_mut().unwrap().insert(name, value).is_some() {
            Err(String::from("Can not define symbol in enviroment"))
        } else {
            Ok(())
        }
    }

    fn get_variable(&self, name: &str) -> Option<PtrValue<'ctx>> {
        for scope in self.scopes.iter().rev() {
            match scope.get(name) {
                Some(value) => return Some(value.clone()),
                None => continue
            }
        }
        
        None
    }
}

fn translate_to_llvm_ty<'input>(context: &'input Context, basic_type: &LangType) -> BasicTypeEnum<'input> {
    match basic_type {
        LangType::Primitives(Primitive::Int) => context.i64_type().as_basic_type_enum(),
        LangType::Primitives(Primitive::Float) => context.f64_type().as_basic_type_enum(),
        LangType::Primitives(Primitive::Bool) => context.bool_type().as_basic_type_enum(),
        LangType::Primitives(Primitive::Char) => context.i8_type().as_basic_type_enum(),
        
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
                let val = self.env_variables.get_variable(&id).unwrap();
                self.builder.build_load(
                        val.value_type,
                        val.ptr, 
                        id).unwrap()
            }
            
            HirExprKind::Binary { op, lhs, rhs } => {
                let lhs = self.generate_inner_decls_ir(lhs);

                let rhs = self.generate_inner_decls_ir(rhs);
                
                let ty = self.global_ctx.module_ty_info.borrow().get_type(&node.id).unwrap().ty.clone();

                build_llvm_binop(&self.builder, lhs, rhs, op, &ty)
            }

            HirExprKind::Block(block) => {
                self.env_variables.push_scope();
                
                let mut return_expr: Option<BasicValueEnum<'llvm>> = None;
                
                block.iter().for_each(|f| {
                    return_expr = Some(self.generate_inner_decls_ir(f));
                });
                
                self.env_variables.pop_scope();
                
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

            
            HirExprKind::If { cond, then, _else } => {
                let condition = self.generate_inner_decls_ir(&cond).into_int_value();
                
                let current_block = self.builder.get_insert_block().unwrap();
                let function = current_block.get_parent().unwrap();
                
                let result_type = translate_to_llvm_ty(
                    self.llvm_ctx, 
                    &self.global_ctx.module_ty_info.borrow().get_type(&node.id).unwrap().ty
                );
                
                let result_alloca = self.builder.build_alloca(result_type, "if_result").unwrap();

                let then_block = self.llvm_ctx.append_basic_block(function, "then");
                let else_block = self.llvm_ctx.append_basic_block(function, "else");
                let merge_block = self.llvm_ctx.append_basic_block(function, "merge");
                
                self.builder.build_conditional_branch(condition, then_block, else_block).unwrap();
                
                self.builder.position_at_end(then_block);
                let then_val = self.generate_inner_decls_ir(&then);
                
                if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
                    self.builder.build_store(result_alloca, then_val).unwrap();
                    self.builder.build_unconditional_branch(merge_block).unwrap();
                }
                
                self.builder.position_at_end(else_block);
                let else_val = if let Some(else_expr) = _else {
                    self.generate_inner_decls_ir(else_expr)
                } else {
                    match result_type {
                        BasicTypeEnum::IntType(int_ty) => int_ty.const_zero().as_basic_value_enum(),
                        BasicTypeEnum::FloatType(float_ty) => float_ty.const_float(0.0).as_basic_value_enum(),
                        _ => self.default_val(),
                    }
                };
                
                if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
                    self.builder.build_store(result_alloca, else_val).unwrap();
                    self.builder.build_unconditional_branch(merge_block).unwrap();
                }
                
                self.builder.position_at_end(merge_block);
                
                if merge_block.get_first_instruction().is_some() || 
                merge_block.get_terminator().is_some() {
                    self.builder.build_load(result_type, result_alloca, "if_result").unwrap()
                } else {
                    self.default_val()
                }
            }

            HirExprKind::VarDef { name, value } => {
                let val: BasicValueEnum<'_> = self.generate_inner_decls_ir(&value);
                let alloca = self.builder.build_alloca(val.get_type(), name).unwrap();

                self.builder.build_store(alloca, val).unwrap();

                self.env_variables.declare_variable(name, PtrValue {
                    ptr: alloca,
                    value_type: val.get_type()
                }).unwrap();

                self.default_val()
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

                        let llvm_block = self.llvm_ctx.append_basic_block(signature, "start");
                        self.builder.position_at_end(llvm_block);

                        for i in 0..args.len() {
                            let param = signature.get_nth_param(i as u32).unwrap();
                            param.set_name(args[i].0);

                            let alloca = self.builder.build_alloca(param.get_type(), args[i].0).unwrap();

                            self.builder.build_store(alloca, param).unwrap();

                            self.env_variables.declare_variable(args[i].0, PtrValue { 
                                ptr: alloca, 
                                value_type:  param.get_type()
                            }).unwrap();
                        }

                        self.generate_inner_decls_ir(&body);

                        let current_block = self.builder.get_insert_block().unwrap();

                        if current_block.get_terminator().is_none() {
                            if matches!(ret_ty.ty, LangType::Primitives(Primitive::Unit)) {
                                self.builder.build_return(None).unwrap();
                            } else {
                                self.builder.build_unreachable().unwrap();
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn generate_object_code<'a, 'b>(ctx: &'a GlobalCtx<'a>) -> Vec<u8> {
    let llvm_ctx = Context::create();

    Target::initialize_all(&InitializationConfig::default());

    let triple = TargetMachine::get_default_triple();
    let cpu_features = TargetMachine::get_host_cpu_features();
    let cpu_name = TargetMachine::get_host_cpu_name();
    let target = Target::from_triple(&triple).unwrap();

    let machine = target
        .create_target_machine(
            &triple,
            cpu_name.to_str().unwrap(),
            cpu_features.to_str().unwrap(),
            inkwell::OptimizationLevel::Aggressive,
            inkwell::targets::RelocMode::PIC,
            inkwell::targets::CodeModel::Default,
        )
        .unwrap();

    let passopt = PassBuilderOptions::create();
    passopt.set_verify_each(true);

    let mut module_gen = ModuleCodeGenerator::new(&llvm_ctx, ctx);

    module_gen.generate_ir();

    module_gen.llvm_mod.set_triple(&triple);
    module_gen.llvm_mod.set_data_layout(&machine.get_target_data().get_data_layout());

    module_gen.llvm_mod.run_passes(&format!("default<O3>"), &machine, passopt).unwrap();

    module_gen.llvm_mod.print_to_stderr();
    
    let mem_buf = machine.write_to_memory_buffer(
        &module_gen.llvm_mod, 
        inkwell::targets::FileType::Object
    ).unwrap();

    mem_buf.as_slice().to_vec()
}