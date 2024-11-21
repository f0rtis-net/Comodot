use itt::IttType;

pub fn type_to_char_translator(_type: &IttType) -> Result<char, String> {
    match _type {
        IttType::Int => Ok('i'),
        IttType::Bool => Ok('b'),
        IttType::Char => Ok('c'),
        IttType::Custom => Ok('u'),
        IttType::Void => Ok('v'),
        IttType::Float => Ok('f'),
        IttType::String => Ok('s'),
        _ => Err(String::from("Invalid type to short"))
    }
}

pub fn mangle_function_name(module_name: &str, name: &str, arguments: &Vec<IttType>) -> Result<String, String> {
    let mut arg_prefix = String::new();
    
    if name == "main" {
        return Ok(String::from("language_main"));
    }
    
    
    for arg in arguments.iter() {
        arg_prefix.push('_');
        arg_prefix.push(type_to_char_translator(arg)?);
    }
    
    Ok(format!("__Nm_{module_name}_f_{name}{arg_prefix}"))
}