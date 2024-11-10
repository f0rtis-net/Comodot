use inkwell::{values::{BasicValue, BasicValueEnum}, FloatPredicate, IntPredicate};
use itt::{IttBinaryOperations, IttType};
use inkwell::builder::Builder;

fn build_sum<'input>(builder: &'input Builder, _type: &IttType, lhs: BasicValueEnum<'input>, rhs: BasicValueEnum<'input>) -> BasicValueEnum<'input> {
    match _type {
        IttType::Int| IttType::Char => 
            builder.build_int_add(lhs.into_int_value(), rhs.into_int_value(), "int_add")
            .unwrap().as_basic_value_enum(),
        IttType::Float => 
            builder.build_float_add(lhs.into_float_value(), rhs.into_float_value(), "float_add")
            .unwrap().as_basic_value_enum(),
        _ => panic!("{:?}", _type)
    }
}

fn build_sub<'input>(builder: &'input Builder, _type: &IttType, lhs: BasicValueEnum<'input>, rhs: BasicValueEnum<'input>) -> BasicValueEnum<'input> {
    match _type {
        IttType::Int | IttType::Char => 
            builder.build_int_sub(lhs.into_int_value(), rhs.into_int_value(), "int_sub")
            .unwrap().as_basic_value_enum(),
        IttType::Float => 
            builder.build_float_sub(lhs.into_float_value(), rhs.into_float_value(), "float_sub")
            .unwrap().as_basic_value_enum(),
        _ => panic!("{:?}", _type)
    }
}

fn build_div<'input>(builder: &'input Builder, _type: &IttType, lhs: BasicValueEnum<'input>, rhs: BasicValueEnum<'input>) -> BasicValueEnum<'input> {
    match _type {
        IttType::Int | IttType::Char => 
            builder.build_int_signed_div(lhs.into_int_value(), rhs.into_int_value(), "int_div")
            .unwrap().as_basic_value_enum(),
        IttType::Float => 
            builder.build_float_div(lhs.into_float_value(), rhs.into_float_value(), "float_div")
            .unwrap().as_basic_value_enum(),
        _ => panic!("")
    }
}

fn build_mul<'input>(builder: &'input Builder, _type: &IttType, lhs: BasicValueEnum<'input>, rhs: BasicValueEnum<'input>) -> BasicValueEnum<'input> {
    match _type {
        IttType::Int | IttType::Char => 
            builder.build_int_mul(lhs.into_int_value(), rhs.into_int_value(), "int_mul")
            .unwrap().as_basic_value_enum(),
        IttType::Float => 
            builder.build_float_mul(lhs.into_float_value(), rhs.into_float_value(), "float_mul")
            .unwrap().as_basic_value_enum(),
        _ => panic!("{:?}", _type)
    }
}

fn build_gt_compare<'input>(builder: &'input Builder, _type: &IttType, lhs: BasicValueEnum<'input>, rhs: BasicValueEnum<'input>) -> BasicValueEnum<'input> {
    match _type {
        IttType::Int | IttType::Char => builder.build_int_compare(IntPredicate::SGT, lhs.into_int_value(), rhs.into_int_value(), "cmpres").unwrap().as_basic_value_enum(),
        IttType::Float => builder.build_float_compare(FloatPredicate::OGT, lhs.into_float_value(), rhs.into_float_value(), "cmpres").unwrap().as_basic_value_enum(),
        _ => panic!("{:?}", _type)
    }
}

fn build_lt_compare<'input>(builder: &'input Builder, _type: &IttType, lhs: BasicValueEnum<'input>, rhs: BasicValueEnum<'input>) -> BasicValueEnum<'input> {
    match _type {
        IttType::Int | IttType::Char => builder.build_int_compare(IntPredicate::SLT, lhs.into_int_value(), rhs.into_int_value(), "cmpres").unwrap().as_basic_value_enum(),
        IttType::Float => builder.build_float_compare(FloatPredicate::OLT, lhs.into_float_value(), rhs.into_float_value(), "cmpres").unwrap().as_basic_value_enum(),
        _ => panic!("{:?}", _type)
    }
}

fn build_eq_compare<'input>(builder: &'input Builder, _type: &IttType, lhs: BasicValueEnum<'input>, rhs: BasicValueEnum<'input>) -> BasicValueEnum<'input> {
    match _type {
        IttType::Int | IttType::Bool | IttType::Char => builder.build_int_compare(IntPredicate::EQ, lhs.into_int_value(), rhs.into_int_value(), "cmpres").unwrap().as_basic_value_enum(),
        IttType::Float => builder.build_float_compare(FloatPredicate::UEQ, lhs.into_float_value(), rhs.into_float_value(), "cmpres").unwrap().as_basic_value_enum(),
        _ => panic!("{:?}", _type)
    }
}

fn build_and<'input>(builder: &'input Builder, _type: &IttType, lhs: BasicValueEnum<'input>, rhs: BasicValueEnum<'input>) -> BasicValueEnum<'input> {
    match _type {
        IttType::Bool => builder.build_and(lhs.into_int_value(), rhs.into_int_value(), "ssl_and").unwrap().as_basic_value_enum(),
        _ => panic!("Invalid type to process [and] operation.")
    }
}

fn build_or<'input>(builder: &'input Builder, _type: &IttType, lhs: BasicValueEnum<'input>, rhs: BasicValueEnum<'input>) -> BasicValueEnum<'input> {
    match _type {
        IttType::Bool => builder.build_or(lhs.into_int_value(), rhs.into_int_value(), "ssl_or").unwrap().as_basic_value_enum(),
        _ => panic!("Invalid type to process [or] operation.")
    }
}

pub(crate) fn build_llvm_binop<'input>(
    builder: &'input Builder,
    lhs: BasicValueEnum<'input>,
    rhs: BasicValueEnum<'input>,
    op: &IttBinaryOperations,
    binary_ops_type: &IttType
) -> BasicValueEnum<'input> {
    match op {
        IttBinaryOperations::SUM => build_sum(builder, binary_ops_type, lhs, rhs),
        IttBinaryOperations::SUB => build_sub(builder, binary_ops_type, lhs, rhs),
        IttBinaryOperations::DIV => build_div(builder, binary_ops_type, lhs, rhs),
        IttBinaryOperations::MUL => build_mul(builder, binary_ops_type, lhs, rhs),
        IttBinaryOperations::AND => build_and(builder, binary_ops_type, lhs, rhs),
        IttBinaryOperations::OR => build_or(builder, binary_ops_type, lhs, rhs),
        IttBinaryOperations::GT => build_gt_compare(builder, binary_ops_type, lhs, rhs),
        IttBinaryOperations::LT => build_lt_compare(builder, binary_ops_type, lhs, rhs),
        IttBinaryOperations::EQ => build_eq_compare(builder, binary_ops_type, lhs, rhs)
    }
}
