use std::{path::PathBuf, io::{BufReader, BufRead}, fs::File};

use crate::compiler::tokens::Token;

use super::tokens::{TokenKind, Keyword, Symbol};

pub(crate) struct Lexer {
    pub(crate) filename: String,
    pub(crate) source: Vec<String>,
}

impl Lexer {
    pub(crate) fn new(path: PathBuf) -> Lexer {
        let filename = path.file_stem().unwrap().to_str().unwrap();

        let mut source = Vec::new();

        let in_file = File::open(&path).unwrap();
        let reader = BufReader::new(in_file);
        for line in reader.lines() {
            let line = line.unwrap_or("".to_string());
            source.push(line);
        }

        Lexer {
            filename: filename.to_string(),
            source: source,
        }
    }

    pub(crate) fn lex<'a>(&'a self) -> Vec<Token<'a>> {
        let mut tokens = Vec::new();

        let mut lineno = 0;
        let mut in_comment = false;
        for line in self.source.iter() {
            lineno += 1;

            let line = line.trim();
            if line.is_empty() || line.starts_with("//") {
                continue;
            }

            if line.starts_with("/**") {
                in_comment = true;
            }

            let cmt_split = line.split("//").collect::<Vec<&str>>();
            let line = cmt_split[0];
            let line = line.trim();

            if line.ends_with("*/") {
                in_comment = false;
                continue;
            }

            if in_comment {
                continue;
            }

            let match_idx = line.match_indices(|c: char| {
                match c {
                    '{' | '}' | '(' | ')' | '[' | ']' | '.' | ',' |';' |
                    '+' | '-' | '*' | '/' | '&' | '|' | '=' | '~' | '<' |
                    '>' | ' ' | '"' => true,
                    _ => false,
                }
            });

            let mut toks = Vec::new();
            let mut last = 0;
            for (idx, matched) in match_idx {
                if last != idx {
                    toks.push(&line[last..idx]);
                }
                toks.push(matched);
                last = idx + matched.len();
            }

            let mut i = 0;
            // keeping len of read characters so that
            // we can substring any string tokens we find along the way
            let mut read = 0;
            let mut str_s;
            while i < toks.len() {
                if i > 0 {
                    read += toks[i - 1].len();
                }

                if toks[i].is_empty() || toks[i] == " " {
                    i += 1;
                    continue;
                }

                if toks[i] == "\"" {
                    read += 1;
                    str_s = read;
                    let mut j = i + 1;
                    while toks[j] != "\"" {
                        read += toks[j].len();
                        j += 1;
                    }

                    let token = Token {
                        kind: TokenKind::String(&line[str_s..read]),
                        file: &self.filename,
                        line: lineno,
                    };
                    tokens.push(token);

                    i = j + 1;
                    continue;
                }

                let token = match toks[i] {
                    "}" => TokenKind::Symbol(Symbol::RightCurly),
                    "{" => TokenKind::Symbol(Symbol::LeftCurly),
                    ")" => TokenKind::Symbol(Symbol::RightRound),
                    "(" => TokenKind::Symbol(Symbol::LeftRound),
                    "]" => TokenKind::Symbol(Symbol::RightSquare),
                    "[" => TokenKind::Symbol(Symbol::LeftSquare),
                    "." => TokenKind::Symbol(Symbol::Dot),
                    "," => TokenKind::Symbol(Symbol::Comma),
                    ";" => TokenKind::Symbol(Symbol::Semicolon),
                    "+" => TokenKind::Symbol(Symbol::Plus),
                    "-" => TokenKind::Symbol(Symbol::Minus),
                    "*" => TokenKind::Symbol(Symbol::Multiply),
                    "/" => TokenKind::Symbol(Symbol::Divide),
                    "&" => TokenKind::Symbol(Symbol::And),
                    "|" => TokenKind::Symbol(Symbol::Or),
                    "=" => TokenKind::Symbol(Symbol::Equal),
                    "~" => TokenKind::Symbol(Symbol::Not),
                    "<" => TokenKind::Symbol(Symbol::Less),
                    ">" => TokenKind::Symbol(Symbol::Greater),
                    "class" => TokenKind::Keyword(Keyword::Class),
                    "constructor" => TokenKind::Keyword(Keyword::Constructor),
                    "function" => TokenKind::Keyword(Keyword::Function),
                    "method" => TokenKind::Keyword(Keyword::Method),
                    "field" => TokenKind::Keyword(Keyword::Field),
                    "static" => TokenKind::Keyword(Keyword::Static),
                    "var" => TokenKind::Keyword(Keyword::Var),
                    "int" => TokenKind::Keyword(Keyword::Int),
                    "char" => TokenKind::Keyword(Keyword::Char),
                    "bool" => TokenKind::Keyword(Keyword::Boolean),
                    "void" => TokenKind::Keyword(Keyword::Void),
                    "true" => TokenKind::Keyword(Keyword::True),
                    "false" => TokenKind::Keyword(Keyword::False),
                    "null" => TokenKind::Keyword(Keyword::Null),
                    "this" => TokenKind::Keyword(Keyword::This),
                    "let" => TokenKind::Keyword(Keyword::Let),
                    "do" => TokenKind::Keyword(Keyword::Do),
                    "if" => TokenKind::Keyword(Keyword::If),
                    "else" => TokenKind::Keyword(Keyword::Else),
                    "while" => TokenKind::Keyword(Keyword::While),
                    "return" => TokenKind::Keyword(Keyword::Return),

                    sym => {
                        if sym.parse::<i32>().is_ok() {
                            TokenKind::Int(sym.parse::<i32>().unwrap())
                        } else {
                            TokenKind::Identifier(sym)
                        }
                    }
                };

                let token = Token {
                    kind: token,
                    file: &self.filename,
                    line: lineno,
                };
                tokens.push(token);

                i += 1;
            }
        }

        tokens
    }
}