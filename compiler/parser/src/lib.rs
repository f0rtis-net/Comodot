use ast::ParsedFile;
use lalrpop_util::lalrpop_mod;
use lexer::Lexer;
lalrpop_mod!(parser);

pub fn parse_file<'a>(file_name: &'a str, content: &'a str) -> ParsedFile<'a> {
    let lexer = Lexer::new(content);
    
    let mut result = parser::UnitParser::new().parse(content, lexer).unwrap();
    
    result.name = file_name;
    
    result
}