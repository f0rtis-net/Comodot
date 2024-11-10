pub mod keywords;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum Token<'input> {
    IDENTIFIER(&'input str),
    INTEGER(i64),
    FLOAT(f64),
    BOOL(bool),
    STR(&'input str),
    AND,
    OR,
    IF,
    ELSE,
    PLUS,
    MINUS,
    SLASH,
    STAR,
    LBRACKET,
    EXTERN,
    RBRACKET,
    LRBRACKET,
    RRBRACKET,
    SEMICOLON,
    FUNCTION,
    ASSIGN,
    GT, LT, EQ,
    CONST,
    PUBLIC,
    PRIVATE,
    RETURN,
    COMMA,
    COLON,
    IMPORT,
    URESOLVED,
    EXCLAMATION
}

