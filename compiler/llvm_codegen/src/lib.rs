use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::{Linkage, Module};
use inkwell::targets::{CodeModel, FileType, RelocMode, Target, TargetMachine, TargetTriple};
use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::values::{BasicValue, BasicValueEnum, FunctionValue};
use inkwell::{OptimizationLevel};
use itt::symbol_table::{GlobalSymbolTable, ModuleSymbolTable, SymbolTableTypes};
use itt::{
    IttByFileParametrizedTree, IttExpressions, IttFunction, IttOperators, IttTreeRootNodes,
    IttTypes, IttVariable, IttVisible,
};
use std::collections::HashMap;
use std::error::Error;
use std::ops::{Deref};
use std::path::{PathBuf};

pub enum BuildType {
    EXECUTABLE,
    DynamicLibrary,
}

pub enum CodegenOptimizationLevel {
    NONE,
    LOW,
    MEDIUM,
    STRONG,
}

pub struct CodegenConfig {
    arch: Option<String>,
    build_type: BuildType,
    optimization_level: CodegenOptimizationLevel,
    linker: String,
    linker_path: Option<String>,
}

pub struct Codegen<'a> {
    codegen_targets: Vec<&'a IttByFileParametrizedTree>,
    write_dir: String,
    config: CodegenConfig,
    triple: Option<TargetTriple>,
}

impl<'a> Codegen<'a> {
    pub fn new(
        config: CodegenConfig,
        build_dir: String,
        targets: Vec<&'a IttByFileParametrizedTree>,
    ) -> Codegen<'a> {
        Codegen {
            codegen_targets: targets,
            write_dir: build_dir,
            config,
            triple: None,
        }
    }

    fn configure_target_machine(&mut self) -> Option<TargetMachine> {
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

        self.triple = Some(triple);

        machine
    }

    pub fn generate(&mut self, symbol_table: &mut GlobalSymbolTable) {
        let machine = self.configure_target_machine();

        let context = Context::create();
        let builder = context.create_builder();

        let mut codegen = CodeGenerator {
            context: &context,
            module: None,
            builder: &builder,
            module_symbol_table: None,
            typed_scopes: vec![HashMap::new()],
            cached_functions: HashMap::new()
        };

        let mut generated_modules: Vec<Module> = Vec::new();

        self.codegen_targets.iter().for_each(|file| {
            let module = context.create_module(file.file_name.as_str());

            codegen.module_symbol_table =
                Some(symbol_table.get_module_table(file.file_name.as_str()).clone());

            if self.triple.is_some() {
                module.set_triple(self.triple.as_ref().unwrap())
            }

            codegen.module = Some(module);

            codegen.translate(file);

            generated_modules.push(codegen.module.take().unwrap());
        });

        for module in &generated_modules {
            let path =
                format!("{}/{}", self.write_dir, module.get_name().to_str().unwrap()).to_owned();
            let ll_path = PathBuf::from(format!("{}.ll", path));

            module.print_to_file(ll_path).unwrap();

            let obj_path = PathBuf::from(format!("{}.o", path));

            machine
                .as_ref()
                .unwrap()
                .write_to_file(module, FileType::Object, obj_path.as_path())
                .unwrap();
        }
    }
}

pub fn test_generation(
    file: &IttByFileParametrizedTree,
    symbol_table: &mut GlobalSymbolTable,
) -> Result<(), Box<dyn Error>> {
    let gen_config = CodegenConfig {
        arch: None,
        build_type: BuildType::EXECUTABLE,
        optimization_level: CodegenOptimizationLevel::NONE,
        linker: "".to_string(),
        linker_path: None,
    };

    let mut codegen = Codegen::new(gen_config, ".".to_string(), vec![file]);

    codegen.generate(symbol_table);

    Ok(())
}

struct CodeGenerator<'a> {
    context: &'a Context,
    module: Option<Module<'a>>,
    builder: &'a Builder<'a>,
    module_symbol_table: Option<ModuleSymbolTable>,
    typed_scopes: Vec<HashMap<String, BasicValueEnum<'a>>>,
    cached_functions: HashMap<String, FunctionValue<'a>>
}

impl<'a> CodeGenerator<'a> {
    fn get_linker_visibility(&mut self, visibility: IttVisible) -> Linkage {
        match visibility {
            IttVisible::GLOBAL => Linkage::External,
            IttVisible::PRIVATE => Linkage::Private,
        }
    }

