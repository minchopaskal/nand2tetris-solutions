use super::tokens::{Keyword, Symbol, Token, TokenData};

pub(crate) type TermId = usize;
pub(crate) type IdentifierId = usize;

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
pub(crate) struct VarDec {
    pub(crate) var_type: Type,
    pub(crate) name: IdentifierId,
}

#[derive(Debug)]
pub(crate) struct Param {
    pub(crate) p_type: Type,
    pub(crate) name: IdentifierId,
}

#[derive(Debug)]
pub(crate) enum SubroutineType {
    Void,
    Type(Type),
}

impl ToString for SubroutineType {
    fn to_string(&self) -> String {
        match *self {
            SubroutineType::Void => "void".to_string(),
            SubroutineType::Type(t) => t.to_string(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct SubroutineBody {
    pub(crate) var_decs: Vec<VarDec>,
    pub(crate) stmts: Vec<Statement>,
}

#[derive(Debug)]
pub(crate) struct SubroutineDec {
    pub(crate) kind: SubroutineKind,
    pub(crate) f_type: SubroutineType,
    pub(crate) name: IdentifierId,
    pub(crate) params: Vec<Param>,
    pub(crate) body: SubroutineBody,
}

#[derive(Debug)]
pub(crate) struct ClassVarDec {
    pub(crate) kind: ClassVarKind,
    pub(crate) var_dec: VarDec,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum Type {
    Int,
    Char,
    Boolean,
    ClassName(IdentifierId),
}

impl ToString for Type {
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
pub(crate) enum Statement {
    Let(LetStmt),
    If(IfStmt),
    While(WhileStmt),
    Do(DoStmt),
    Return(ReturnStmt),
}

#[derive(Debug)]
pub(crate) struct LetStmt {
    pub(crate) name: IdentifierId,
    pub(crate) idx: Option<Expression>,
    pub(crate) eq_to: Expression,
}

#[derive(Debug)]
pub(crate) struct IfStmt {
    pub(crate) cond: Expression,
    pub(crate) body: Vec<Statement>,
    pub(crate) else_body: Vec<Statement>,
}

#[derive(Debug)]
pub(crate) struct WhileStmt {
    pub(crate) cond: Expression,
    pub(crate) body: Vec<Statement>,
}

#[derive(Debug)]
pub(crate) struct DoStmt {
    pub(crate) call: SubroutineCall,
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

#[derive(Debug, Clone, Copy)]
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
pub(crate) enum Term {
    Int(i32),
    String(IdentifierId),
    VarName(IdentifierId),
    KeywordConstant(KeywordConstant),
    ArrayAccess(ArrayAccess),
    Call(SubroutineCall),
    BracketExpression(Expression),
    Unary(UnaryTerm),
}

#[derive(Debug)]
pub(crate) struct SubroutineCall {
    pub(crate) caller: Option<IdentifierId>,
    pub(crate) name: IdentifierId,
    pub(crate) args: Vec<Expression>,
}

#[derive(Debug)]
pub(crate) struct ArrayAccess {
    pub(crate) var: IdentifierId,
    pub(crate) idx: Expression,
}

#[derive(Debug)]
pub(crate) struct Expression {
    pub(crate) init_term: TermId,
    pub(crate) ops: Vec<(Op, TermId)>,
}

#[derive(Default, Debug)]
pub(crate) struct ClassNode {
    pub(crate) name: IdentifierId,
    pub(crate) fields: Vec<ClassVarDec>,
    pub(crate) subroutines: Vec<SubroutineDec>,
}

#[derive(Debug, Default)]
pub struct SyntaxTree<'a> {
    pub(crate) filename: String,
    pub(crate) terms: Vec<Term>,
    pub(crate) root: ClassNode,
    pub(crate) tokens: Vec<Token<'a>>,
}

impl<'a> SyntaxTree<'a> {
    pub(crate) fn new() -> SyntaxTree<'a> {
        Default::default()
    }

    pub(crate) fn get_this() -> &'a str {
        "this"
    }

    pub(crate) fn get_type(&self) -> Type {
        Type::ClassName(self.root.name)
    }

    pub(crate) fn get_id(&self, id: IdentifierId) -> &str {
        if id == usize::MAX {
            return "this";
        }

        match self.tokens[id].data {
            TokenData::String(s) => s,
            TokenData::Identifier(s) => s,
            _ => panic!(
                "Internal error: expected string/identifier for token {} in {}",
                id, self.filename
            ),
        }
    }
}
