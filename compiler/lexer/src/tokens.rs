#[derive(Clone, Debug, PartialEq, Copy)]
pub enum NumberBase {
    BINARY,
    DECIMAL,
    HEX,
    OCTAL
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum TokenType {
    NUMBER(NumberBase),
    KEYWORD,
    IDENTIFIER,
    UNDEFINED,
    GRID,
    EXCLAMATION,
    AMPERSAND,
    OR,
    VerticalSlash,
    STAR,
    LBRACE,
    RBRACE,
    LFBRACE,
    RFBRACE,
    COMMA,
    PLUS,
    MINUS,
    SLASH,
    PERCENT,
    DoubleQuote,
    QUOTE,
    ARROW,
    LT,
    GT,
    EQ,
    DoubleEQ,
    AND,
    DotComma,
    DoubleDot,
    RStraitBrace,
    LStraitBrace,
    UpArrow,
    TILDA,
    BackSlash
}

pub struct Position {
    pub line: i32,
    pub column: i32,
}

pub struct Token{
    pub type_: TokenType,
    pub position: Position,
    pub value: String,
}
