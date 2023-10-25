use super::tokens::{Identifier, Keyword, Symbol};

pub(crate) type TermId = usize;

// Program structure
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum ClassVarKind {
    Static,
    Field,
}

impl ToString for ClassVarKind {
    fn to_string(&self) -> String {
        match *self {
            ClassVarKind::Static => "static".to_string(),
            ClassVarKind::Field => "field".to_string(),
        }
    }
}

#[derive(Debug)]
pub(crate) enum SubroutineKind {
    Constructor,
    Function,
    Method,
}

impl ToString for SubroutineKind {
    fn to_string(&self) -> String {
        match *self {
            SubroutineKind::Constructor => "constructor".to_string(),
            SubroutineKind::Function => "function".to_string(),
            SubroutineKind::Method => "method".to_string(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct VarDec<'a> {
    pub(crate) var_type: Type<'a>,
    pub(crate) name: Identifier<'a>,
}

#[derive(Debug)]
pub(crate) struct Param<'a> {
    pub(crate) p_type: Type<'a>,
    pub(crate) name: Identifier<'a>,
}

#[derive(Debug)]
pub(crate) enum SubroutineType<'a> {
    Void,
    Type(Type<'a>),
}

impl<'a> ToString for SubroutineType<'a> {
    fn to_string(&self) -> String {
        match *self {
            SubroutineType::Void => "void".to_string(),
            SubroutineType::Type(t) => t.to_string(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct SubroutineBody<'a> {
    pub(crate) var_decs: Vec<VarDec<'a>>,
    pub(crate) stmts: Vec<Statement<'a>>,
}

#[derive(Debug)]
pub(crate) struct SubroutineDec<'a> {
    pub(crate) kind: SubroutineKind,
    pub(crate) f_type: SubroutineType<'a>,
    pub(crate) name: Identifier<'a>,
    pub(crate) params: Vec<Param<'a>>,
    pub(crate) body: SubroutineBody<'a>,
}

#[derive(Debug)]
pub(crate) struct ClassVarDec<'a> {
    pub(crate) kind: ClassVarKind,
    pub(crate) var_dec: VarDec<'a>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum Type<'a> {
    Int,
    Char,
    Boolean,
    ClassName(Identifier<'a>),
}

impl<'a> ToString for Type<'a> {
    fn to_string(&self) -> String {
        match *self {
            Type::Int => "int".to_string(),
            Type::Char => "char".to_string(),
            Type::Boolean => "boolean".to_string(),
            Type::ClassName(s) => s.to_string(),
        }
    }
}

// Statements
#[derive(Debug)]
pub(crate) enum Statement<'a> {
    Let(LetStmt<'a>),
    If(IfStmt<'a>),
    While(WhileStmt<'a>),
    Do(DoStmt<'a>),
    Return(ReturnStmt),
}

#[derive(Debug)]
pub(crate) struct LetStmt<'a> {
    pub(crate) name: Identifier<'a>,
    pub(crate) idx: Option<Expression>,
    pub(crate) eq_to: Expression,
}

#[derive(Debug)]
pub(crate) struct IfStmt<'a> {
    pub(crate) cond: Expression,
    pub(crate) body: Vec<Statement<'a>>,
    pub(crate) else_body: Vec<Statement<'a>>,
}

#[derive(Debug)]
pub(crate) struct WhileStmt<'a> {
    pub(crate) cond: Expression,
    pub(crate) body: Vec<Statement<'a>>,
}

#[derive(Debug)]
pub(crate) struct DoStmt<'a> {
    pub(crate) call: SubroutineCall<'a>,
}

#[derive(Debug)]
pub(crate) struct ReturnStmt {
    pub(crate) ret_val: Option<Expression>,
}

#[derive(Debug)]
// Expressions
pub(crate) enum Op {
    Unknown,
    Plus,
    Minus,
    Multiply,
    Divide,
    And,
    Or,
    Less,
    Greater,
    Equal,
}

impl ToString for Op {
    fn to_string(&self) -> String {
        match &self {
            Op::Unknown => "unknown".to_string(),
            Op::Plus => "+".to_string(),
            Op::Minus => "-".to_string(),
            Op::Multiply => "*".to_string(),
            Op::Divide => "/".to_string(),
            Op::And => "&".to_string(),
            Op::Or => "|".to_string(),
            Op::Less => "<".to_string(),
            Op::Greater => ">".to_string(),
            Op::Equal => "=".to_string(),
        }
    }
}

impl From<Symbol> for Op {
    fn from(value: Symbol) -> Self {
        match value {
            Symbol::Plus => Op::Plus,
            Symbol::Minus => Op::Minus,
            Symbol::Multiply => Op::Multiply,
            Symbol::Divide => Op::Divide,
            Symbol::And => Op::And,
            Symbol::Or => Op::Or,
            Symbol::Less => Op::Less,
            Symbol::Greater => Op::Greater,
            Symbol::Equal => Op::Equal,
            _ => Op::Unknown,
        }
    }
}

#[derive(Debug)]
pub(crate) enum UnaryOp {
    Unknown,
    Minus,
    Not,
}

impl ToString for UnaryOp {
    fn to_string(&self) -> String {
        match *self {
            UnaryOp::Unknown => "unknown".to_string(),
            UnaryOp::Minus => "-".to_string(),
            UnaryOp::Not => "~".to_string(),
        }
    }
}

impl From<Symbol> for UnaryOp {
    fn from(value: Symbol) -> Self {
        match value {
            Symbol::Minus => UnaryOp::Minus,
            Symbol::Not => UnaryOp::Not,
            _ => UnaryOp::Unknown,
        }
    }
}

#[derive(Debug)]
pub(crate) enum KeywordConstant {
    Unknown,
    True,
    False,
    Null,
    This,
}

impl ToString for KeywordConstant {
    fn to_string(&self) -> String {
        match *self {
            KeywordConstant::Unknown => "unknown".to_string(),
            KeywordConstant::False => "false".to_string(),
            KeywordConstant::True => "true".to_string(),
            KeywordConstant::Null => "null".to_string(),
            KeywordConstant::This => "this".to_string(),
        }
    }
}

impl From<Keyword> for KeywordConstant {
    fn from(value: Keyword) -> Self {
        match value {
            Keyword::True => KeywordConstant::True,
            Keyword::False => KeywordConstant::False,
            Keyword::Null => KeywordConstant::Null,
            Keyword::This => KeywordConstant::This,
            _ => KeywordConstant::Unknown,
        }
    }
}

#[derive(Debug)]
pub(crate) struct UnaryTerm {
    pub(crate) op: UnaryOp,
    pub(crate) term: TermId,
}

#[derive(Debug)]
pub(crate) enum Term<'a> {
    Int(i32),
    String(&'a str),
    VarName(Identifier<'a>),
    KeywordConstant(KeywordConstant),
    ArrayAccess(ArrayAccess<'a>),
    Call(SubroutineCall<'a>),
    BracketExpression(Expression),
    Unary(UnaryTerm),
}

#[derive(Debug)]
pub(crate) struct SubroutineCall<'a> {
    pub(crate) caller: Option<Identifier<'a>>,
    pub(crate) name: Identifier<'a>,
    pub(crate) args: Vec<Expression>,
}

#[derive(Debug)]
pub(crate) struct ArrayAccess<'a> {
    pub(crate) var: Identifier<'a>,
    pub(crate) idx: Expression,
}

#[derive(Debug)]
pub(crate) struct Expression {
    pub(crate) init_term: TermId,
    pub(crate) ops: Vec<(Op, TermId)>,
}

#[derive(Default, Debug)]
pub(crate) struct ClassNode<'a> {
    pub(crate) name: Identifier<'a>,
    pub(crate) fields: Vec<ClassVarDec<'a>>,
    pub(crate) subroutines: Vec<SubroutineDec<'a>>,
}

#[derive(Debug)]
pub struct SyntaxTree<'a> {
    pub(crate) filename: String,
    pub(crate) terms: Vec<Term<'a>>,
    pub(crate) root: ClassNode<'a>,
}

impl<'a> SyntaxTree<'a> {
    pub(crate) fn new() -> SyntaxTree<'a> {
        SyntaxTree {
            filename: String::default(),
            terms: Vec::new(),
            root: Default::default(),
        }
    }
}

