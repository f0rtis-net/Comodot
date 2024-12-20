use tokens::Token;

#[derive(Debug)]
pub struct ParsedUnit<'input> {
    pub unit_hash: &'input str,
    pub unit_name: &'input str,
    pub unit_content: Vec<AstDefinitions<'input>>
}

#[derive(Debug, Clone)]
pub enum AstDefinitions<'input> {
    Function(AstFunction<'input>),
    Extern(ExternFnDeclaration<'input>),
    Import(ImportDirective<'input>)
}

#[derive(Debug, Clone)]
pub enum AstExpr<'input> {
    Identifier(&'input str), 
    Integer(i64), 
    Float(f64), 
    Bool(bool),
    String(&'input str),
    Block(Vec<AstExpr<'input>>), 
    Binary(BinaryExpression<'input>), 
    Call(CallExpression<'input>), 
    Return(Option<Box<AstExpr<'input>>>), 
    VarDef(VariableDefinition<'input>),
    IfExpr(IfExpression<'input>),
}

#[derive(Debug, Clone)]
pub struct ExternFnDeclaration<'input> {
    pub name: &'input str,
    pub args: Vec<(&'input str, Token<'input>)>,
    pub return_type: Token<'input>,
}

#[derive(Debug, Clone)]
pub struct ArrayDefinition<'input> {
    pub len: usize,
    pub name: &'input str,
    pub elements: Vec<Token<'input>>
}

#[derive(Debug, Clone)]
pub struct ArrayAccess<'input> {
    pub alias: &'input str,
    pub index: i64
}

#[derive(Debug, Clone)]
pub struct IfExpression<'input> {
    pub logic_condition: Box<AstExpr<'input>>,
    pub if_block: Box<AstExpr<'input>>,
    pub else_block: Option<Box<AstExpr<'input>>>
}

#[derive(Debug, Clone)]
pub struct ImportDirective<'input> {
    pub import_name: &'input str,
    pub import_hash: &'input str,
    pub target_found: &'input str
}

#[derive(Debug, Clone)]
pub struct VariableDefinition<'input> {
    pub name: &'input str,
    pub _type: Token<'input>,
    pub constant: bool,
    pub is_global: bool,
    pub content: Box<AstExpr<'input>>
}

#[derive(Debug, Clone)]
pub struct CallExpression<'input> {
    pub alias: Option<&'input str>,
    pub name: &'input str,
    pub args: Vec<AstExpr<'input>>
}

#[derive(Debug, Clone)]
pub struct BinaryExpression<'input> {
    pub lhs: Box<AstExpr<'input>>,
    pub rhs: Box<AstExpr<'input>>,
    pub operator: Token<'input>
}

#[derive(Debug, Clone)]
pub struct AstFunction<'input> {
    pub name: String,
    pub args: Vec<(&'input str, Token<'input>)>,
    pub return_type: Token<'input>,
    pub visibility: Token<'input>,
    pub body: Box<AstExpr<'input>>
}