    fn get_llvm_type(&mut self, basic_type: &IttTypes) -> BasicTypeEnum<'a> {
        match basic_type {
            IttTypes::INT => self.context.i64_type().as_basic_type_enum(),
            IttTypes::CHAR => self.context.i8_type().as_basic_type_enum(),
            IttTypes::FLOAT => self.context.f64_type().as_basic_type_enum(),
            IttTypes::BOOLEAN => self.context.i8_type().as_basic_type_enum(),
            _ => panic!("Unsupported type"),
        }
    }

    fn build_binary_expression(
        &mut self,
        lhs: BasicValueEnum<'a>,
        rhs: BasicValueEnum<'a>,
        op: &IttOperators,
    ) -> BasicValueEnum<'a> {
        let result = match op {
            IttOperators::PLUS => self
                .builder
                .build_int_add(lhs.into_int_value(), rhs.into_int_value(), "int_add")
                .unwrap(),
            IttOperators::MINUS => self
                .builder
                .build_int_sub(lhs.into_int_value(), rhs.into_int_value(), "int_sub")
                .unwrap(),
            IttOperators::DIV => self
                .builder
                .build_int_signed_div(lhs.into_int_value(), rhs.into_int_value(), "int_div")
                .unwrap(),
            IttOperators::MUL => self
                .builder
                .build_int_mul(lhs.into_int_value(), rhs.into_int_value(), "int_mul")
                .unwrap(),
        };

        result.as_basic_value_enum()
    }

    fn translate_node(&mut self, node: &'a IttExpressions) -> BasicValueEnum<'a> {
        match node {
            IttExpressions::Int(value) => BasicValueEnum::IntValue(
                self.context
                    .i64_type()
                    .const_int(unsafe { std::mem::transmute(*value) }, false),
            ),

            IttExpressions::Float(value) => BasicValueEnum::FloatValue(
                self.context
                    .f64_type()
                    .const_float(unsafe { std::mem::transmute(*value) }),
            ),

            IttExpressions::Bool(true) => self
                .context
                .bool_type()
                .const_all_ones()
                .as_basic_value_enum(),

            IttExpressions::Id(value) =>  {
                let enclosure = self.module_symbol_table.as_mut().unwrap().get_current_enclosure_index();
                self.typed_scopes.get_mut(enclosure).unwrap().get_mut(value).unwrap().clone()
            },
            
            IttExpressions::Bool(false) => {
                self.context.bool_type().const_zero().as_basic_value_enum()
            }

            IttExpressions::Binary(lhs, rhs, op, _) => {
                let lhs = self.translate_node(lhs);
                let rhs = self.translate_node(rhs);
                self.build_binary_expression(lhs, rhs, op)
            }

            IttExpressions::Call(calle, _) => {
                match calle.deref() {
                    IttExpressions::Id(val) => {
                        let fn_def = self.module_symbol_table.as_mut().unwrap().try_to_find_in_scopes(val);

                        if fn_def.is_none() {
                            //try to find in submodule scopes
                        }
                        
                        //optimize situation and try to find in module already builded function
                        let mut llvm_fn_def = self.module.as_ref().unwrap().get_function(val);
                        
                        if llvm_fn_def.is_none() {
                            // no way, we must generate dereffered declaration
                            
                            llvm_fn_def = match fn_def.unwrap().clone() {
                                SymbolTableTypes::Function(_fn) => {
                                    let result = self.generate_fn_declaration(&_fn);
                                    
                                    Some(result)
                                }
                                _ => panic!()
                            };
                        }

                        let val = self
                            .builder
                            .build_call(llvm_fn_def.unwrap(), &[], val)
                            .unwrap()
                            .try_as_basic_value();

                        match val.left() {
                            Some(v) => v,
                            None => panic!(),
                        }
                    }
                    _ => unimplemented!(),
                }
            }

            IttExpressions::Return(expr, _) => self.translate_node(expr),
            _ => panic!("Unknown node type"),
        }
    }
    
    fn generate_fn_declaration(&mut self, function: &IttFunction) ->  FunctionValue<'a> {
        
        let name = &function.name;
        
        if let Some(cached_fn) = self.cached_functions.get(name) {
                return cached_fn.clone();
        }
        
        let linkage = self.get_linker_visibility(function.visibility.clone());
        
        let argument_types: Vec<_> = function
            .arguments
            .iter()
            .map(|arg| self.get_llvm_type(&arg._type).into())
            .collect();

        let func_signature = match function.return_type {
            IttTypes::VOID => self.context.void_type().fn_type(&argument_types, false),
            _ => self
                .get_llvm_type(&function.return_type)
                .fn_type(&argument_types, false),
        };

        let fn_declaration =
            self.module
                .as_ref()
                .unwrap()
                .add_function(name, func_signature, Some(linkage));
        
        self.cached_functions.insert(name.to_string(), fn_declaration);
        
        return fn_declaration;
    }
    
    fn translate_function(&mut self, function: &'a IttFunction) {
        //
        self.module_symbol_table.as_mut().unwrap().enter_to_scope();
        //
        
        let fn_declaration = self.generate_fn_declaration(function);
        
        for i in 0..function.arguments.len() {
            let param = fn_declaration.get_nth_param(i as u32).unwrap();
            match param {
                BasicValueEnum::IntValue(val) => val.set_name(&function.arguments[i].name),
                BasicValueEnum::FloatValue(val) => val.set_name(&function.arguments[i].name),
                _ => {}
            }
            
            let enclosure = self.module_symbol_table.as_ref().unwrap().get_current_enclosure_index();
            
            self.typed_scopes.get_mut(enclosure).unwrap().insert(function.arguments[i].name.clone(), param);
        }
        
        let llvm_block = self.context.append_basic_block(fn_declaration, "fn_body");

        self.builder.position_at_end(llvm_block);
            
        let mut return_expr: Option<BasicValueEnum<'a>> = None;

        for expr in function.body.expressions.iter().rev() {
            return_expr = Some(self.translate_node(expr));
        }

        //
        self.module_symbol_table.as_mut().unwrap().exit_from_scope();
        //

        if function.return_type == IttTypes::VOID {
            self.builder.build_return(None).unwrap();
        } else {
            self.builder
                .build_return(Some(&return_expr.unwrap()))
                .unwrap();
        }
    }

    fn translate_constant(&mut self, constant: &'a IttVariable) {
        let result = self.translate_node(constant.value.as_ref().unwrap());
        //let global_value = GlobalValue::from(result.into());
        //global_value.set_linkage(self.get_linker_visibility(constant.visibility.clone()));
        //self.env.append_to_global_scope(constant.name.as_str(), result);

        //self.module.as_ref().unwrap().add_global(global_value.get_value_type(), Some(AddressSpace::default()), "lol");
    }

    fn translate(&mut self, file: &'a IttByFileParametrizedTree) {
        file.expressions.iter().for_each(|expression| {
            match expression {
                IttTreeRootNodes::Func(function) => self.translate_function(function),
                IttTreeRootNodes::ConstantValue(variable) => self.translate_constant(variable),
            };
        })
    }
}
