use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::Keyword;

lazy_static! {
    pub static ref RESERVED_KEYWORDS: HashMap<&'static str, Keyword>  = HashMap::from([
        ("void", Keyword::VOID),
        ("if", Keyword::IF),
        ("else", Keyword::ELSE),

    ]);
}