use ast::ParsedUnit;
use lalrpop_util::lalrpop_mod;
use lexer::Lexer;
lalrpop_mod!(parser);

pub struct Parser;

impl Parser {
    pub fn generate_parsed_unit_from_input<'input>(unit_name: &'input str, content: &'input str) -> ParsedUnit<'input> {
        let lexer = Lexer::new(content);
        
        let mut result: ParsedUnit<'input> = parser::UnitParser::new().parse(content, lexer).unwrap();
        
        result.unit_name = unit_name;
        
        result
    }
}