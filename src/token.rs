#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal(char),
    Eof,

    // identifiers, literals
    Ident(String),
    Int(usize),
    Bool(bool),

    // statements
    Assign,
    If,
    Else,

    // operators
    Plus,
    Minus,
    Slash,
    Asterisk,
    LessThan,
    GreaterThan,
    Bang,
    Equal,
    NotEqual,

    // delimeters
    Comma,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    // keywords
    Func,
    Let,
    Return,
}

impl Token {
    pub fn look_up_ident(ident: String) -> Token {
        match ident.as_str() {
            "fn" => Token::Func,
            "let" => Token::Let,
            "if" => Token::If,
            "else" => Token::Else,
            "true" => Token::Bool(true),
            "false" => Token::Bool(false),
            "return" => Token::Return,
            _ => Token::Ident(ident),
        }
    }
}
