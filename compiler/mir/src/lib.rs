use ast::{AstDefinitions, AstExpr, ParsedUnit};
use tokens::Token;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IttType {
    Void, 
    Int, 
    Char,
    Float, 
    Bool,
    String,
    Custom,
    UNRESOLVED
}

#[derive(Debug, Clone, Copy)]
pub enum IttVisibility {
    GLOBAL,
    FILE
}

#[derive(Debug, Clone, Copy)]
pub enum IttBinaryOperations {
    SUM,
    DIV,
    MUL,
    SUB,
    AND,
    OR,
    LT,
    GT,
    EQ
}

#[derive(Debug, Clone)]
pub struct TypedUnit<'input> {
    pub unit_hash: &'input str,
    pub unit_name: &'input str,
    pub unit_content: Vec<IttDefinitions<'input>>
}

#[derive(Debug, Clone)]
pub enum IttDefinitions<'input> {
    Function(IttFunction<'input>),
    Extern(IttExternFnDeclaration<'input>),
}

#[derive(Debug, Clone, Copy)]
pub struct IttImportDirective<'input> {
    pub import_name: &'input str,
    pub import_hash: &'input str,
    pub target_found: &'input str
}

#[derive(Debug, Clone)]
pub struct IttExternFnDeclaration<'input> {
    pub name: &'input str,
    pub args: Vec<(&'input str, IttType)>,
    pub return_type: IttType,
}

