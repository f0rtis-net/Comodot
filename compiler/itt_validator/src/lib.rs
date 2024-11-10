use itt::{IttDefinitions, IttExprs, TypedNode, TypedUnit};
use itt_symbol_misc::func_table::FunctionSymbolTable;
use std::cell::RefCell;

pub struct IttTreeValidator<'input> {
    function_decls: &'input RefCell<FunctionSymbolTable<'input>>,
    unit: &'input TypedUnit<'input>
}

impl<'input> IttTreeValidator<'input> {
    pub fn new(
        function_decls: &'input RefCell<FunctionSymbolTable<'input>>, 
        unit: &'input TypedUnit<'input>
    ) -> Self {
        Self {
            function_decls,
            unit
        }
    }
    
    fn validate_node(&self, node: &'input TypedNode<'input>) {
        match node.node.as_ref() {
            IttExprs::Binary(binop) => {
                if binop.lhs._type != binop.rhs._type {
                    panic!("Type of left part of binop do not match with right part.");
                }
            }
            
            IttExprs::Block(block) => {
                block.iter().for_each(|stmt| {
                    self.validate_node(stmt);
                });
            }
            
            IttExprs::IfExpr(if_expr) => {
                self.validate_node(&if_expr.logic_condition);
                self.validate_node(&if_expr.if_block);
                
                if if_expr.else_block.is_some() {
                    self.validate_node(&if_expr.else_block.as_ref().unwrap());
                }
            }
            
            IttExprs::Return(expr) => {
                if expr.is_some() {
                    self.validate_node(expr.as_ref().unwrap());
                }
            }
            
            _ => ()
        }
    }
    
    pub fn validate_tree(&self) {
        self.unit.unit_content.iter().for_each(|stmt| {
            match stmt {
                IttDefinitions::Function(fun) => {
                    match fun.body.node.as_ref() {
                        IttExprs::Block(block) => {
                            block.iter().for_each(|expr| self.validate_node(expr));
                            
                            let ret_type = block.last().unwrap()._type;
                            if ret_type != fun.return_type { 
                                panic!("Return type of function signature not match to return type of function main block");
                            } 
                        }
                        _ => ()
                    }
                }
                _ => ()
            }
        });
    }
}