use beautiful_ast_printer::print_ast;
use llvm_codegen::test_sum;

fn main() {
    test_sum().expect("Failed");

    print_ast();
}
