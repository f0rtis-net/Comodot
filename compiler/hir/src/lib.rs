#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HirId(u64);

impl HirId {
    pub fn new() -> HirId {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        HirId(id)
    }
}

#[derive(Debug)]
pub enum HirVisibility {
    Public, 
    Private
}

#[derive(Debug)]
pub struct HirFile<'a> {
    pub name: &'a str,
    pub items: Vec<HirModuleItem<'a>>,
    pub imports: Vec<HirImport<'a>>, 
}

#[derive(Debug)]
pub struct HirImport<'a> {
    pub path: Vec<&'a str>,  
    pub alias: Option<&'a str>,
}

#[derive(Debug)]
pub enum HirModuleItem <'a> {
    Func {
        id: HirId,
        name: &'a str,
        args: Vec<(&'a str, HirId)>,
        body: HirExpr<'a>,
        visibility: HirVisibility
    }
}

#[derive(Debug, Clone)]
pub struct HirExpr<'a> {
    pub id: HirId,
    pub kind: HirExprKind<'a>,
}

#[derive(Debug, Clone, Copy)]
pub enum HirBinOps {
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
pub enum HirExprKind<'a> {
    Id(&'a str),
    Int(i64),
    Float(f64),
    Bool(bool),
    Char(char),

    Block(Vec<HirExpr<'a>>),

    Call {
        name: &'a str,
        args: Vec<HirExpr<'a>>,
    },
    Binary {
        op: HirBinOps,
        lhs: Box<HirExpr<'a>>,
        rhs: Box<HirExpr<'a>>
    },
    Return(Option<Box<HirExpr<'a>>>),
    If {
        cond: Box<HirExpr<'a>>,
        then: Box<HirExpr<'a>>,
        _else: Option<Box<HirExpr<'a>>>
    },
    VarDef {
        name: &'a str,
        value: Box<HirExpr<'a>>,
    }
}