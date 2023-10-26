use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use super::lexer::Lexer;
use super::syntax::{
    ArrayAccess, ClassNode, ClassVarDec, ClassVarKind, Expression, IdentifierId, KeywordConstant,
    Op, Param, Statement, SubroutineBody, SubroutineCall, SubroutineDec, SubroutineKind,
    SubroutineType, SyntaxTree, Term, Type, UnaryTerm, VarDec,
};
use super::tokens::{Keyword, Symbol, Token, TokenData, TokenKind};

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

struct ParserData<'a> {
    tokens: Vec<Token<'a>>,
    terms: Vec<Term>,
    ptr: usize,
}

pub struct Parser {
    pub filename: String,

    source: Vec<String>,
    lexer: Lexer,
    //tokens: Vec<Token>,
}

impl Parser {
    pub fn new(path: PathBuf) -> Parser {
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
            //tokens: Default::default(),
        }
    }

    pub fn parse(&mut self) -> Result<SyntaxTree, ParseError> {
        let mut d = ParserData {
            tokens: self.lexer.lex(&self.source),
            terms: Vec::new(),
            ptr: 0,
        };
        let root_node_res = self.parse_root(&mut d);

        let mut tree = SyntaxTree::new();
        tree.filename = self.filename.clone();
        tree.root = if let Ok(node) = root_node_res {
            node
        } else {
            return Err(root_node_res.err().unwrap());
        };
        tree.terms = d.terms;
        tree.tokens = d.tokens;

        println!("{:?}", tree.root);
        for (i, term) in tree.terms.iter().enumerate() {
            println!("{i} - {term:?}");
        }
        for (i, tok) in tree.tokens.iter().enumerate() {
            println!("{i} - {tok:?}");
        }

        Ok(tree)
    }

    fn parse_root(&self, d: &mut ParserData) -> Result<ClassNode, ParseError> {
        self.expect_keyword(Keyword::Class, d)?;
        self.expect(TokenKind::Identifier, d)?;
        let class_name = if let TokenData::Identifier(_name) = d.tokens[d.ptr - 1].data {
            d.ptr - 1
        } else {
            panic!("Expected class name identifier!");
        };
        self.expect_symbol(Symbol::LeftCurly, d)?;

        let var_dec = self.parse_classvardec(d)?;
        let subroutine_dec = self.parse_subroutinedec(d)?;

        Ok(ClassNode {
            name: class_name,
            fields: var_dec,
            subroutines: subroutine_dec,
        })
    }

    fn parse_type(&self, d: &mut ParserData) -> Result<Type, ParseError> {
        self.expect_any(vec![TokenKind::Identifier, TokenKind::Keyword], d)?;
        let curr_tok = &d.tokens[d.ptr - 1];
        let var_type = match curr_tok.data {
            TokenData::Identifier(_id) => Type::ClassName(d.ptr - 1),
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

    fn parse_vardec(&self, d: &mut ParserData) -> Result<Vec<VarDec>, ParseError> {
        let mut res = Vec::new();

        let var_type = self.parse_type(d)?;
        loop {
            let name = self.parse_name(d)?;

            res.push(VarDec { var_type, name });

            if let TokenData::Symbol(Symbol::Semicolon) = d.tokens[d.ptr].data {
                break;
            }
            self.expect_symbol(Symbol::Comma, d)?;
        }

        self.expect_symbol(Symbol::Semicolon, d)?;

        Ok(res)
    }

    fn parse_classvardec(&self, d: &mut ParserData) -> Result<Vec<ClassVarDec>, ParseError> {
        let mut res = Vec::new();

        let mut tok = &d.tokens[d.ptr].data;
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

            let kind_tok = self.expect_any_keyword(vec![Keyword::Field, Keyword::Static], d)?;
            let curr_tok = &d.tokens[d.ptr - 1];
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
            let var_decs = self.parse_vardec(d)?;
            for var_dec in var_decs {
                res.push(ClassVarDec { kind, var_dec });
            }
            tok = &d.tokens[d.ptr].data;
        }

        Ok(res)
    }

    fn parse_subroutinedec(&self, d: &mut ParserData) -> Result<Vec<SubroutineDec>, ParseError> {
        let mut res = Vec::new();

        let mut tok = &d.tokens[d.ptr].data;
        loop {
            if let TokenData::Symbol(Symbol::RightCurly) = tok {
                break;
            }

            let func_kind = self.expect_any_keyword(
                vec![Keyword::Constructor, Keyword::Function, Keyword::Method],
                d,
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
                if let Ok(t) = self.parse_type(d) {
                    SubroutineType::Type(t)
                } else {
                    self.revert(d)?;
                    self.expect_keyword(Keyword::Void, d)?;
                    SubroutineType::Void
                }
            };

            let name = self.parse_name(d)?;

            self.expect_symbol(Symbol::LeftRound, d)?;
            let params = self.parse_parameter_list(d)?;
            self.expect_symbol(Symbol::RightRound, d)?;

            let body = self.parse_subroutine_body(d)?;

            res.push(SubroutineDec {
                kind,
                f_type,
                name,
                params,
                body,
            });

            tok = &d.tokens[d.ptr].data;
        }

        Ok(res)
    }

    fn parse_parameter_list(&self, d: &mut ParserData) -> Result<Vec<Param>, ParseError> {
        let mut res = Vec::new();

        if let TokenData::Symbol(Symbol::RightRound) = d.tokens[d.ptr].data {
            return Ok(res);
        }

        loop {
            let p_type = self.parse_type(d)?;
            let name = self.parse_name(d)?;

            res.push(Param { p_type, name });

            if let TokenData::Symbol(Symbol::Comma) = d.tokens[d.ptr].data {
                self.advance(d)?;
                continue;
            }

            break;
        }

        Ok(res)
    }

    fn parse_statements(&self, d: &mut ParserData) -> Result<Vec<Statement>, ParseError> {
        let mut stmts = Vec::new();

        while let TokenData::Keyword(kw) = &d.tokens[d.ptr].data {
            match *kw {
                Keyword::Let => {
                    stmts.push(self.parse_let(d)?);
                }
                Keyword::If => {
                    stmts.push(self.parse_if(d)?);
                }
                Keyword::While => {
                    stmts.push(self.parse_while(d)?);
                }
                Keyword::Do => {
                    stmts.push(self.parse_do(d)?);
                }
                Keyword::Return => {
                    stmts.push(self.parse_return(d)?);
                }
                _ => {
                    break;
                }
            }
        }

        Ok(stmts)
    }

    fn parse_subroutine_body(&self, d: &mut ParserData) -> Result<SubroutineBody, ParseError> {
        self.expect_symbol(Symbol::LeftCurly, d)?;

        let mut var_decs = Vec::new();
        while let TokenData::Keyword(Keyword::Var) = d.tokens[d.ptr].data {
            self.advance(d)?;
            let mut vars = self.parse_vardec(d)?;
            var_decs.append(&mut vars);
        }

        let stmts = self.parse_statements(d)?;

        self.expect_symbol(Symbol::RightCurly, d)?;
        Ok(SubroutineBody { var_decs, stmts })
    }

    fn parse_let(&self, d: &mut ParserData) -> Result<Statement, ParseError> {
        self.expect_keyword(Keyword::Let, d)?;
        let name = self.parse_name(d)?;

        let idx = if d.tokens[d.ptr].data == TokenData::Symbol(Symbol::LeftSquare) {
            self.advance(d)?;
            let expr = self.parse_expression(d)?;
            self.expect_symbol(Symbol::RightSquare, d)?;

            Some(expr)
        } else {
            None
        };

        self.expect_symbol(Symbol::Equal, d)?;

        let eq_to = self.parse_expression(d)?;

        self.expect_symbol(Symbol::Semicolon, d)?;

        Ok(Statement::Let(super::syntax::LetStmt { name, idx, eq_to }))
    }

    fn parse_if(&self, d: &mut ParserData) -> Result<Statement, ParseError> {
        self.expect_keyword(Keyword::If, d)?;
        self.expect_symbol(Symbol::LeftRound, d)?;
        let cond = self.parse_expression(d)?;
        self.expect_symbol(Symbol::RightRound, d)?;
        self.expect_symbol(Symbol::LeftCurly, d)?;
        let body = self.parse_statements(d)?;
        self.expect_symbol(Symbol::RightCurly, d)?;

        let else_body = if let TokenData::Keyword(Keyword::Else) = d.tokens[d.ptr].data {
            self.advance(d)?;
            self.expect_symbol(Symbol::LeftCurly, d)?;
            let stmts = self.parse_statements(d)?;
            self.expect_symbol(Symbol::RightCurly, d)?;

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

    fn parse_while(&self, d: &mut ParserData) -> Result<Statement, ParseError> {
        self.expect_keyword(Keyword::While, d)?;
        self.expect_symbol(Symbol::LeftRound, d)?;
        let cond = self.parse_expression(d)?;
        self.expect_symbol(Symbol::RightRound, d)?;
        self.expect_symbol(Symbol::LeftCurly, d)?;
        let body = self.parse_statements(d)?;
        self.expect_symbol(Symbol::RightCurly, d)?;

        Ok(Statement::While(super::syntax::WhileStmt { cond, body }))
    }

    fn parse_do(&self, d: &mut ParserData) -> Result<Statement, ParseError> {
        self.expect_keyword(Keyword::Do, d)?;
        let call = self.parse_subroutine_call(d)?;
        self.expect_symbol(Symbol::Semicolon, d)?;

        Ok(Statement::Do(super::syntax::DoStmt { call }))
    }

    fn parse_return(&self, d: &mut ParserData) -> Result<Statement, ParseError> {
        self.expect_keyword(Keyword::Return, d)?;
        if let TokenData::Symbol(Symbol::Semicolon) = d.tokens[d.ptr].data {
            self.advance(d)?;
            return Ok(Statement::Return(super::syntax::ReturnStmt {
                ret_val: None,
            }));
        }

        let ret_val = Some(self.parse_expression(d)?);
        self.expect_symbol(Symbol::Semicolon, d)?;
        Ok(Statement::Return(super::syntax::ReturnStmt { ret_val }))
    }

    fn parse_subroutine_call(&self, d: &mut ParserData) -> Result<SubroutineCall, ParseError> {
        let name = self.parse_name(d)?;
        let (name, caller) = if let TokenData::Symbol(Symbol::Dot) = d.tokens[d.ptr].data {
            self.advance(d)?;
            let n = self.parse_name(d)?;
            (n, Some(name))
        } else {
            (name, None)
        };
        self.expect_symbol(Symbol::LeftRound, d)?;

        let mut args = Vec::new();
        if let TokenData::Symbol(Symbol::RightRound) = d.tokens[d.ptr].data {
            self.advance(d)?;
            return Ok(SubroutineCall { caller, name, args });
        }
        loop {
            args.push(self.parse_expression(d)?);
            if let TokenData::Symbol(Symbol::RightRound) = d.tokens[d.ptr].data {
                break;
            }
            self.expect_symbol(Symbol::Comma, d)?;
        }
        self.expect_symbol(Symbol::RightRound, d)?;

        Ok(SubroutineCall { caller, name, args })
    }

    fn parse_expression(&self, d: &mut ParserData) -> Result<Expression, ParseError> {
        let init_term = self.parse_term(d)?;

        let mut ops = Vec::new();
        while self.is_op(d)? {
            let op = self.parse_op(d)?;
            ops.push((op, self.parse_term(d)?));
        }

        Ok(Expression { init_term, ops })
    }

    fn is_op(&self, d: &mut ParserData) -> Result<bool, ParseError> {
        if d.ptr >= d.tokens.len() {
            return Err(ParseError {
                message: "Reading tokens after EOF!".to_string(),
            });
        }

        if let TokenData::Symbol(s) = d.tokens[d.ptr].data {
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

    fn parse_op(&self, d: &mut ParserData) -> Result<Op, ParseError> {
        if d.ptr >= d.tokens.len() {
            return Err(ParseError {
                message: "Reading tokens after EOF!".to_string(),
            });
        }

        let curr = d.ptr;
        d.ptr += 1;

        if let TokenData::Symbol(s) = d.tokens[curr].data {
            let op = s.into();
            match op {
                Op::Unknown => return_internal!(),
                _ => Ok(op),
            }
        } else {
            return_internal!();
        }
    }

    fn parse_term(&self, d: &mut ParserData) -> Result<usize, ParseError> {
        self.advance(d)?;
        let curr_token = &d.tokens[d.ptr - 1];
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
                    d.terms.push(Term::KeywordConstant(kw.into()));
                }
            }
            TokenData::Symbol(s) => {
                match s {
                    Symbol::Minus | Symbol::Not => {
                        let op = s.into();
                        let term = self.parse_term(d)?;
                        d.terms.push(Term::Unary(UnaryTerm { op, term }));
                    }
                    Symbol::LeftRound => {
                        let expr = self.parse_expression(d)?;
                        self.expect_symbol(Symbol::RightRound, d)?;
                        d.terms.push(Term::BracketExpression(expr));
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
                d.terms.push(Term::Int(i));
            }
            TokenData::String(_s) => {
                d.terms.push(Term::String(d.ptr - 1));
            }
            TokenData::Identifier(_id) => {
                let id = d.ptr - 1;
                if let Ok(s) = self
                    .expect_any_symbol(vec![Symbol::LeftSquare, Symbol::LeftRound, Symbol::Dot], d)
                {
                    match s {
                        Symbol::LeftSquare => {
                            let expr = self.parse_expression(d)?;
                            self.expect_symbol(Symbol::RightSquare, d)?;
                            d.terms
                                .push(Term::ArrayAccess(ArrayAccess { var: id, idx: expr }));
                        }
                        Symbol::LeftRound | Symbol::Dot => {
                            self.revert(d)?;
                            self.revert(d)?;
                            let call = self.parse_subroutine_call(d)?;
                            d.terms.push(Term::Call(call));
                        }
                        _ => {
                            return_internal!();
                        }
                    }
                } else {
                    self.revert(d)?;
                    d.terms.push(Term::VarName(id));
                }
            }
        }

        Ok(d.terms.len() - 1)
    }

    fn parse_name(&self, d: &mut ParserData) -> Result<IdentifierId, ParseError> {
        self.expect(TokenKind::Identifier, d)?;
        if let TokenData::Identifier(_id) = d.tokens[d.ptr - 1].data {
            Ok(d.ptr - 1)
        } else {
            return_internal!();
        }
    }

    // Token iteration:
    fn expect(&self, kind: TokenKind, d: &mut ParserData) -> Result<(), ParseError> {
        if d.ptr >= d.tokens.len() {
            return Err(ParseError {
                message: format!("Expected {:?}, after reaching end of file", kind),
            });
        }

        let curr = d.ptr;
        d.ptr += 1;

        let tk: TokenKind = (&d.tokens[curr].data).into();
        if tk != kind {
            return Err(ParseError {
                message: format!(
                    "Expected {:?}, but got {:?} at {}:{}",
                    kind, d.tokens[curr].data, d.tokens[curr].file, d.tokens[curr].line
                ),
            });
        }

        Ok(())
    }

    fn expect_any(
        &self,
        kinds: Vec<TokenKind>,
        d: &mut ParserData,
    ) -> Result<TokenKind, ParseError> {
        assert!(!kinds.is_empty());

        if d.ptr >= d.tokens.len() {
            return Err(ParseError {
                message: format!("Expected one of {:?}, after reaching end of file", kinds),
            });
        }

        let curr = d.ptr;
        d.ptr += 1;

        let tk: TokenKind = (&d.tokens[curr].data).into();
        for kind in kinds.iter() {
            if tk == *kind {
                return Ok(tk);
            }
        }

        let tok = &d.tokens[curr];
        Err(ParseError {
            message: format!(
                "Expected one of {:?}, got {:?} in {}:{}",
                kinds, tk, tok.file, tok.line
            ),
        })
    }

    fn expect_any_keyword(
        &self,
        kws: Vec<Keyword>,
        d: &mut ParserData,
    ) -> Result<Keyword, ParseError> {
        if kws.is_empty() {
            return_internal!();
        }

        if d.ptr >= d.tokens.len() {
            return Err(ParseError {
                message: format!("Expected one of {:?}, after reaching end of file", kws),
            });
        }

        let curr = d.ptr;
        d.ptr += 1;

        if let TokenData::Keyword(k) = d.tokens[curr].data {
            for kw in kws.iter() {
                if k == *kw {
                    return Ok(k);
                }
            }
        }

        let tok = &d.tokens[curr];
        Err(ParseError {
            message: format!(
                "Expected any of {:?}, got {:?} in {}:{}",
                kws, tok.data, tok.file, tok.line
            ),
        })
    }

    fn expect_any_symbol(
        &self,
        symbols: Vec<Symbol>,
        d: &mut ParserData,
    ) -> Result<Symbol, ParseError> {
        if symbols.is_empty() {
            return_internal!();
        }

        if d.ptr >= d.tokens.len() {
            return Err(ParseError {
                message: format!("Expected one of {:?}, after reaching end of file", symbols),
            });
        }

        let curr = d.ptr;
        d.ptr += 1;

        if let TokenData::Symbol(s) = d.tokens[curr].data {
            for sym in symbols.iter() {
                if s == *sym {
                    return Ok(s);
                }
            }
        }

        let tok = &d.tokens[curr];
        Err(ParseError {
            message: format!(
                "Expected any of {:?}, got {:?} in {}:{}",
                symbols, tok.data, tok.file, tok.line
            ),
        })
    }

    fn expect_keyword(&self, kw: Keyword, d: &mut ParserData) -> Result<(), ParseError> {
        if d.ptr >= d.tokens.len() {
            return Err(ParseError {
                message: format!("Expected {:?}, after reaching end of file", kw),
            });
        }

        let curr = d.ptr;
        d.ptr += 1;

        if let TokenData::Keyword(w) = d.tokens[curr].data {
            if w != kw {
                return Err(ParseError {
                    message: format!(
                        "Expected {:?}, but got {:?} at {}:{}",
                        kw, d.tokens[curr].data, d.tokens[curr].file, d.tokens[curr].line
                    ),
                });
            }
        } else {
            return Err(ParseError {
                message: format!(
                    "Expected {:?}, but got {:?} at {}:{}",
                    kw, d.tokens[curr].data, d.tokens[curr].file, d.tokens[curr].line
                ),
            });
        }

        Ok(())
    }

    fn expect_symbol(&self, sym: Symbol, d: &mut ParserData) -> Result<(), ParseError> {
        if d.ptr >= d.tokens.len() {
            return Err(ParseError {
                message: format!("Expected {:?}, after reaching end of file", sym),
            });
        }

        let curr = d.ptr;
        d.ptr += 1;

        if let TokenData::Symbol(s) = d.tokens[curr].data {
            if s != sym {
                return Err(ParseError {
                    message: format!(
                        "Expected {:?}, but got {:?} at {}:{}",
                        sym, d.tokens[curr].data, d.tokens[curr].file, d.tokens[curr].line
                    ),
                });
            }
        } else {
            return Err(ParseError {
                message: format!(
                    "Expected {:?}, but got {:?} at {}:{}",
                    sym, d.tokens[curr].data, d.tokens[curr].file, d.tokens[curr].line
                ),
            });
        }

        Ok(())
    }

    fn advance(&self, d: &mut ParserData) -> Result<(), ParseError> {
        if d.ptr >= d.tokens.len() {
            return Err(ParseError {
                message: "Advance tokens after EOF.".to_string(),
            });
        }

        d.ptr += 1;

        Ok(())
    }

    fn revert(&self, d: &mut ParserData) -> Result<(), ParseError> {
        if d.ptr == 0 {
            return Err(ParseError {
                message: "Revert first token".to_string(),
            });
        }

        d.ptr -= 1;

        Ok(())
    }
}
