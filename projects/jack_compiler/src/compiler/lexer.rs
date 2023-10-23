use crate::compiler::tokens::Token;

use super::tokens::{TokenData, Keyword, Symbol};

pub(crate) struct Lexer {
    pub(crate) filename: String,
}

impl Lexer {
    pub(crate) fn new(filename: String) -> Lexer {
        Lexer {
            filename: filename.to_string(),
        }
    }

    pub(crate) fn lex<'a>(&'a self, source: &'a Vec<String>) -> Vec<Token<'a>> {
        let mut tokens = Vec::new();

        let mut lineno = 0;
        let mut in_comment = false;
        for line in source.iter() {
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
                        data: TokenData::String(&line[str_s..read]),
                        file: &self.filename,
                        line: lineno,
                    };
                    tokens.push(token);

                    i = j + 1;
                    continue;
                }

                let token = match toks[i] {
                    "}" => TokenData::Symbol(Symbol::RightCurly),
                    "{" => TokenData::Symbol(Symbol::LeftCurly),
                    ")" => TokenData::Symbol(Symbol::RightRound),
                    "(" => TokenData::Symbol(Symbol::LeftRound),
                    "]" => TokenData::Symbol(Symbol::RightSquare),
                    "[" => TokenData::Symbol(Symbol::LeftSquare),
                    "." => TokenData::Symbol(Symbol::Dot),
                    "," => TokenData::Symbol(Symbol::Comma),
                    ";" => TokenData::Symbol(Symbol::Semicolon),
                    "+" => TokenData::Symbol(Symbol::Plus),
                    "-" => TokenData::Symbol(Symbol::Minus),
                    "*" => TokenData::Symbol(Symbol::Multiply),
                    "/" => TokenData::Symbol(Symbol::Divide),
                    "&" => TokenData::Symbol(Symbol::And),
                    "|" => TokenData::Symbol(Symbol::Or),
                    "=" => TokenData::Symbol(Symbol::Equal),
                    "~" => TokenData::Symbol(Symbol::Not),
                    "<" => TokenData::Symbol(Symbol::Less),
                    ">" => TokenData::Symbol(Symbol::Greater),
                    "class" => TokenData::Keyword(Keyword::Class),
                    "constructor" => TokenData::Keyword(Keyword::Constructor),
                    "function" => TokenData::Keyword(Keyword::Function),
                    "method" => TokenData::Keyword(Keyword::Method),
                    "field" => TokenData::Keyword(Keyword::Field),
                    "static" => TokenData::Keyword(Keyword::Static),
                    "var" => TokenData::Keyword(Keyword::Var),
                    "int" => TokenData::Keyword(Keyword::Int),
                    "char" => TokenData::Keyword(Keyword::Char),
                    "bool" => TokenData::Keyword(Keyword::Boolean),
                    "void" => TokenData::Keyword(Keyword::Void),
                    "true" => TokenData::Keyword(Keyword::True),
                    "false" => TokenData::Keyword(Keyword::False),
                    "null" => TokenData::Keyword(Keyword::Null),
                    "this" => TokenData::Keyword(Keyword::This),
                    "let" => TokenData::Keyword(Keyword::Let),
                    "do" => TokenData::Keyword(Keyword::Do),
                    "if" => TokenData::Keyword(Keyword::If),
                    "else" => TokenData::Keyword(Keyword::Else),
                    "while" => TokenData::Keyword(Keyword::While),
                    "return" => TokenData::Keyword(Keyword::Return),

                    sym => {
                        if sym.parse::<i32>().is_ok() {
                            TokenData::Int(sym.parse::<i32>().unwrap())
                        } else {
                            TokenData::Identifier(sym)
                        }
                    }
                };

                let token = Token {
                    data: token,
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