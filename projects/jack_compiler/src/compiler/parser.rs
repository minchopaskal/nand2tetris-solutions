use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use super::lexer::Lexer;
use super::syntax::{
    ArrayAccess, ClassNode, ClassVarDec, ClassVarKind, Expression, KeywordConstant, Op, Param,
    Statement, SubroutineBody, SubroutineCall, SubroutineDec, SubroutineKind, SubroutineType,
    SyntaxTree, Term, Type, UnaryTerm, VarDec,
};
use super::tokens::{Identifier, Keyword, Symbol, Token, TokenData, TokenKind};

#[macro_export]
macro_rules! return_internal {
    () => {
        return Err(ParseError {
            message: format!("Internal error at {}:{}", file!(), line!()),
        })
    };
}

#[derive(Debug)]
pub struct ParseError {
    message: String,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ParseError {}

pub struct Parser<'a> {
    pub filename: String,

    source: Vec<String>,
    lexer: Lexer,
    tokens: Vec<Token<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(path: PathBuf) -> Parser<'a> {
        let filename = path.file_stem().unwrap().to_str().unwrap();

        let mut source = Vec::new();
        let in_file = File::open(&path).unwrap();
        let reader = BufReader::new(in_file);
        for line in reader.lines() {
            let line = line.unwrap_or("".to_string());
            source.push(line);
        }

