use std::borrow::Cow;

#[derive(Debug, Clone)]
pub enum Primitive {
    Int,
    Float,
    Char, 
    Bool,
    Unit
}

#[derive(Debug, Clone)]
pub enum LangType {
    UNRESOLVED,
    HINT(String),
    Primitives(Primitive)
}

impl LangType {
    pub fn is_primitive(&self) -> bool {
        match self {
            LangType::Primitives(_) => true,
            _ => false
        }
    }

    pub fn is_unit(&self) -> bool {
        match self {
            LangType::Primitives(Primitive::Unit) => true,
            _ => false
        }
    }

    pub fn short_text(&self) -> Cow<str> {
        let res = match self {
            LangType::Primitives(Primitive::Int) => "i",
            LangType::Primitives(Primitive::Float) => "f",
            LangType::Primitives(Primitive::Char) => "c",
            LangType::Primitives(Primitive::Bool) => "b",
            LangType::Primitives(Primitive::Unit) => "u",
            LangType::UNRESOLVED => "UNRESOLVED",
            LangType::HINT(s) => s
        };

        Cow::from(res)
    }
}