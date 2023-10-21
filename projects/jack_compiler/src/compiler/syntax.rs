use super::tokens::{Identifier, TokenKind};

// Program structure
pub(crate) enum ClassVarKind {
    Static,
    Field,
}

pub(crate) enum SubroutineKind {
    Constructor,
    Function,
    Method,
}

pub(crate) struct VarDec<'a> {
    pub(crate) v_type: Type<'a>,
    pub(crate) names: Vec<Identifier<'a>>,
}

pub(crate) struct Param<'a> {
    pub(crate) p_type: Type<'a>,
    pub(crate) name: Identifier<'a>,
}

pub(crate) enum SubroutineType<'a> {
    Void,
    Type(Type<'a>),
}

pub(crate) struct SubroutineBody<'a> {
    pub(crate) var_decs: Vec<VarDec<'a>>,
    pub(crate) stmts: Vec<Statement<'a>>,
}

pub(crate) struct SubroutineDec<'a> {
    pub(crate) kind: SubroutineKind,
    pub(crate) f_type: SubroutineType<'a>,
    pub(crate) name: Identifier<'a>,
    pub(crate) params: Vec<Param<'a>>,
    pub(crate) body: SubroutineBody<'a>,
}

pub(crate) struct ClassVarDec<'a> {
    pub(crate) kind: ClassVarKind,
    pub(crate) var_type: Type<'a>,
    pub(crate) names: Vec<Identifier<'a>>,
}

pub(crate) enum Type<'a> {
    Int,
    Char,
    Boolean,
    ClassName(&'a str)
}

// Statements
pub(crate) enum Statement<'a> {
    Let(LetStmt<'a>),
    If(IfStmt<'a>),
    While(WhileStmt<'a>),
    Do(DoStmt<'a>),
    Return(ReturnStmt<'a>),
}

pub(crate) struct LetStmt<'a> {
    pub(crate) name: Identifier<'a>,
    pub(crate) idx: Option<Expression<'a>>,
    pub(crate) eq_to: Expression<'a>,
}

pub(crate) struct IfStmt<'a> {
    pub(crate) cond: Expression<'a>,
    pub(crate) body: Vec<Statement<'a>>,
    pub(crate) else_body: Vec<Statement<'a>>,
}

pub(crate) struct WhileStmt<'a> {
    pub(crate) cond: Expression<'a>,
    pub(crate) body: Vec<Statement<'a>>,
}

pub(crate) struct DoStmt<'a> {
    pub(crate) call: SubroutineCall<'a>,
}

pub(crate) struct ReturnStmt<'a> {
    pub(crate) ret_val: Option<Expression<'a>>,
}

// Expressions
pub(crate) enum Op {
    Plus,
    Minus,
    Multiply,
    Divide,
    And,
    Or,
    Less,
    Greater,
    Equal
}

pub(crate) enum UnaryOp {
    Minus,
    Not,
}

pub(crate) enum KeywordConstant {
    True,
    False,
    Null,
    This,
}

pub(crate) struct UnaryTerm<'a> {
    pub(crate) op: UnaryOp,
    pub(crate) term: &'a Term<'a>,
}

pub(crate) enum TermKind<'a> {
    Primitive(&'a TokenKind<'a>), // Maybe it's bad to reuse TokenKind here
    ArrayAccess(ArrayAccess<'a>),
    Call(SubroutineCall<'a>),
    BracketExpression(Expression<'a>),
    Unary(UnaryTerm<'a>)
}

pub(crate) struct SubroutineCall<'a> {
    pub(crate) caller: Option<Identifier<'a>>,
    pub(crate) name: Identifier<'a>,
    pub(crate) args: Vec<Expression<'a>>,
}

pub(crate) struct Term<'a> {
    pub(crate) kind: TermKind<'a>,
}

pub(crate) struct ArrayAccess<'a> {
    pub(crate) var: Identifier<'a>,
    pub(crate) idx: Expression<'a>,
}

pub(crate) struct Expression<'a> {
    pub(crate) init_term: &'a Term<'a>,
    pub(crate) ops: Vec<(Op, &'a Term<'a>)>,
}

#[derive(Default)]
pub(crate) struct ClassNode<'a> {
    pub(crate) name: Identifier<'a>,
    pub(crate) fields: Vec<ClassVarDec<'a>>,
    pub(crate) subroutines: Vec<SubroutineDec<'a>>,
}

pub struct SyntaxTree<'a> {
    terms: Vec<Term<'a>>,
    pub(crate) root: ClassNode<'a>,
}

impl<'a> SyntaxTree<'a> {
    pub(crate) fn new() -> SyntaxTree<'a> {
        SyntaxTree { 
            terms: Vec::new(),
            root: Default::default(),
        }
    }
}