        Parser {
            filename: filename.to_string(),
            source,
            lexer: Lexer::new(filename.to_string()),
            tokens: Default::default(),
        }
    }

    pub fn parse(&'a mut self) -> Result<SyntaxTree<'a>, ParseError> {
        self.tokens = self.lexer.lex(&self.source);

        let mut terms = Vec::new();
        let mut ptr = 0;
        let root_node_res = self.parse_root(&mut terms, &mut ptr);

        let mut tree = SyntaxTree::new();
        tree.root = if let Ok(node) = root_node_res {
            node
        } else {
            return Err(root_node_res.err().unwrap());
        };
        tree.terms = terms;

        println!("{:?}", tree.root);
        println!(
            "{:?}",
            tree.terms
                .iter()
                .zip(0..tree.terms.len())
                .collect::<Vec<(&Term<'a>, usize)>>()
        );

        Ok(tree)
    }

    fn parse_root(
        &'a self,
        terms: &mut Vec<Term<'a>>,
        ptr: &mut usize,
    ) -> Result<ClassNode<'a>, ParseError> {
        self.expect_keyword(Keyword::Class, ptr)?;
        let _ = self.expect(TokenKind::Identifier, ptr)?;
        let class_name = if let TokenData::Identifier(name) = self.tokens[*ptr - 1].data {
            name
        } else {
            panic!("Expected class name identifier!");
        };
        self.expect_symbol(Symbol::LeftCurly, ptr)?;

        let var_dec = self.parse_classvardec(ptr)?;
        let subroutine_dec = self.parse_subroutinedec(terms, ptr)?;

        Ok(ClassNode {
            name: class_name,
            fields: var_dec,
            subroutines: subroutine_dec,
        })
    }

    fn parse_type(&'a self, ptr: &mut usize) -> Result<Type<'a>, ParseError> {
        self.expect_any(vec![TokenKind::Identifier, TokenKind::Keyword], ptr)?;
        let curr_tok = &self.tokens[*ptr - 1];
        let var_type = match curr_tok.data {
            TokenData::Identifier(id) => Type::ClassName(id),
            TokenData::Keyword(kw) => match kw {
                Keyword::Boolean => Type::Boolean,
                Keyword::Char => Type::Char,
                Keyword::Int => Type::Int,
                _ => {
                    return Err(ParseError {
                        message: format!(
                            "Unexpected keyword {:?} in type at {}:{}",
                            kw, curr_tok.file, curr_tok.line
                        ),
                    });
                }
            },
            _ => {
                return_internal!();
            }
        };

        Ok(var_type)
    }

    fn parse_vardec(&'a self, ptr: &mut usize) -> Result<Vec<VarDec<'a>>, ParseError> {
        let mut res = Vec::new();

        let var_type = self.parse_type(ptr)?;
        loop {
            let name = self.parse_name(ptr)?;

            res.push(VarDec { var_type, name });

            if let TokenData::Symbol(Symbol::Semicolon) = self.peek(ptr)? {
                break;
            }
            self.expect_symbol(Symbol::Comma, ptr)?;
        }

        self.expect_symbol(Symbol::Semicolon, ptr)?;

        Ok(res)
    }

    fn parse_classvardec(&'a self, ptr: &mut usize) -> Result<Vec<ClassVarDec<'a>>, ParseError> {
        let mut res = Vec::new();

        let mut tok = self.peek(ptr)?;
        if let TokenData::Symbol(Symbol::RightCurly) = tok {
            return Ok(res);
        }

        while let TokenData::Keyword(kw) = tok {
            if *kw == Keyword::Constructor || *kw == Keyword::Method || *kw == Keyword::Function {
                break;
            }

            match *kw {
                Keyword::Constructor | Keyword::Method | Keyword::Function => {
                    break;
                }
                _ => (),
            }

            let kind_tok = self.expect_any_keyword(vec![Keyword::Field, Keyword::Static], ptr)?;
            let curr_tok = &self.tokens[*ptr - 1];
            let kind = match kind_tok {
                Keyword::Field => ClassVarKind::Field,
                Keyword::Static => ClassVarKind::Static,
                _ => {
                    return Err(ParseError {
                        message: format!(
                            "Unexpected keyword {:?} in class var dec at {}:{}",
                            kind_tok, curr_tok.file, curr_tok.line
                        ),
                    });
                }
            };
            let var_decs = self.parse_vardec(ptr)?;
            for var_dec in var_decs {
                res.push(ClassVarDec { kind, var_dec });
            }
            tok = self.peek(ptr)?;
        }

        Ok(res)
    }

    fn parse_subroutinedec(
        &'a self,
        terms: &mut Vec<Term<'a>>,
        ptr: &mut usize,
    ) -> Result<Vec<SubroutineDec<'a>>, ParseError> {
        let mut res = Vec::new();

        let mut tok = self.peek(ptr)?;
        loop {
            if let TokenData::Symbol(Symbol::RightCurly) = tok {
                break;
            }

            let func_kind = self.expect_any_keyword(
                vec![Keyword::Constructor, Keyword::Function, Keyword::Method],
                ptr,
            )?;
            let kind = match func_kind {
                Keyword::Constructor => SubroutineKind::Constructor,
                Keyword::Function => SubroutineKind::Function,
                Keyword::Method => SubroutineKind::Method,
                _ => {
                    return_internal!();
                }
            };

            let f_type = {
                if let Ok(t) = self.parse_type(ptr) {
                    SubroutineType::Type(t)
                } else {
                    self.revert(ptr)?;
                    self.expect_keyword(Keyword::Void, ptr)?;
                    SubroutineType::Void
                }
            };

            let name = self.parse_name(ptr)?;

            self.expect_symbol(Symbol::LeftRound, ptr)?;
            let params = self.parse_parameter_list(ptr)?;
            self.expect_symbol(Symbol::RightRound, ptr)?;

            let body = self.parse_subroutine_body(terms, ptr)?;

            res.push(SubroutineDec {
                kind,
                f_type,
                name,
                params,
                body,
            });

            tok = self.peek(ptr)?;
        }

        Ok(res)
    }

    fn parse_parameter_list(&'a self, ptr: &mut usize) -> Result<Vec<Param<'a>>, ParseError> {
        let mut res = Vec::new();

        if let TokenData::Symbol(Symbol::RightRound) = self.peek(ptr)? {
            return Ok(res);
        }

        loop {
            let p_type = self.parse_type(ptr)?;
            let name = self.parse_name(ptr)?;

            res.push(Param { p_type, name });

            if let TokenData::Symbol(Symbol::Comma) = self.tokens[*ptr].data {
                self.advance(ptr)?;
                continue;
            }

            break;
        }

        Ok(res)
    }

    fn parse_statements(
        &'a self,
        terms: &mut Vec<Term<'a>>,
        ptr: &mut usize,
    ) -> Result<Vec<Statement<'a>>, ParseError> {
        let mut stmts = Vec::new();

        while let TokenData::Keyword(kw) = self.peek(ptr)? {
            match *kw {
                Keyword::Let => {
                    stmts.push(self.parse_let(terms, ptr)?);
                }
                Keyword::If => {
                    stmts.push(self.parse_if(terms, ptr)?);
                }
                Keyword::While => {
                    stmts.push(self.parse_while(terms, ptr)?);
                }
                Keyword::Do => {
                    stmts.push(self.parse_do(terms, ptr)?);
                }
                Keyword::Return => {
                    stmts.push(self.parse_return(terms, ptr)?);
                }
                _ => {
                    break;
                }
            }
        }

        Ok(stmts)
    }

    fn parse_subroutine_body(
        &'a self,
        terms: &mut Vec<Term<'a>>,
        ptr: &mut usize,
    ) -> Result<SubroutineBody<'a>, ParseError> {
        self.expect_symbol(Symbol::LeftCurly, ptr)?;

        let mut var_decs = Vec::new();
        while let TokenData::Keyword(Keyword::Var) = self.peek(ptr)? {
            self.advance(ptr)?;
            let mut vars = self.parse_vardec(ptr)?;
            var_decs.append(&mut vars);
        }

        let stmts = self.parse_statements(terms, ptr)?;

        self.expect_symbol(Symbol::RightCurly, ptr)?;
        Ok(SubroutineBody { var_decs, stmts })
    }

    fn parse_let(
        &'a self,
        terms: &mut Vec<Term<'a>>,
        ptr: &mut usize,
    ) -> Result<Statement<'a>, ParseError> {
        self.expect_keyword(Keyword::Let, ptr)?;
        let name = self.parse_name(ptr)?;

        let idx = if self.tokens[*ptr].data == TokenData::Symbol(Symbol::LeftSquare) {
            self.advance(ptr)?;
            let expr = self.parse_expression(terms, ptr)?;
            self.expect_symbol(Symbol::RightSquare, ptr)?;

            Some(expr)
        } else {
            None
        };

        self.expect_symbol(Symbol::Equal, ptr)?;

        let eq_to = self.parse_expression(terms, ptr)?;

        self.expect_symbol(Symbol::Semicolon, ptr)?;

        Ok(Statement::Let(super::syntax::LetStmt { name, idx, eq_to }))
    }

    fn parse_if(
        &'a self,
        terms: &mut Vec<Term<'a>>,
        ptr: &mut usize,
    ) -> Result<Statement<'a>, ParseError> {
        self.expect_keyword(Keyword::If, ptr)?;
        self.expect_symbol(Symbol::LeftRound, ptr)?;
        let cond = self.parse_expression(terms, ptr)?;
        self.expect_symbol(Symbol::RightRound, ptr)?;
        self.expect_symbol(Symbol::LeftCurly, ptr)?;
        let body = self.parse_statements(terms, ptr)?;
        self.expect_symbol(Symbol::RightCurly, ptr)?;

        let else_body = if let TokenData::Keyword(Keyword::Else) = self.tokens[*ptr].data {
            self.advance(ptr)?;
            self.expect_symbol(Symbol::LeftCurly, ptr)?;
            let stmts = self.parse_statements(terms, ptr)?;
            self.expect_symbol(Symbol::RightCurly, ptr)?;

            stmts
        } else {
            Vec::new()
        };

        Ok(Statement::If(super::syntax::IfStmt {
            cond,
            body,
            else_body,
        }))
    }

    fn parse_while(
        &'a self,
        terms: &mut Vec<Term<'a>>,
        ptr: &mut usize,
    ) -> Result<Statement<'a>, ParseError> {
        self.expect_keyword(Keyword::While, ptr)?;
        self.expect_symbol(Symbol::LeftRound, ptr)?;
        let cond = self.parse_expression(terms, ptr)?;
        self.expect_symbol(Symbol::RightRound, ptr)?;
        self.expect_symbol(Symbol::LeftCurly, ptr)?;
        let body = self.parse_statements(terms, ptr)?;
        self.expect_symbol(Symbol::RightCurly, ptr)?;

        Ok(Statement::While(super::syntax::WhileStmt { cond, body }))
    }

    fn parse_do(
        &'a self,
        terms: &mut Vec<Term<'a>>,
        ptr: &mut usize,
    ) -> Result<Statement<'a>, ParseError> {
        self.expect_keyword(Keyword::Do, ptr)?;
        let call = self.parse_subroutine_call(terms, ptr)?;
        self.expect_symbol(Symbol::Semicolon, ptr)?;

        Ok(Statement::Do(super::syntax::DoStmt { call }))
    }

    fn parse_return(
        &'a self,
        terms: &mut Vec<Term<'a>>,
        ptr: &mut usize,
    ) -> Result<Statement<'a>, ParseError> {
        self.expect_keyword(Keyword::Return, ptr)?;
        if let TokenData::Symbol(Symbol::Semicolon) = self.tokens[*ptr].data {
            self.advance(ptr)?;
            return Ok(Statement::Return(super::syntax::ReturnStmt {
                ret_val: None,
            }));
        }

        let ret_val = Some(self.parse_expression(terms, ptr)?);
        Ok(Statement::Return(super::syntax::ReturnStmt { ret_val }))
    }

    fn parse_subroutine_call(
        &'a self,
        terms: &mut Vec<Term<'a>>,
        ptr: &mut usize,
    ) -> Result<SubroutineCall<'a>, ParseError> {
        let name = self.parse_name(ptr)?;
        let (name, caller) = if let TokenData::Symbol(Symbol::Dot) = self.tokens[*ptr].data {
            self.advance(ptr)?;
            let n = self.parse_name(ptr)?;
            (n, Some(name))
        } else {
            (name, None)
        };
        self.expect_symbol(Symbol::LeftRound, ptr)?;

        let mut args = Vec::new();
        if let TokenData::Symbol(Symbol::RightRound) = self.tokens[*ptr].data {
            self.advance(ptr)?;
            return Ok(SubroutineCall { caller, name, args });
        }
        loop {
            args.push(self.parse_expression(terms, ptr)?);
            if let TokenData::Symbol(Symbol::RightRound) = self.tokens[*ptr].data {
                break;
            }
            self.expect_symbol(Symbol::Comma, ptr)?;
        }
        self.expect_symbol(Symbol::RightRound, ptr)?;

        Ok(SubroutineCall { caller, name, args })
    }

    fn parse_expression(
        &'a self,
        terms: &mut Vec<Term<'a>>,
        ptr: &mut usize,
    ) -> Result<Expression, ParseError> {
        let init_term = self.parse_term(terms, ptr)?;

        let mut ops = Vec::new();
        while self.is_op(ptr)? {
            let op = self.parse_op(ptr)?;
            ops.push((op, self.parse_term(terms, ptr)?));
        }

        Ok(Expression { init_term, ops })
    }

    fn is_op(&self, ptr: &mut usize) -> Result<bool, ParseError> {
        if *ptr >= self.tokens.len() {
            return Err(ParseError {
                message: format!("Reading tokens after EOF!"),
            });
        }

        if let TokenData::Symbol(s) = self.tokens[*ptr].data {
            match s {
                Symbol::Plus
                | Symbol::Minus
                | Symbol::Multiply
                | Symbol::Divide
                | Symbol::And
                | Symbol::Or
                | Symbol::Less
                | Symbol::Greater
                | Symbol::Equal => Ok(true),
                _ => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    fn parse_op(&'a self, ptr: &mut usize) -> Result<Op, ParseError> {
        if *ptr >= self.tokens.len() {
            return Err(ParseError {
                message: format!("Reading tokens after EOF!"),
            });
        }

        let curr = *ptr;
        *ptr += 1;

        if let TokenData::Symbol(s) = self.tokens[curr].data {
            let op = s.into();
            match op {
                Op::Unknown => return_internal!(),
                _ => Ok(op),
            }
        } else {
            return_internal!();
        }
    }

    fn parse_term(
        &'a self,
        terms: &mut Vec<Term<'a>>,
        ptr: &mut usize,
    ) -> Result<usize, ParseError> {
        self.advance(ptr)?;
        let curr_token = &self.tokens[*ptr - 1];
        match curr_token.data {
            TokenData::Keyword(kw) => {
                if let KeywordConstant::Unknown = kw.into() {
                    return Err(ParseError {
                        message: format!(
                            "Unexpected keyword {:?} in term at {}:{}",
                            kw, curr_token.file, curr_token.line
                        ),
                    });
                } else {
                    terms.push(Term::KeywordConstant(kw.into()));
                }
            }
            TokenData::Symbol(s) => {
                match s {
                    Symbol::Minus | Symbol::Not => {
                        let op = s.into();
                        let term = self.parse_term(terms, ptr)?;
                        terms.push(Term::Unary(UnaryTerm { op, term }));
                    }
                    Symbol::LeftRound => {
                        let expr = self.parse_expression(terms, ptr)?;
                        self.expect_symbol(Symbol::RightRound, ptr)?;
                        terms.push(Term::BracketExpression(expr));
                    }
                    _ => {
                        return Err(ParseError {
                            message: format!(
                                "Unexpected symbol {:?} in expression at {}:{}",
                                s, curr_token.file, curr_token.line
                            ),
                        })
                    }
                };
            }
            TokenData::Int(i) => {
                terms.push(Term::Int(i));
            }
            TokenData::String(s) => {
                terms.push(Term::String(s));
            }
            TokenData::Identifier(id) => {
                if let Ok(s) = self.expect_any_symbol(
                    vec![Symbol::LeftSquare, Symbol::LeftRound, Symbol::Dot],
                    ptr,
                ) {
                    match s {
                        Symbol::LeftSquare => {
                            let expr = self.parse_expression(terms, ptr)?;
                            self.expect_symbol(Symbol::RightSquare, ptr)?;
                            terms.push(Term::ArrayAccess(ArrayAccess { var: id, idx: expr }));
                        }
                        Symbol::LeftRound | Symbol::Dot => {
                            self.revert(ptr)?;
                            self.revert(ptr)?;
                            let call = self.parse_subroutine_call(terms, ptr)?;
                            terms.push(Term::Call(call));
                        }
                        _ => {
                            return_internal!();
                        }
                    }
                } else {
                    self.revert(ptr)?;
                    terms.push(Term::VarName(id));
                }
            }
        }

        Ok(terms.len() - 1)
    }

    fn parse_name(&'a self, ptr: &mut usize) -> Result<Identifier<'a>, ParseError> {
        self.expect(TokenKind::Identifier, ptr)?;
        if let TokenData::Identifier(id) = self.tokens[*ptr - 1].data {
            Ok(id)
        } else {
            return_internal!();
        }
    }

    // Token iteration:
    fn expect(&self, kind: TokenKind, ptr: &mut usize) -> Result<(), ParseError> {
        if *ptr >= self.tokens.len() {
            return Err(ParseError {
                message: format!("Expected {:?}, after reaching end of file", kind),
            });
        }

        let curr = *ptr;
        *ptr += 1;

        let tk: TokenKind = (&self.tokens[curr].data).into();
        if tk != kind {
            return Err(ParseError {
                message: format!(
                    "Expected {:?}, but got {:?} at {}:{}",
                    kind, self.tokens[curr].data, self.tokens[curr].file, self.tokens[curr].line
                ),
            });
        }

        Ok(())
    }

    fn expect_any(&self, kinds: Vec<TokenKind>, ptr: &mut usize) -> Result<TokenKind, ParseError> {
        assert!(!kinds.is_empty());

        if *ptr >= self.tokens.len() {
            return Err(ParseError {
                message: format!("Expected one of {:?}, after reaching end of file", kinds),
            });
        }

        let curr = *ptr;
        *ptr += 1;

        let tk: TokenKind = (&self.tokens[curr].data).into();
        for kind in kinds.iter() {
            if tk == *kind {
                return Ok(tk);
            }
        }

        let tok = &self.tokens[curr];
        return Err(ParseError {
            message: format!(
                "Expected one of {:?}, got {:?} in {}:{}",
                kinds, tk, tok.file, tok.line
            ),
        });
    }

    fn expect_any_keyword(
        &self,
        kws: Vec<Keyword>,
        ptr: &mut usize,
    ) -> Result<Keyword, ParseError> {
        if kws.is_empty() {
            return_internal!();
        }

        if *ptr >= self.tokens.len() {
            return Err(ParseError {
                message: format!("Expected one of {:?}, after reaching end of file", kws),
            });
        }

        let curr = *ptr;
        *ptr += 1;

        if let TokenData::Keyword(k) = self.tokens[curr].data {
            for kw in kws.iter() {
                if k == *kw {
                    return Ok(k);
                }
            }
        }

        let tok = &self.tokens[curr];
        return Err(ParseError {
            message: format!(
                "Expected any of {:?}, got {:?} in {}:{}",
                kws, tok.data, tok.file, tok.line
            ),
        });
    }

    fn expect_any_symbol(
        &self,
        symbols: Vec<Symbol>,
        ptr: &mut usize,
    ) -> Result<Symbol, ParseError> {
        if symbols.is_empty() {
            return_internal!();
        }

        if *ptr >= self.tokens.len() {
            return Err(ParseError {
                message: format!("Expected one of {:?}, after reaching end of file", symbols),
            });
        }

        let curr = *ptr;
        *ptr += 1;

        if let TokenData::Symbol(s) = self.tokens[curr].data {
            for sym in symbols.iter() {
                if s == *sym {
                    return Ok(s);
                }
            }
        }

        let tok = &self.tokens[curr];
        return Err(ParseError {
            message: format!(
                "Expected any of {:?}, got {:?} in {}:{}",
                symbols, tok.data, tok.file, tok.line
            ),
        });
    }

    fn expect_keyword(&self, kw: Keyword, ptr: &mut usize) -> Result<(), ParseError> {
        if *ptr >= self.tokens.len() {
            return Err(ParseError {
                message: format!("Expected {:?}, after reaching end of file", kw),
            });
        }

        let curr = *ptr;
        *ptr += 1;

        if let TokenData::Keyword(w) = self.tokens[curr].data {
            if w != kw {
                return Err(ParseError {
                    message: format!(
                        "Expected {:?}, but got {:?} at {}:{}",
                        kw, self.tokens[curr].data, self.tokens[curr].file, self.tokens[curr].line
                    ),
                });
            }
        } else {
            return Err(ParseError {
                message: format!(
                    "Expected {:?}, but got {:?} at {}:{}",
                    kw, self.tokens[curr].data, self.tokens[curr].file, self.tokens[curr].line
                ),
            });
        }

        Ok(())
    }

    fn expect_symbol(&self, sym: Symbol, ptr: &mut usize) -> Result<(), ParseError> {
        if *ptr >= self.tokens.len() {
            return Err(ParseError {
                message: format!("Expected {:?}, after reaching end of file", sym),
            });
        }

        let curr = *ptr;
        *ptr += 1;

        if let TokenData::Symbol(s) = self.tokens[curr].data {
            if s != sym {
                return Err(ParseError {
                    message: format!(
                        "Expected {:?}, but got {:?} at {}:{}",
                        sym, self.tokens[curr].data, self.tokens[curr].file, self.tokens[curr].line
                    ),
                });
            }
        } else {
            return Err(ParseError {
                message: format!(
                    "Expected {:?}, but got {:?} at {}:{}",
                    sym, self.tokens[curr].data, self.tokens[curr].file, self.tokens[curr].line
                ),
            });
        }

        Ok(())
    }

    fn advance(&self, ptr: &mut usize) -> Result<(), ParseError> {
        if *ptr >= self.tokens.len() {
            return Err(ParseError {
                message: format!("Advance tokens after EOF."),
            });
        }

        *ptr += 1;

        Ok(())
    }

    fn revert(&self, ptr: &mut usize) -> Result<(), ParseError> {
        if *ptr == 0 {
            return Err(ParseError {
                message: format!("Revert first token"),
            });
        }

        *ptr -= 1;

        Ok(())
    }

    fn peek(&self, ptr: &mut usize) -> Result<&TokenData<'a>, ParseError> {
        if *ptr >= self.tokens.len() {
            return Err(ParseError {
                message: format!("Peek tokens after EOF."),
            });
        }

        //Ok((&self.tokens[*ptr].data).into())
        Ok(&self.tokens[*ptr].data)
    }
}
