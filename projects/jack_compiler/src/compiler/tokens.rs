/* Jack lexicon
    keyword: 'class' | 'constructor' | 'function' |
    'method' | 'field' | 'static' | 'var' | 'int' |
    'char' | 'boolean' | 'void' | 'true' | 'false' |
    'null' | 'this' | 'let' | 'do' | 'if' | 'else' |
    'while' | 'return’

    symbol: '{' | '}' | '(' | ')' | '[' | ']' | '. ' | ', ' | '; ' | '+' | '-' | '*' |

    '/' | '&' | '|' | '<' | '>' | '=' | '~'

    integerConstant: a decimal number in the range 0 ... 32767
    StringConstant: '"' a sequence of Unicode characters,
    not including double quote or newline '"'

    identifier: a sequence of letters, digits, and
    underscore ( '_' ) not starting with a digit.
*/

#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) enum Keyword {
    Class,
    Constructor,
    Function,
    Method,
    Field,
    Static,
    Var,
    Int,
    Char,
    Boolean,
    Void,
    True,
    False,
    Null,
    This,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) enum Symbol {
    RightCurly,
    LeftCurly,
    RightRound,
    LeftRound,
    RightSquare,
    LeftSquare,
    Dot,
    Comma,
    Semicolon,
    Plus,
    Minus,
    Multiply,
    Divide,
    And,
    Or,
    Equal,
    Not,
    Less,
    Greater,
}

pub(crate) type Identifier<'a> = &'a str;

#[derive(Debug, PartialEq)]
pub(crate) enum TokenData<'a> {
    Keyword(Keyword),
    Symbol(Symbol),
    Int(i32),
    String(&'a str),
    Identifier(Identifier<'a>),
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub(crate) enum TokenKind {
    Keyword,
    Symbol,
    Int,
    String,
    Identifier,
}

impl From<&TokenData<'_>> for TokenKind {
    fn from(value: &TokenData<'_>) -> Self {
        match value {
            TokenData::Symbol(_) => TokenKind::Symbol,
            TokenData::Keyword(_) => TokenKind::Keyword,
            TokenData::Int(_) => TokenKind::Int,
            TokenData::String(_) => TokenKind::String,
            TokenData::Identifier(_) => TokenKind::Identifier,
        }
    }
}

#[derive(Debug)]
pub(crate) struct Token<'a> {
    pub(crate) data: TokenData<'a>,
    pub(crate) file: &'a str,
    pub(crate) line: usize,
}
