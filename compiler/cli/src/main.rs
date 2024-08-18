use ast::expressions::function_literal::FunctionLiteral;
use ast::expressions::integer_literal::IntegerLiteral;
use ast::misc::file::ParsedFile;
use ast::primitives::expression::Expression;
use ast::primitives::statement::Statement;
use ast::statements::block_statement::BlockStatement;
use ast::statements::return_statement::ReturnStatement;
use beautiful_ast_printer::print_ast;
use llvm_codegen::test_generation;
use tokens::Keyword;
use tokens::TokenType::KEYWORD;

fn main() {
   let main_function = FunctionLiteral {
      name: String::from("main"),
      body: BlockStatement {
         statements: vec![
            ReturnStatement {
               value: Some(IntegerLiteral {
                  value: 12,
               }.clone_boxed()),
            }.clone_boxed()
         ],
      },
      return_type: KEYWORD(Keyword::VOID),
      visibility: false,
   };

   let file = ParsedFile::new(
      String::from("test file"),
      String::from("LOLOLO"), String::from("test"),
      vec![main_function.clone_boxed()]
   );

   print_ast(&file);

   test_generation(&file).expect("TODO: panic message");
}
