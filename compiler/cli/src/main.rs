use itt::symbol_table::{GlobalSymbolTable, SymbolTableTypes};
use itt::{IttBlock, IttByFileParametrizedTree, IttExpressions, IttFunction, IttOperators, IttTreeRootNodes, IttTypes, IttVisible};
use itt_semantic_analyzer::analyze;
use llvm_codegen::test_generation;

fn main() {
   /*let result = parser();

   print_ast(&result);

   test_generation(&result).unwrap();*/
   
   let return_expr = IttExpressions::Return(
       Box::new(IttExpressions::Binary(
           Box::new(IttExpressions::Int(-1)),
           Box::new(IttExpressions::Int(2)),
           IttOperators::MUL,
           IttTypes::INT
       )), 
      IttTypes::INT
   );

   let block = IttBlock {
      expressions: vec![return_expr],
      block_type: IttTypes::INT
   };

   let func = IttFunction {
      name: "test1".to_string(),
      arguments: vec![],
      return_type: IttTypes::INT,
      visibility: IttVisible::GLOBAL,
      body: block,
   };

   let return_expr1 = IttExpressions::Return(
       Box::new(IttExpressions::Call(
          Box::new(IttExpressions::Id("test1".to_string())),
          vec![],
       )),
      IttTypes::INT
   );

   let block1 = IttBlock {
      expressions: vec![return_expr1],
      block_type: IttTypes::INT
   };

   let func1 = IttFunction {
      name: "main".to_string(),
      arguments: vec![],
      return_type: IttTypes::INT,
      visibility: IttVisible::GLOBAL,
      body: block1,
   };

   let file = IttByFileParametrizedTree {
      expressions: vec![IttTreeRootNodes::Func(func1.clone()), IttTreeRootNodes::Func(func.clone())],
      file_name: "lol".to_string(),
      file_hash: "a7wda7d8aa".to_string(),
   };
   
   let mut global_table = GlobalSymbolTable::new();
   
   global_table.define_module("lol".to_string());
   
   global_table.get_module_table("lol").add_to_current_scope("test1".to_string(), SymbolTableTypes::Function(func));
   
   global_table.get_module_table("lol").add_to_current_scope("main".to_string(), SymbolTableTypes::Function(func1));

   analyze(&file);

   test_generation(&file, &mut global_table).unwrap();
}
