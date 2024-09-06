pub mod keywords;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum Token {
    IDENTIFIER(&'static str),
    INTEGER(i64),
    REAL(f64),
    PLUS,
    MINUS,
    SLASH,
    STAR,
    LBRACKET,
    RBRACKET,
    LRBRACKET,
    RRBRACKET,
    SEMICOLON,
    FUNCTION,
    GT, LT, EQ,
    IntType,
    VoidType,
    CharType,
    RealType,
    BooleanType,
    UnknownType,
    CONST,
    PUBLIC,
    PRIVATE,
    RETURN
}

