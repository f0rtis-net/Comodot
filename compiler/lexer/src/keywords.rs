use lazy_static::lazy_static;

lazy_static! {
    pub static ref KEYWORDS: Vec<&'static str> = vec![
        "fn",
        "let",
        "if",
        "else",
        "while",
        "for",
        "in",
        "return",
        "break",
        "continue",
        "struct",
        "enum",
        "impl",
        "use",
        "as",
        "match",
        "true",
        "false",
        "self",
        "static",
        "mut",
        "const",
        "unsafe",
        "extern",
        "pub",
        "println",
        "print"
    ];
}