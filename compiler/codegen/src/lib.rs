use std::collections::HashMap;
use std::path::PathBuf;

use inkwell::basic_block::BasicBlock;
use inkwell::targets::{CodeModel, FileType, RelocMode, Target, TargetMachine, TargetTriple};
use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::values::{BasicMetadataValueEnum, BasicValue, BasicValueEnum};
use inkwell::{AddressSpace, FloatPredicate, IntPredicate, OptimizationLevel};
use inkwell::{context::Context, values::FunctionValue};
use inkwell::module::{Linkage, Module};
use inkwell::builder::Builder;
use itt::{IttBinaryOperations, IttDefinitions, IttExprs, IttIfExpression, IttType, IttVisibility, TypedNode, TypedUnit};
use itt_symbol_misc::func_table::FunctionSymbolTable;
use itt_symbol_misc::local_env::LocalEnv;
use name_mangler::mangle_function_name;

mod name_mangler;

pub fn test_gen(code_unit: &TypedUnit, function_table: &FunctionSymbolTable) {
    let context = Context::create();
    let builder = context.create_builder();
    let modul = context.create_module("test");
    
    let mut mod_gen = ModuleCodegen::new(modul, &builder, &context);
    
    let generated = mod_gen.generate(function_table, code_unit);
    
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
    
    let path = PathBuf::from("gen.o");
    
    generated.print_to_stderr();
    
    machine
        .as_ref()
        .unwrap()
        .write_to_file(&generated, FileType::Object, path.as_path())
        .unwrap();
}

