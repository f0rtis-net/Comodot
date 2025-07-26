use std::{collections::HashMap, path::PathBuf};

use hir::{HirExpr, HirExprKind, HirFile, HirId, HirModuleItem, HirVisibility};
use inkwell::{builder::Builder, context::Context, module::{Linkage, Module}, targets::{CodeModel, FileType, RelocMode, Target, TargetMachine, TargetTriple}, types::{BasicType, BasicTypeEnum}, values::{BasicMetadataValueEnum, BasicValue, BasicValueEnum, FunctionValue, PointerValue}, OptimizationLevel};
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

pub fn test_gen(ctx: &mut GlobalCtx, code: &Vec<HirFile>) -> String {
    let context = Context::create();
    let builder = context.create_builder();
    let modul = context.create_module(&ctx.get_module_name());
    
    let mut mod_gen = ModuleCodegen::new(modul, &builder, &context);
    
    let generated = mod_gen.generate(code, ctx);
    
    Target::initialize_all(&Default::default());
    let triple = TargetTriple::from(TargetMachine::get_default_triple());
    
    let target = Target::from_triple(&triple).unwrap();
    
    generated.print_to_stderr();

    let machine = target.create_target_machine(
        &triple,
        "generic",
        "",
        OptimizationLevel::None,
        RelocMode::Default,
        CodeModel::Default,
    );
    
    let path = PathBuf::from(format!("{}.o", &ctx.get_module_name()));
    
    machine
        .as_ref()
        .unwrap()
        .write_to_file(&generated, FileType::Object, path.as_path())
        .unwrap();
    
    return path.to_str().unwrap_or("").to_string();
}

fn translate_to_llvm_ty<'input>(context: &'input Context, basic_type: &LangType) -> BasicTypeEnum<'input> {
    match basic_type {
        LangType::Primitives(Primitive::Int) => context.i64_type().as_basic_type_enum(),
        LangType::Primitives(Primitive::Float) => context.i8_type().as_basic_type_enum(),
        LangType::Primitives(Primitive::Bool) => context.f64_type().as_basic_type_enum(),
        LangType::Primitives(Primitive::Char) => context.bool_type().as_basic_type_enum(),
        
        _ => panic!("Unsupported type"),
    }
}

pub struct ModuleCodegen<'input> {
    module: Module<'input>,
    builder: &'input Builder<'input>,
    context: &'input Context,
    generated_functions: HashMap<String, FunctionValue<'input>>,
    variable_env: VariableEnv<'input>,
}

impl <'input> ModuleCodegen<'input> {
    pub fn new(module: Module<'input>, builder: &'input Builder<'input>, context: &'input Context) -> Self {
        ModuleCodegen {
            module,
            builder,
            context,
            generated_functions: HashMap::new(),
            variable_env: VariableEnv::new()
        }
    }
    
    fn get_function_signature(
        &mut self, 
        name: &str, 
        arg_types: &Vec<LangType>, 
        is_extern: bool, 
        return_type: LangType
    )   -> FunctionValue<'input> {
                
        if let Some(cached_fn) = self.generated_functions.get(name) {
            return cached_fn.clone();
        }
        
        let declaration = self.generate_function_definition(
            name, 
            arg_types, 
            is_extern, 
            return_type
        );
        
        self.generated_functions.insert(name.to_string(), declaration);
        
