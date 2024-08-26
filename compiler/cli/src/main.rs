use beautiful_ast_printer::print_ast;
use parser::parser;

fn main() {
   let result = parser();

   print_ast(&result);
}
