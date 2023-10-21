use std::path::PathBuf;

use super::lexer::Lexer;
use super::syntax::{SyntaxTree, Term, ClassNode, ClassVarDec, SubroutineDec};
use super::tokens::{Token, TokenKind, Keyword, Symbol};

#[derive(Copy, Clone)]
struct TokenIterator<'a> {
    tokens: &'a Vec<Token<'a>>,
    ptr: usize,
}

impl<'a> TokenIterator<'a> {
    fn expect(&mut self, kind: TokenKind) {
        if self.ptr >= self.tokens.len() {
            panic!("Expected {:?}, after reaching end of file", kind);
        }

        let curr = self.ptr;
        self.ptr += 1;

        if self.tokens[curr].kind != kind {
            panic!("Expected {:?}, but got {:?} at {}:{}", kind, self.tokens[curr].kind, self.tokens[curr].file, self.tokens[curr].line);
        }
    }

    fn get(&mut self) -> &Token<'a> {
        if self.ptr >= self.tokens.len() {
            panic!("Reading tokens after EOF.");
        }

        let curr = self.ptr;
        self.ptr += 1;

        &self.tokens[curr]
    }
}

pub struct Parser<'a> {
    pub filename: String,
    
    pub(crate) lexer: Lexer,
    pub(crate) tokens: Vec<Token<'a>>
}

impl<'a> Parser<'a> {
    pub fn new(path: PathBuf) -> Parser<'a> {
        let filename = path.file_stem().unwrap().to_str().unwrap();
        Parser {
            filename: filename.to_string(), 
            lexer: Lexer::new(path),
            tokens: Vec::new(),
        }
    }

    pub fn parse(&'a mut self) -> SyntaxTree {
        let mut tree = SyntaxTree::new();

        self.tokens = self.lexer.lex();
        // for tok in tokens {
        //     println!("{:?}", tok);
        // }

        let mut it = TokenIterator {
            tokens: &self.tokens,
            ptr: 0,
        };
        
        let mut terms = Vec::new();
        let root = self.parse_root(&mut it, &mut terms);

        tree
    }

    fn parse_root(&'a self, tokens: &'a mut TokenIterator, terms: &mut Vec<Term<'a>>) -> ClassNode<'a> {
        tokens.expect(TokenKind::Keyword(Keyword::Class));
        let class_name_token = tokens.get();
        let class_name = if let TokenKind::Identifier(n) = class_name_token.kind {
            n
        } else {
            panic!("Expected identifier for class name at {}:{}", class_name_token.file, class_name_token.line);
        };
        tokens.expect(TokenKind::Symbol(Symbol::LeftCurly));

        let var_dec = self.parse_classvardev(tokens, terms);
        let subroutine_dec = self.parse_subroutinedev(tokens, terms);

        ClassNode {
            name: class_name,
            fields: var_dec,
            subroutines: subroutine_dec,
        }
    }

    fn parse_classvardev(&'a self, tokens: &mut TokenIterator, terms: &mut Vec<Term<'a>>) -> Vec<ClassVarDec<'a>> {
        todo!()
    }

    fn parse_subroutinedev(&'a self, tokens: &mut TokenIterator, terms: &mut Vec<Term<'a>>) -> Vec<SubroutineDec<'a>> {
        todo!()
    }
}