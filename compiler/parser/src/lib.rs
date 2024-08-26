use lalrpop_util::lalrpop_mod;
use ast::misc::file::ParsedFile;
use lexer::Lexer;

lalrpop_mod!(parser);

pub fn parser() -> ParsedFile {
    let input = "(10 - 1) / 2";

    let lexer = Lexer::new(&input);
    //let mut next_tok = || lexer.next().unwrap().ok().unwrap().0;

    parser::FileParser::new().parse(&input, lexer).unwrap()
}