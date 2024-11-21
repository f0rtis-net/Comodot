use std::collections::HashMap;
use std::path::PathBuf;

use builder::build_llvm_binop;
use inkwell::basic_block::BasicBlock;
use inkwell::targets::{CodeModel, FileType, RelocMode, Target, TargetMachine, TargetTriple};
use inkwell::types::BasicType;
use inkwell::values::{BasicMetadataValueEnum, BasicValue, BasicValueEnum};
use inkwell::OptimizationLevel;
use inkwell::{context::Context, values::FunctionValue};
use inkwell::module::{Linkage, Module};
use inkwell::builder::Builder;
use itt::{IttDefinitions, IttExprs, IttFunction, IttIfExpression, IttType, IttVisibility, TypedNode, TypedUnit};
use itt_symbol_misc::local_env::LocalEnv;
use itt_symbol_misc::name_mangler::mangle_function_name;
use misc::get_llvm_type;

mod misc;
mod builder;

pub fn test_gen(code_unit: &TypedUnit) -> String {
    let context = Context::create();
    let builder = context.create_builder();
    let modul = context.create_module(code_unit.unit_name);
    
    let mut mod_gen = ModuleCodegen::new(modul, &builder, &context);
    
    let generated = mod_gen.generate(code_unit);
    
    Target::initialize_all(&Default::default());
    let triple = TargetTriple::from(TargetMachine::get_default_triple());
    
    let target = Target::from_triple(&triple).unwrap();
    
    let machine = target.create_target_machine(
        &triple,
        "generic",
        "",
        OptimizationLevel::None,
        RelocMode::Default,
        CodeModel::Default,
    );
    
    let path = PathBuf::from(format!("{}.o", code_unit.unit_name));
    
    //generated.print_to_stderr();
    
    machine
        .as_ref()
        .unwrap()
        .write_to_file(&generated, FileType::Object, path.as_path())
        .unwrap();
    
    return path.to_str().unwrap_or("").to_string();
}

pub struct ModuleCodegen<'input> {
    module: Module<'input>,
    builder: &'input Builder<'input>,
    context: &'input Context,
    generated_functions: HashMap<String, FunctionValue<'input>>,
    generated_env: LocalEnv<'input, BasicValueEnum<'input>>
}

impl <'input> ModuleCodegen<'input> {
    pub fn new(module: Module<'input>, builder: &'input Builder<'input>, context: &'input Context) -> Self {
        ModuleCodegen {
            module,
            builder,
            context,
            generated_functions: HashMap::new(),
            generated_env: LocalEnv::new()
        }
    }
    
    fn generate_if_else(&mut self, if_expr: &'input IttIfExpression<'input>) -> BasicValueEnum<'input> {
        let condition = self.generate_node(&if_expr.logic_condition).into_int_value();
        
        let insert_block = self.builder.get_insert_block().unwrap();
        let function = insert_block.get_parent().unwrap();
        
        let mut then_block = self.context.append_basic_block(function, "then");
        
