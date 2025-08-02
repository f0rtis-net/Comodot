use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Primitive {
    Int,
    Float,
    Char, 
    Bool,
    Unit
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LangType {
    UNRESOLVED,
    Primitives(Primitive),
    StaticArray {
        size: u64,
        ty: Box<LangType>
    }
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
        match self {
            LangType::Primitives(Primitive::Int) => "i".into(),
            LangType::Primitives(Primitive::Float) => "f".into(),
            LangType::Primitives(Primitive::Char) => "c".into(),
            LangType::Primitives(Primitive::Bool) => "b".into(),
            LangType::Primitives(Primitive::Unit) => "u".into(),
            LangType::StaticArray{ty, size} => format!("[{};{}]", ty.short_text(), size).into(),
            LangType::UNRESOLVED => "unresolved".into()
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            LangType::UNRESOLVED => vec![0],
  
            LangType::Primitives(p) => {
                let mut bytes = vec![2];
                bytes.push(p.to_byte());
                bytes
            },
            LangType::StaticArray{ty, size} => {
                let mut bytes = vec![3];
                bytes.extend_from_slice(ty.to_bytes().as_slice());
                bytes.extend_from_slice(&size.to_le_bytes());
                
                bytes
            }
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        match bytes.get(0)? {
            0 => Some(LangType::UNRESOLVED),
            2 => match bytes.get(1)? {
                0 => Some(LangType::Primitives(Primitive::Int)),
                1 => Some(LangType::Primitives(Primitive::Float)),
                2 => Some(LangType::Primitives(Primitive::Char)),
                3 => Some(LangType::Primitives(Primitive::Bool)),
                4 => Some(LangType::Primitives(Primitive::Unit)),
                _ => None,
            },
            _ => None,
        }
    }
}

impl Primitive {
    pub fn to_byte(&self) -> u8 {
        match self {
            Primitive::Int => 0,
            Primitive::Float => 1,
            Primitive::Char => 2,
            Primitive::Bool => 3,
            Primitive::Unit => 4,
        }
    }
}