pub mod symbol_table;

#[derive(Clone, Eq, PartialEq)]
pub enum IttTypes {
    INT,
    CHAR,
    FLOAT,
    BOOLEAN,
    CUSTOM,
    VOID
}

#[derive(Clone, Eq, PartialEq)]
pub enum IttVisible {
    GLOBAL,
    PRIVATE
}

#[derive(Clone)]
pub enum IttTreeRootNodes {
    Func(IttFunction),
    ConstantValue(IttVariable)
}

#[derive(Clone, Eq, PartialEq)]
pub enum IttOperators {
    PLUS, MINUS, MUL, DIV
}

#[derive(Clone)]
pub enum IttExpressions {
    Int(i64),
    Bool(bool),
    Char(char),
    Float(f64),
    Id(String),
    Variable(IttVariable),
    Binary(Box<IttExpressions>, Box<IttExpressions>, IttOperators, IttTypes),
    Return(Box<IttExpressions>, IttTypes),
    Call(Box<IttExpressions>, Vec<IttExpressions>)
}

#[derive(Clone)]
pub struct IttVariable {
    pub name: String,
    pub mutability: bool,
    pub visibility: IttVisible,
    pub _type: IttTypes,
    pub value: Option<Box<IttExpressions>>
}

#[derive(Clone)]
pub struct IttBlock {
    pub expressions: Vec<IttExpressions>,
    pub block_type: IttTypes
}

#[derive(Clone)]
pub struct IttFunction {
    pub name: String,
    pub arguments: Vec<IttVariable>,
    pub return_type: IttTypes,
    pub visibility: IttVisible,
    pub body: IttBlock
}

#[derive(Clone)]
pub struct IttByFileParametrizedTree {
    pub expressions: Vec<IttTreeRootNodes>,
    pub file_name: String,
    pub file_hash: String
}