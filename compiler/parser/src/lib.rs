use ast::ParsedUnit;
use lalrpop_util::lalrpop_mod;
use lexer::Lexer;
lalrpop_mod!(parser);

pub fn generate_parsed_unit_from_input<'a>(unit_name: &'a str, content: &'a str) -> ParsedUnit<'a> {
    let lexer = Lexer::new(content);
    
    let mut result = parser::UnitParser::new().parse(content, lexer).unwrap();
    
    result.unit_name = unit_name;
    
    result
}