#[derive(Debug, Clone)]
pub enum IttExprs<'input> {
    Identifier(&'input str),
    Integer(i64),
    Float(f64),
    Bool(bool),
    String(&'input str),
    Block(Vec<TypedNode<'input>>),
    Binary(IttBinaryExpression<'input>),
    Call(IttCallExpression<'input>),
    Return(Option<TypedNode<'input>>),
    VarDef(IttVariableDefinition<'input>),
    IfExpr(IttIfExpression<'input>)
}

#[derive(Debug, Clone)]
pub struct IttIfExpression<'input> {
    pub logic_condition: TypedNode<'input>,
    pub if_block: TypedNode<'input>,
    pub else_block: Option<TypedNode<'input>>
}

#[derive(Debug, Clone)]
pub struct IttVariableDefinition<'input> {
    pub name: &'input str,
    pub _type: IttType,
    pub constant: bool,
    pub is_global: bool,
    pub content: TypedNode<'input>
}

#[derive(Debug, Clone)]
pub struct IttCallExpression<'input> {
    pub alias: Option<&'input str>,
    pub name: &'input str,
    pub args: Vec<TypedNode<'input>>
}

#[derive(Debug, Clone)]
pub struct IttBinaryExpression<'input> {
    pub lhs: TypedNode<'input>,
    pub rhs: TypedNode<'input>,
    pub operator: IttBinaryOperations
}

#[derive(Debug, Clone)]
pub struct IttFunction<'input> {
    pub name: &'input str,
    pub args: Vec<(&'input str, IttType)>,
    pub return_type: IttType,
    pub visibility: IttVisibility,
    pub body: TypedNode<'input>
}

#[derive(Debug, Clone)]
pub struct TypedNode<'input> {
    pub _type: IttType,
    pub node: Box<IttExprs<'input>>
}

pub struct IttTreeBuilder<'input> {
    name: &'input str
}

impl<'input> IttTreeBuilder<'input> {    
    pub fn new() -> Self {
        Self {
            name: "test"
        }
    }
    
    fn translate_to_itt_type(&self, ast_def: &Token<'input>) -> IttType {
        match ast_def {
            Token::IDENTIFIER("Void") => IttType::Void,
            Token::IDENTIFIER("Int") => IttType::Int,
            Token::IDENTIFIER("Char") => IttType::Char,
            Token::IDENTIFIER("Bool") => IttType::Bool,
            Token::IDENTIFIER("Float") => IttType::Float,
            Token::IDENTIFIER("String") => IttType::String,
            _ => IttType::UNRESOLVED
        }
    }
    
    fn translate_to_itt_visibility(&self, ast_def: &Token<'input>) -> IttVisibility {
        match ast_def {
            Token::PRIVATE => IttVisibility::FILE,
            Token::PUBLIC => IttVisibility::GLOBAL,
            _ => panic!("invalid visibility")
        }
    }
    
    fn translate_to_itt_binop(&self, ast_def: &Token<'input>) -> IttBinaryOperations {
        match ast_def {
            Token::PLUS => IttBinaryOperations::SUM,
            Token::MINUS => IttBinaryOperations::SUB,
            Token::STAR => IttBinaryOperations::MUL,
            Token::SLASH => IttBinaryOperations::DIV,
            Token::AND => IttBinaryOperations::AND,
            Token::OR => IttBinaryOperations::OR,
            Token::LT => IttBinaryOperations::LT,
            Token::GT => IttBinaryOperations::GT,
            Token::EQ => IttBinaryOperations::EQ,
            _ => panic!("invalid binary operation")
        }
    }
    
    pub fn translate_node(&self, node: &AstExpr<'input>) -> TypedNode<'input> {
        match node {
            AstExpr::Identifier(id) => TypedNode {_type: IttType::UNRESOLVED, node: Box::new(IttExprs::Identifier(id))},
            AstExpr::Integer(n) => TypedNode {_type: IttType::Int, node: Box::new(IttExprs::Integer(*n))},
            AstExpr::Float(n) => TypedNode {_type: IttType::Float, node: Box::new(IttExprs::Float(*n))},
            AstExpr::Bool(n) => TypedNode {_type: IttType::Bool, node: Box::new(IttExprs::Bool(*n))},
            AstExpr::String(val) => TypedNode {_type: IttType::String, node: Box::new(IttExprs::String(val))},
            
            AstExpr::IfExpr(if_expr) => {
                let cond = self.translate_node(&if_expr.logic_condition);
                let if_block = self.translate_node(&if_expr.if_block);
                
                let else_block = if if_expr.else_block.is_some() {
                    Some(self.translate_node(if_expr.else_block.as_ref().unwrap()))
                } else {
                    None
                };
                
                TypedNode {
                    _type: if_block._type,
                    node: Box::new(IttExprs::IfExpr(IttIfExpression {
                        logic_condition: cond,
                        if_block,
                        else_block
                    }))
                }
            }
            
            AstExpr::Binary(binexpr) => {
                let l_t = self.translate_node(&binexpr.lhs);
                let r_t = self.translate_node(&binexpr.rhs);
                
                let translated_operator = self.translate_to_itt_binop(&binexpr.operator);
                
                TypedNode {
                    _type: l_t._type,
                    node: Box::new(IttExprs::Binary(IttBinaryExpression {
                        lhs: l_t,
                        rhs: r_t,
                        operator: translated_operator
                    }))
                }
            }
            
            AstExpr::Return(ret_expr) => {
                let mut itt_ret_expr: Option<TypedNode> = None;
                let mut itt_ret_type = IttType::Void;
                
                if ret_expr.is_some() {
                    let translated = self.translate_node(ret_expr.as_ref().unwrap());
                    itt_ret_expr = Some(translated.clone());
                    itt_ret_type = translated._type;
                }           
                
                TypedNode {
                    _type: itt_ret_type,
                    node: Box::new(IttExprs::Return(itt_ret_expr))
                }
            },
            
            AstExpr::Call(call) => {
                let transformed_args = call.args.iter().map(|arg| self.translate_node(arg)).collect();
                
                let transformed_call = IttExprs::Call(IttCallExpression{
                    alias: call.alias,
                    name: call.name,
                    args: transformed_args
                });
                
                TypedNode {
                    _type: IttType::UNRESOLVED,
                    node: Box::new(transformed_call)
                }
            }
            
            AstExpr::VarDef(def) => {
                let translated_content = self.translate_node(&def.content);
                
                let transformed_node = IttExprs::VarDef(IttVariableDefinition{
                    name: def.name,
                    _type: self.translate_to_itt_type(&def._type),
                    constant: def.constant,
                    is_global: def.is_global,
                    content: translated_content.clone(),
                });
                
                TypedNode {
                    _type: self.translate_to_itt_type(&def._type),
                    node: Box::new(transformed_node)
                }
            }
            
            AstExpr::Block(block) => {
                let translated_block: Vec<TypedNode> = block.iter().map(|expr| self.translate_node(expr)).collect();
                
                TypedNode {
                    _type: translated_block.last().unwrap()._type,
                    node: Box::new(IttExprs::Block(translated_block))
                }
            }
        }
    }
    
    pub fn translate(&self, ast: &'input ParsedUnit<'input>) -> TypedUnit<'input> {
        let mut result = TypedUnit {
            unit_hash: ast.unit_hash,
            unit_name: ast.unit_name,
            unit_content: vec![]
        };
        
        ast.unit_content.iter().for_each(|ast_exprs| {
            match ast_exprs {
                AstDefinitions::Extern(ext_fn) => {
                    let translated_args = ext_fn.args.iter().map(|arg| {
                        (arg.0, self.translate_to_itt_type(&arg.1))
                    }).collect();
                    
                    let translated_ret_type = self.translate_to_itt_type(&ext_fn.return_type);
                    
                    result.unit_content.push(IttDefinitions::Extern(IttExternFnDeclaration { 
                        name: ext_fn.name, 
                        args: translated_args, 
                        return_type: translated_ret_type 
                    }));
                }
                
                AstDefinitions::Function(function) => {
                    let translated_args = function.args.iter().map(|arg| {
                        (arg.0, self.translate_to_itt_type(&arg.1))
                    }).collect();
                    
                    let translated_ret_type = self.translate_to_itt_type(&function.return_type);
                    
                    let translated_visibility = self.translate_to_itt_visibility(&function.visibility);
                    
                    let translated_body = self.translate_node(&function.body);
                    
                    result.unit_content.push(IttDefinitions::Function(IttFunction{
                        name: &function.name,
                        args: translated_args,
                        return_type: translated_ret_type,
                        visibility: translated_visibility,
                        body: translated_body
                    }));
                }
                
                _ => panic!()
            }
        });
        
        result
    }
}