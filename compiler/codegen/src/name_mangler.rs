use itt::IttType;

pub fn mangle_function_name<'input>( 
    unit_name: &str, 
    name: &'input str, 
    arg_types: &Vec<IttType>, 
    ret_type: IttType) -> &'input str {
    
    if name == "main" {
        return name;
    }
        
    let mut new_name = String::from(format!("_ZN_{unit_name}_{name}_{}", short_type(&ret_type)));
    
    arg_types.iter().for_each(|arg| {
        new_name.push_str("_");
        new_name.push(short_type(arg));
    });

    Box::leak(new_name.into_boxed_str())
}

fn short_type(tp: &IttType) -> char {
    match tp {
        IttType::Int => 'i',
        IttType::Bool => 'b',
        IttType::Char => 'c',
        IttType::Float => 'f', 
        IttType::Custom => 'u',
        IttType::Void => 'v',
        IttType::String => 's',
        IttType::UNRESOLVED => panic!("")
    }
}