        declaration
    }
    
    fn generate_function_definition(
        &mut self, 
        name: &str, 
        arg_types: &Vec<LangType>, 
        is_extern: bool,
        return_type: LangType
    ) -> FunctionValue<'input> {           
        let argument_types: Vec<_> = arg_types
            .iter()
            .map(|arg| translate_to_llvm_ty(self.context, arg).into())
            .collect();
        
        let linkage = if is_extern {
            Linkage::External
        } else {
            Linkage::Internal
        };
        
        let func_signature = match return_type {
            LangType::Primitives(Primitive::Unit) => self.context.void_type().fn_type(&argument_types, false),
            _ => 
                translate_to_llvm_ty(self.context, &return_type)
                .fn_type(&argument_types, false),
        };
        
        self.module.add_function(name, func_signature, Some(linkage))
    }
    
    pub fn generate_node(&mut self, ctx: &mut GlobalCtx, node: &'input HirExpr) -> BasicValueEnum<'input> {
        match &node.kind {
            HirExprKind::Int(val) => BasicValueEnum::IntValue(
                self.context.i64_type().const_int(unsafe { std::mem::transmute(*val) }, false)
            ),
            
            HirExprKind::Float(val) => BasicValueEnum::FloatValue(
                self.context.f64_type().const_float(*val)
            ),
            
            HirExprKind::Bool(val) => {
                if *val == true {
                    self.context.bool_type().const_all_ones().as_basic_value_enum()
                } else {
                    self.context.bool_type().const_zero().as_basic_value_enum()
                }
            }
            
            HirExprKind::Id(id) =>  {
                self.variable_env.get_variable(&id).unwrap()
            }
            
            HirExprKind::Binary { op, lhs, rhs } => {
                let lhs = self.generate_node(ctx, lhs);

                let rhs = self.generate_node(ctx, rhs);
                
                let ty = ctx.get_module_ty_info().get_type(&node.id).unwrap();

                build_llvm_binop(&self.builder, lhs, rhs, op, &ty.ty)
            }

            HirExprKind::Call { name, args} => {
                let arg_types: Vec<LangType> = args.iter()
                    .map(|arg| {
                        ctx.get_module_ty_info()
                            .get_type(&arg.id)
                            .map(|ti| ti.ty.clone())
                            .unwrap()
                    })
                    .collect();

                let llvm_args: Vec<BasicMetadataValueEnum<'input>> = args.iter()
                    .map(|arg| self.generate_node(ctx, arg).into())
                    .collect();

                let func_ty = ctx.get_module_ty_info().get_type(&node.id).unwrap();
                let return_ty = func_ty.ty.clone();
            
                let function = self.get_function_signature(
                    name,
                    &arg_types,
                    false, 
                    return_ty,
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

            HirExprKind::Block(block) => {
                //self.generated_env.push_scope();
                
                let mut return_expr: Option<BasicValueEnum<'input>> = None;
                
                block.iter().for_each(|f| {
                    return_expr = Some(self.generate_node(ctx, f));
                });
                
                //self.generated_env.pop_scope();
                
                return_expr.unwrap()
            }
            
            HirExprKind::Return(expr) => {
                if let Some(expr) = expr {
                    let value = self.generate_node(ctx, expr);
                    self.builder.build_return(Some(&value)).unwrap();
                } else {
                    self.builder.build_return(None).unwrap();
                }
                
                self.default_val()
            }

            _ => panic!("Invalid node to generate.")
        }   
    }
    
    fn default_val(&self) -> BasicValueEnum<'input> {
        BasicValueEnum::IntValue(self.context.i64_type().const_zero())
    }
    
    fn generate_function(&mut self, ctx: &mut GlobalCtx, id: &HirId, name: String, args: &Vec<(&str, HirId)>, body: &'input HirExpr<'input>, visibility: &HirVisibility) {
        let is_globally_visible = match visibility {
            HirVisibility::Public => true,
            HirVisibility::Private => false
        };

        let ret_ty = ctx.get_module_ty_info().get_type(id).unwrap().ty.clone();
        let arg_types: Vec<_> = args.iter()
            .map(|(_, arg_id)| {
                ctx.get_module_ty_info().get_type(arg_id).unwrap().ty.clone()
            })
            .collect();

        let signature = self.get_function_signature(
            &name, 
            &arg_types, 
        is_globally_visible, 
        ret_ty
        );

        for i in 0..args.len() {
            let param = signature.get_nth_param(i as u32).unwrap();
            param.set_name(args[i].0);
            self.variable_env.declare_variable(args[i].0, param);
        }

        let llvm_block = self.context.append_basic_block(signature, "start");
        self.builder.position_at_end(llvm_block);
        self.generate_node(ctx, &body);

        if llvm_block.get_terminator().is_none() {
            self.builder.build_return(None).unwrap();
        }
    }
    
    pub fn generate(&mut self, code: &'input Vec<HirFile>, ctx: &mut GlobalCtx) -> Module<'input> {    
        for file in code {
            for node in &file.items {
                match node {
                    HirModuleItem::Func { id, name, args, body, visibility } => {
                        self.generate_function(ctx, id, name.to_string(), args, &body, visibility);
                    }
                }
            }
        }
        
        self.module.clone()
    }
}