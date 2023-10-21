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

#[derive(Debug, PartialEq)]
pub(crate) enum Keyword {
    Unknown,

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

#[derive(Debug, PartialEq)]
pub(crate) enum Symbol {
    Unknown,

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
pub(crate) enum TokenKind<'a> {
    Keyword(Keyword),
    Symbol(Symbol),
    Int(i32),
    String(&'a str),
    Identifier(Identifier<'a>),
}

#[derive(Debug)]
pub(crate) struct Token<'a> {
    pub(crate) kind: TokenKind<'a>,
    pub(crate) file: &'a str,
    pub(crate) line: usize,
}