pub struct ModuleCodegen<'input> {
    module: Module<'input>,
    builder: &'input Builder<'input>,
    context: &'input Context,
    generated_functions: HashMap<&'input str, FunctionValue<'input>>,
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
    
    fn get_linkage(&self, visibility: IttVisibility) -> Linkage {
        match visibility {
            IttVisibility::FILE => Linkage::Private, 
            IttVisibility::GLOBAL => Linkage::External
        }
    }
    
    fn get_llvm_type(&mut self, basic_type: &IttType) -> BasicTypeEnum<'input> {
        match basic_type {
            IttType::Int => self.context.i64_type().as_basic_type_enum(),
            IttType::Char => self.context.i8_type().as_basic_type_enum(),
            IttType::Float => self.context.f64_type().as_basic_type_enum(),
            IttType::Bool => self.context.bool_type().as_basic_type_enum(),
            IttType::String => self.context.ptr_type(AddressSpace::from(0)).as_basic_type_enum(),
            
            _ => panic!("Unsupported type"),
        }
    }
    
    fn build_sum(&self, _type: &IttType, lhs: BasicValueEnum<'input>, rhs: BasicValueEnum<'input>) -> BasicValueEnum<'input> {
        match _type {
            IttType::Int => self
                .builder.build_int_add(lhs.into_int_value(), rhs.into_int_value(), "int_add")
                .unwrap().as_basic_value_enum(),
            IttType::Float => self
                .builder.build_float_add(lhs.into_float_value(), rhs.into_float_value(), "float_add")
                .unwrap().as_basic_value_enum(),
            _ => panic!("{:?}", _type)
        }
    }
    
    fn build_sub(&self, _type: &IttType, lhs: BasicValueEnum<'input>, rhs: BasicValueEnum<'input>) -> BasicValueEnum<'input> {
        match _type {
            IttType::Int => self
                .builder.build_int_sub(lhs.into_int_value(), rhs.into_int_value(), "int_sub")
                .unwrap().as_basic_value_enum(),
            IttType::Float => self
                .builder.build_float_sub(lhs.into_float_value(), rhs.into_float_value(), "float_sub")
                .unwrap().as_basic_value_enum(),
            _ => panic!("{:?}", _type)
        }
    }
    
    fn build_div(&self, _type: &IttType, lhs: BasicValueEnum<'input>, rhs: BasicValueEnum<'input>) -> BasicValueEnum<'input> {
        match _type {
            IttType::Int => self
                .builder.build_int_signed_div(lhs.into_int_value(), rhs.into_int_value(), "int_div")
                .unwrap().as_basic_value_enum(),
            IttType::Float => self
                .builder.build_float_div(lhs.into_float_value(), rhs.into_float_value(), "float_div")
                .unwrap().as_basic_value_enum(),
            _ => panic!("")
        }
    }
    
    fn build_mul(&self, _type: &IttType, lhs: BasicValueEnum<'input>, rhs: BasicValueEnum<'input>) -> BasicValueEnum<'input> {
        match _type {
            IttType::Int => self
                .builder.build_int_mul(lhs.into_int_value(), rhs.into_int_value(), "int_mul")
                .unwrap().as_basic_value_enum(),
            IttType::Float => self
                .builder.build_float_mul(lhs.into_float_value(), rhs.into_float_value(), "float_mul")
                .unwrap().as_basic_value_enum(),
            _ => panic!("{:?}", _type)
        }
    }
    
    fn build_gt_compare(&self, _type: &IttType, lhs: BasicValueEnum<'input>, rhs: BasicValueEnum<'input>) -> BasicValueEnum<'input> {
        match _type {
            IttType::Int => self.builder.build_int_compare(IntPredicate::SGT, lhs.into_int_value(), rhs.into_int_value(), "cmpres").unwrap().as_basic_value_enum(),
            IttType::Float => self.builder.build_float_compare(FloatPredicate::OGT, lhs.into_float_value(), rhs.into_float_value(), "cmpres").unwrap().as_basic_value_enum(),
            _ => panic!("{:?}", _type)
        }
    }
    
    fn build_lt_compare(&self, _type: &IttType, lhs: BasicValueEnum<'input>, rhs: BasicValueEnum<'input>) -> BasicValueEnum<'input> {
        match _type {
            IttType::Int => self.builder.build_int_compare(IntPredicate::SLT, lhs.into_int_value(), rhs.into_int_value(), "cmpres").unwrap().as_basic_value_enum(),
            IttType::Float => self.builder.build_float_compare(FloatPredicate::OLT, lhs.into_float_value(), rhs.into_float_value(), "cmpres").unwrap().as_basic_value_enum(),
            _ => panic!("{:?}", _type)
        }
    }
    
    fn build_eq_compare(&self, _type: &IttType, lhs: BasicValueEnum<'input>, rhs: BasicValueEnum<'input>) -> BasicValueEnum<'input> {
        match _type {
            IttType::Int => self.builder.build_int_compare(IntPredicate::EQ, lhs.into_int_value(), rhs.into_int_value(), "cmpres").unwrap().as_basic_value_enum(),
            IttType::Float => self.builder.build_float_compare(FloatPredicate::UEQ, lhs.into_float_value(), rhs.into_float_value(), "cmpres").unwrap().as_basic_value_enum(),
            _ => panic!("{:?}", _type)
        }
    }
    
    fn build_binary_expression(
        &mut self,
        lhs: BasicValueEnum<'input>,
        rhs: BasicValueEnum<'input>,
        op: &IttBinaryOperations,
        expr_type: &IttType,
    ) -> BasicValueEnum<'input> {        
        match op {
            IttBinaryOperations::SUM => self.build_sum(expr_type, lhs, rhs),
            IttBinaryOperations::SUB => self.build_sub(expr_type, lhs, rhs),
            IttBinaryOperations::DIV => self.build_div(expr_type, lhs, rhs),
            IttBinaryOperations::MUL => self.build_mul(expr_type, lhs, rhs),
            IttBinaryOperations::AND => self.builder.build_and(lhs.into_int_value(), rhs.into_int_value(), "ssl_and").unwrap().as_basic_value_enum(),
            IttBinaryOperations::OR => self.builder.build_or(lhs.into_int_value(), rhs.into_int_value(), "ssl_or").unwrap().as_basic_value_enum(),
            IttBinaryOperations::GT => self.build_gt_compare(expr_type, lhs, rhs),
            IttBinaryOperations::LT => self.build_lt_compare(expr_type, lhs, rhs),
            IttBinaryOperations::EQ => self.build_eq_compare(expr_type, lhs, rhs)
        }
    } 
    
    fn generate_if_else(&mut self, function_table: &'input FunctionSymbolTable<'input>, if_expr: &'input IttIfExpression<'input>) -> BasicValueEnum<'input> {
        let condition = self.generate_node(function_table, &if_expr.logic_condition).into_int_value();
        
        let insert_block = self.builder.get_insert_block().unwrap();
        let function = insert_block.get_parent().unwrap();
        
        let then_block = self.context.append_basic_block(function, "then");
        let else_block = self.context.append_basic_block(function, "else");
    
        self.builder.build_conditional_branch(condition, then_block, else_block).unwrap();
        
        self.builder.position_at_end(then_block);
        let then_cond = self.generate_node(function_table, &if_expr.if_block);
        let then_returns = then_block.get_terminator().is_some();
        
        self.builder.position_at_end(else_block);
        let else_cond = self.generate_node(function_table, if_expr.else_block.as_ref().unwrap());
        let else_returns = else_block.get_terminator().is_some();

        if !(then_returns && else_returns) {
            let merge_block = self.context.append_basic_block(function, "ifcont");
            
            if !then_returns {
                self.builder.position_at_end(then_block);
                self.builder.build_unconditional_branch(merge_block).unwrap();
            }
            if !else_returns {
                self.builder.position_at_end(else_block);
                self.builder.build_unconditional_branch(merge_block).unwrap();
            }
    
            self.builder.position_at_end(merge_block);
            let _type = self.get_llvm_type(&if_expr.if_block._type);
            
            let phi = self.builder.build_phi(_type, "ifphi").unwrap();
            
            let incomings: [(&dyn BasicValue<'input>, BasicBlock); 2] = [
                (&then_cond, then_block),
                (&else_cond, else_block)
            ];
            
            phi.add_incoming(&incomings);
            
            return phi.as_basic_value();
        }
        
        if then_returns {
            then_cond
        } else {
            else_cond
        }
    }
    
    fn get_function_signature(
        &mut self, 
        name: &'input str, 
        arg_types: &Vec<IttType>, 
        visibility: IttVisibility, 
        return_type: IttType
    )   -> FunctionValue<'input> {
        
        let builtin_function = matches!(name, "printf");
        
        let mangled_name = if builtin_function {
            name
        } else {
            mangle_function_name(
                self.module.get_name().to_str().unwrap(), 
                name, &arg_types, 
                return_type
            )
        };
        
        if let Some(cached_fn) = self.generated_functions.get(mangled_name) {
            return cached_fn.clone();
        }
        
        let declaration = self.generate_function_signature(
            mangled_name, 
            arg_types, 
            visibility, 
            return_type
        );
        
        self.generated_functions.insert(name, declaration);
        
        declaration
    }
    
    fn generate_function_signature(
        &mut self, 
        name: &'input str, 
        arg_types: &Vec<IttType>, 
        visibility: IttVisibility, 
        return_type: IttType
    ) -> FunctionValue<'input> {           
        let argument_types: Vec<_> = arg_types
            .iter()
            .map(|arg| self.get_llvm_type(&arg).into())
            .collect();
        
        let linkage = self.get_linkage(visibility);

        let func_signature = match return_type {
            IttType::Void => self.context.void_type().fn_type(&argument_types, false),
            _ => self
                .get_llvm_type(&return_type)
                .fn_type(&argument_types, false),
        };
    
        self.module.add_function(name, func_signature, Some(linkage))
    }
    
    pub fn generate_node(&mut self, function_table: &'input FunctionSymbolTable<'input>, node: &'input TypedNode) -> BasicValueEnum<'input> {
        match &*node.node {
            IttExprs::Integer(val) => BasicValueEnum::IntValue(
                self.context.i64_type().const_int(unsafe {std::mem::transmute(*val) }, false)
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
            
            IttExprs::IfExpr(if_expr) => self.generate_if_else(function_table, &if_expr),
            
            IttExprs::Identifier(id) => self.generated_env.lookup(id).unwrap(),
            
            IttExprs::Block(block) => {
                self.generated_env.push_scope();
                
                let mut return_expr: Option<BasicValueEnum<'input>> = None;
                
                block.iter().for_each(|f| {
                    return_expr = Some(self.generate_node(function_table, f));
                });
                
                self.generated_env.pop_scope();
                
                return_expr.unwrap()
            }
            
            IttExprs::VarDef(vdef) => {
                let gen_val = self.generate_node(function_table, &vdef.content);
                
                self.generated_env.define(&vdef.name, gen_val).unwrap();
                
                gen_val
            }
            
            IttExprs::Binary(expr) => {
                let lhs = self.generate_node(function_table, &expr.lhs);
                let rhs = self.generate_node(function_table, &expr.rhs);
                self.build_binary_expression(lhs, rhs, &expr.operator, &node._type)
            }
            
            IttExprs::Call(call) => {
                let arg_types: Vec<_> = call.args.iter().map(|arg| arg._type).collect();
                
                let mangled_name = mangle_function_name(self.module.get_name().to_str().unwrap(), call.name, &arg_types, node._type);
                
                let fn_decl = self.module.get_function(mangled_name).unwrap_or_else(|| {
                    let in_module = function_table.lookup(call.name, &arg_types).unwrap();
                    
                    let transformed_args = in_module.args.iter().map(|arg| arg.1).collect();
                    self.get_function_signature(
                        in_module.name, 
                        &transformed_args, 
                        in_module.visibility, 
                        in_module.return_type, 
                    )
                });
                
                let gen_args: Vec<BasicMetadataValueEnum<'input>> = call.args.iter().map(|arg| {
                    self.generate_node(function_table, arg).into()
                }).collect();
                
                let val = self
                    .builder
                    .build_call(fn_decl, &gen_args, call.name)
                    .unwrap()
                    .try_as_basic_value();
                
                match val.left() {
                    Some(v) => v,
                    None => panic!(),
                }
            }
            
            IttExprs::Return(expr) => {
                if let Some(expr) = expr {
                    let value = self.generate_node(function_table, expr);
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
    
    pub fn generate(&mut self, function_table: &'input FunctionSymbolTable<'input>, unit: &'input TypedUnit<'input>) -> Module<'input> {
        
        unit.unit_content.iter().for_each(|f| {
            match f {
                   
                IttDefinitions::Function(function) => {
                    let transformed_args = function.args.iter().map(|arg| arg.1).collect();
                    
                    let signature = self.get_function_signature(
                        function.name, 
                        &transformed_args, 
                        function.visibility, 
                        function.return_type
                    );
                    
                    self.generated_env.push_scope();
                    
                    for i in 0..function.args.len() {
                        let param = signature.get_nth_param(i as u32).unwrap();
                        
                        param.set_name(function.args[i].0);
                            
                        self.generated_env.define(function.args[i].0, param).unwrap();
                    }
                    
                    let llvm_block = self.context.append_basic_block(signature, "fn_body");
                     
                    self.builder.position_at_end(llvm_block);
                    
                    self.generate_node(function_table, &function.body);
                    
                    self.generated_env.pop_scope();
                }
                
                _ => ()
            }
        });
        
        self.module.clone()
    }
}