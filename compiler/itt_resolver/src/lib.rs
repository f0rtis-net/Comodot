use std::cell::RefCell;

use itt::{IttBinaryOperations, IttDefinitions, IttExprs, IttFunction, IttType, TypedNode, TypedUnit};
use itt_symbol_misc::{func_table::FunctionSymbolTable, local_env::LocalEnv};

#[derive(Clone, Debug)]
struct VariableForResolve<'input> {
    pub name: &'input str,
    pub _type: IttType
}

pub struct IttTreeTypeResolver<'input> {
    function_table: &'input RefCell<FunctionSymbolTable<'input>>,
    local_context: LocalEnv<'input, VariableForResolve<'input>>,
}

impl<'input> IttTreeTypeResolver<'input> {
    pub fn new(fn_table: &'input RefCell<FunctionSymbolTable<'input>>) -> Self {
        Self {
            function_table: fn_table,
            local_context: LocalEnv::new(),
        }
    }
    
    fn is_boolean_actions(&mut self, op: IttBinaryOperations) -> bool {
        matches!(
            op, 
            
            IttBinaryOperations::AND
            | IttBinaryOperations::OR
            | IttBinaryOperations::GT
            | IttBinaryOperations::LT
            | IttBinaryOperations::EQ
        )
    }
    
    fn process_in_fn_expressions(&mut self, unit_name: &str, node: &mut TypedNode<'input>) {
        match node.node.as_mut() {
            IttExprs::Block(block) => {
                self.local_context.push_scope();
                block.iter_mut().for_each(|stmt| self.process_in_fn_expressions(unit_name, stmt));
                self.local_context.pop_scope();
                
                node._type = block.last().unwrap()._type;
            }
    
            IttExprs::Identifier(id) => {
                let found_type = self.local_context.lookup(id);
                if let Some(found) = found_type {
                    node._type = found._type;
                } else {
                    panic!("Symbol not found in context");
                }
            }
            
            IttExprs::Call(calle) => {                
                calle.args.iter_mut().for_each(|arg| {
                    self.process_in_fn_expressions(unit_name, arg);
                });
                
                let arg_types = calle.args.iter().map(|arg| arg._type).collect();
                let mt = self.function_table.borrow();
                let fn_from_table = mt.lookup(unit_name, calle.name, &arg_types);
    
                if let Some(fn_info) = fn_from_table {
                    node._type = fn_info.return_type;
                } else {
                    panic!("No candidate for call.");
                }
            }
            
            IttExprs::IfExpr(expr) => {
                self.process_in_fn_expressions(unit_name, &mut expr.logic_condition);
                self.process_in_fn_expressions(unit_name ,&mut expr.if_block);
                
                if expr.else_block.is_some() {
                    self.process_in_fn_expressions(unit_name ,expr.else_block.as_mut().unwrap());
                }
                
                node._type = expr.if_block._type;
            }
            
            IttExprs::Binary(binary) => {
                self.process_in_fn_expressions(unit_name, &mut binary.lhs);
                self.process_in_fn_expressions(unit_name, &mut binary.rhs);
                
                if self.is_boolean_actions(binary.operator) {
                    node._type = IttType::Bool
                } else {
                    node._type = binary.lhs._type;
                }
            }
            
            IttExprs::Return(ret) => {
                if ret.is_some() {
                    self.process_in_fn_expressions(unit_name, ret.as_mut().unwrap());
                    node._type = ret.as_ref().unwrap()._type;
                }
            }
            
            IttExprs::VarDef(var) => {
                self.process_in_fn_expressions(unit_name, &mut var.content);
                
                node._type = var._type;
                
                self.local_context
                    .define(
                        var.name,
                        VariableForResolve {
                            name: var.name,
                            _type: var._type,
                        },
                    )
                    .unwrap();
            }
    
            _ => (),
        }
    }
    
    fn process_function(&mut self, unit_name: &str, func: &mut IttFunction<'input>) {
        match func.body.node.as_mut() {
            IttExprs::Block(block) => {
                self.local_context.push_scope();
    
                func.args.iter().for_each(|arg| {
                    self.local_context
                        .define(
                            arg.0,
                            VariableForResolve {
                                name: arg.0,
                                _type: arg.1,
                            },
                        )
                        .unwrap();
                });
    
                block.iter_mut().for_each(|stmt| self.process_in_fn_expressions(unit_name, stmt));
                
                func.body._type = block.last().unwrap()._type;
                
                self.local_context.pop_scope();
            }
    
            _ => panic!(),
        }
    }
    
    pub fn process_tree(&mut self, tree: &mut TypedUnit<'input>)  {
        tree.unit_content.iter_mut().for_each(|expr| {
            match expr {
                IttDefinitions::Function(func) => self.process_function(&tree.unit_name, func),
                _ => ()
            }
        });
    }
}