        let mut else_block: Option<BasicBlock<'input>> = if if_expr.else_block.is_some() {
            Some(self.context.append_basic_block(function, "else"))
        } else { 
            None 
        };
        
        let done_block = self.context.append_basic_block(function, "done");
        
        if else_block.is_some() {
            self.builder.build_conditional_branch(condition, then_block, else_block.unwrap()).unwrap();
        } else {
            self.builder.build_conditional_branch(condition, then_block, done_block).unwrap();
        }
        
        self.builder.position_at_end(then_block);
        self.generate_node(&if_expr.if_block);
        then_block = self.builder.get_insert_block().unwrap();
        
        if then_block.get_terminator().is_none() {
            self.builder.build_unconditional_branch(done_block).unwrap();
        }
            
        if else_block.is_some() {
            self.builder.position_at_end(else_block.unwrap());
            self.generate_node(if_expr.else_block.as_ref().unwrap());
            self.builder.build_unconditional_branch(done_block).unwrap();
            else_block = Some(self.builder.get_insert_block().unwrap());
            
            if else_block.unwrap().get_terminator().is_none() {
                self.builder.build_unconditional_branch(done_block).unwrap();
            }
        } 
        
        self.builder.position_at_end(done_block);
        
        self.default_val()
    }
    
    fn get_function_signature(
        &mut self, 
        name: &str, 
        arg_types: &Vec<IttType>, 
        is_extern: bool, 
        return_type: IttType
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
        arg_types: &Vec<IttType>, 
        is_extern: bool,
        return_type: IttType
    ) -> FunctionValue<'input> {           
        let argument_types: Vec<_> = arg_types
            .iter()
            .map(|arg| get_llvm_type(self.context, arg).into())
            .collect();
        
        let linkage = if is_extern {
            Linkage::External
        } else {
            Linkage::Internal
        };
        
        let func_signature = match return_type {
            IttType::Void => self.context.void_type().fn_type(&argument_types, false),
            _ => 
                get_llvm_type(self.context, &return_type)
                .fn_type(&argument_types, false),
        };
        
        self.module.add_function(name, func_signature, Some(linkage))
    }
    
    pub fn generate_node(&mut self, node: &'input TypedNode) -> BasicValueEnum<'input> {
        match &*node.node {
            IttExprs::Integer(val) => BasicValueEnum::IntValue(
                self.context.i64_type().const_int(unsafe { std::mem::transmute(*val) }, false)
            ),
            
            IttExprs::Float(val) => BasicValueEnum::FloatValue(
                self.context.f64_type().const_float(*val)
            ),
            
            IttExprs::Bool(val) => {
                if *val == true {
                    self.context.bool_type().const_all_ones().as_basic_value_enum()
                } else {
                    self.context.bool_type().const_zero().as_basic_value_enum()
                }
            }
            
            IttExprs::String(val) => {
                let string = self.builder.build_global_string_ptr(val, format!("g_s<{val}>").as_str()).unwrap();
                string.as_basic_value_enum()
            }
            
            IttExprs::IfExpr(if_expr) => self.generate_if_else(&if_expr),
            
            IttExprs::Identifier(id) => self.generated_env.lookup(id).unwrap(),
            
            IttExprs::Block(block) => {
                self.generated_env.push_scope();
                
                let mut return_expr: Option<BasicValueEnum<'input>> = None;
                
                block.iter().for_each(|f| {
                    return_expr = Some(self.generate_node(f));
                });
                
                self.generated_env.pop_scope();
                
                return_expr.unwrap()
            }
            
            IttExprs::VarDef(vdef) => {
                let gen_val = self.generate_node(&vdef.content);
                
                self.generated_env.define(&vdef.name, gen_val).unwrap();
                
                gen_val
            }
            
            IttExprs::Binary(expr) => {
                let lhs = self.generate_node(&expr.lhs);
                let rhs = self.generate_node(&expr.rhs);
                build_llvm_binop(&self.builder, lhs, rhs, &expr.operator, &expr.lhs._type)
            }
            
            IttExprs::Call(call) => {
                let arg_types: Vec<_> = call.args.iter().map(|arg| arg._type).collect();
                
                let module_alias = call.alias.unwrap_or(self.module.get_name().to_str().unwrap());
                
                let end_call_name = if matches!(call.visibility, IttVisibility::EXTERN) {
                    call.name.to_string()
                } else {
                    mangle_function_name(module_alias, &call.name, &arg_types).unwrap()
                };
                
                let fn_decl = self.get_function_signature(
                    &end_call_name, 
                    &arg_types, 
                    matches!(call.visibility, IttVisibility::EXTERN | IttVisibility::PUBLIC), 
                    node._type, 
                );
                
                let gen_args: Vec<BasicMetadataValueEnum<'input>> = call.args.iter().map(|arg| {
                    self.generate_node(arg).into()
                }).collect();
                
                let val = self
                    .builder
                    .build_call(fn_decl, &gen_args, call.name.as_str())
                    .unwrap();
                
                if node._type == IttType::Void {
                    return self.default_val()
                }
                
                match val.try_as_basic_value().left() {
                    Some(v) => v,
                    None => panic!(),
                }
            }
            
            IttExprs::Return(expr) => {
                if let Some(expr) = expr {
                    let value = self.generate_node(expr);
                    self.builder.build_return(Some(&value)).unwrap();
                } else {
                    self.builder.build_return(None).unwrap();
                }
                
                self.default_val()
            }
        }
    }
    
    fn default_val(&self) -> BasicValueEnum<'input> {
        BasicValueEnum::IntValue(self.context.i64_type().const_zero())
    }
    
    fn generate_function(&mut self, function: &'input IttFunction) {
        let transformed_args = function.args.iter().map(|arg| arg.1).collect();
        
        let mangled_name = mangle_function_name(self.module.get_name().to_str().unwrap(), &function.name, &transformed_args).unwrap();
        
        let is_globally_visible = match function.visibility {
            IttVisibility::EXTERN | IttVisibility::PUBLIC => true,
            IttVisibility::PRIVATE => false
        };
        
        let signature = self.get_function_signature(
            &mangled_name, 
            &transformed_args, 
            is_globally_visible, 
            function.return_type
        );
        
        self.generated_env.push_scope();
        
        for i in 0..function.args.len() {
            let param = signature.get_nth_param(i as u32).unwrap();
            
            param.set_name(function.args[i].0);
                
            self.generated_env.define(function.args[i].0, param).unwrap();
        }
        
        let llvm_block = self.context.append_basic_block(signature, "start");
        
        self.builder.position_at_end(llvm_block);
        
        self.generate_node(&function.body);
        
        if llvm_block.get_terminator().is_none() {
            self.builder.build_return(None).unwrap();
        }
        
        self.generated_env.pop_scope();
    }
    
    pub fn generate(&mut self, unit: &'input TypedUnit<'input>) -> Module<'input> {    
        unit.unit_content.iter().for_each(|f| {
            match f {
                IttDefinitions::Function(function) => self.generate_function(function), 
                _ => ()
            }
        });
        
        self.module.clone()
    }
}