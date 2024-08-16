#[cfg(test)]
mod tests {
    use crate::Lexer;
    use crate::tokens::{NumberBase, TokenType};

    #[test]
    fn should_success_keywords() {
        let str = "fn let if else while for\nreturn";

        let mut lexer = Lexer::new(str);
        let mut counter = 0;

        loop {
            let token = lexer.process_token();

            if !token.is_ok() || token.as_ref().unwrap().is_none() {
                break
            }

            assert_eq!(token.unwrap().unwrap().type_, TokenType::KEYWORD);

            counter += 1;
        }

        assert_eq!(counter, 7);
    }

    #[test]
    fn should_success_reserved_symbols() {
        let str = "# ! | || & * ( ) { } -> - + / % \" ' = == > <";

        let token_types = [
            TokenType::GRID,
            TokenType::EXCLAMATION,
            TokenType::VerticalSlash,
            TokenType::OR,
            TokenType::AMPERSAND,
            TokenType::STAR,
            TokenType::LBRACE,
            TokenType::RBRACE,
            TokenType::LFBRACE,
            TokenType::RFBRACE,
            TokenType::ARROW,
            TokenType::MINUS,
            TokenType::PLUS,
            TokenType::SLASH,
            TokenType::PERCENT,
            TokenType::DoubleQuote,
            TokenType::QUOTE,
            TokenType::EQ,
            TokenType::DoubleEQ,
            TokenType::GT,
            TokenType::LT
        ];

        let mut lexer = Lexer::new(str);

        let mut counter = 0;

        loop {
            let token = lexer.process_token();

            if !token.is_ok() || token.as_ref().unwrap().is_none() {
                break
            }

            assert_eq!(token.unwrap().unwrap().type_, token_types[counter]);

            counter += 1;
        }

        assert_eq!(counter, token_types.len());
    }

    #[test]
    fn should_success_parse_numbers() {
        let str = "0 1 10_000 10 0x0A1 0b101 0o23";

        let basis_list = [
            NumberBase::DECIMAL,
            NumberBase::DECIMAL,
            NumberBase::DECIMAL,
            NumberBase::DECIMAL,
            NumberBase::HEX,
            NumberBase::BINARY,
            NumberBase::OCTAL
        ];

        let mut lexer = Lexer::new(str);
        let mut counter = 0;

        loop {
            let token = lexer.process_token();

            if !token.is_ok() || token.as_ref().unwrap().is_none() {
                break
            }

            let basis = match token.unwrap().unwrap().type_ {
                TokenType::NUMBER(base) => base,
                _ => panic!("Invalid type of parsed data")
            };

            assert_eq!(basis, basis_list[counter]);

            counter += 1;
        }

        assert_eq!(basis_list.len(), counter);
    }
}