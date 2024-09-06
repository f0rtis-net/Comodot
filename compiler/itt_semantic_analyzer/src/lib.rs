use std::process::exit;
use itt::{IttByFileParametrizedTree, IttExpressions, IttFunction, IttTreeRootNodes, IttTypes, IttVariable};
use itt::IttTypes::VOID;

fn function_analyze(func: &IttFunction) {
    let func_ret_type = &func.return_type;
    let mut block_ret_type = &func.body.block_type;


    if *func_ret_type == VOID  {
        if *block_ret_type != VOID {
            println!("Function with return type VOID can't return value");
            exit(0);
        }
    } else {
        if *block_ret_type == VOID {
            println!("Function with return type!= VOID must return value");
            exit(0);
        }

        if *func_ret_type != *block_ret_type {
            println!("Function return type and return expression type are different");
            exit(0);
        }
    }
}

fn constant_analyze(constant: &IttVariable) {
    if constant.value.is_none() {
        println!("Constant must have value");
        exit(0);
    }
}

pub fn analyze(tree: &IttByFileParametrizedTree) {
    for expr in &tree.expressions {
        match expr {
            IttTreeRootNodes::Func(func) => function_analyze(func),
            IttTreeRootNodes::ConstantValue(variable) => constant_analyze(variable)
        }
    }
}