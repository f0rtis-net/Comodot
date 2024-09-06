use lalrpop_util::lalrpop_mod;
use ast::misc::file::ParsedFile;
use lexer::Lexer;

lalrpop_mod!(parser);

pub fn parser() -> ParsedFile {
    let input = r#"
            func test() > Int {
                ret 2 + 2;
            }

            pub func main() > Int {
                ret (-3 + 3) + (4 + 3);
            }
        "#;

    let lexer = Lexer::new(&input);

    parser::FileParser::new().parse(&input, lexer).unwrap()
}

