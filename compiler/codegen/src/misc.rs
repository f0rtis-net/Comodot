use inkwell::{context::Context, types::{BasicType, BasicTypeEnum},  AddressSpace};
use itt::IttType;

pub(crate) fn get_llvm_type<'input>(context: &'input Context,basic_type: &IttType) -> BasicTypeEnum<'input> {
    match basic_type {
        IttType::Int => context.i64_type().as_basic_type_enum(),
        IttType::Char => context.i8_type().as_basic_type_enum(),
        IttType::Float => context.f64_type().as_basic_type_enum(),
        IttType::Bool => context.bool_type().as_basic_type_enum(),
        IttType::String => context.ptr_type(AddressSpace::from(0)).as_basic_type_enum(),
        
        _ => panic!("Unsupported type"),
    }
}