use std::{iter::Peekable, str::Chars};

use crate::token::Token;

struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    fn new(input: &str) -> Lexer {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.read_char() {
            Some('=') => match self.peek_char() {
                Some(&'=') => {
                    self.read_char();
                    Token::Equal
                }
                _ => Token::Assign,
            },
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('/') => Token::Slash,
            Some('*') => Token::Asterisk,
            Some('<') => Token::LessThan,
            Some('>') => Token::GreaterThan,
            Some('!') => match self.peek_char() {
                Some(&'=') => {
                    self.read_char();
                    Token::NotEqual
                }
                _ => Token::Bang,
            },
            Some(',') => Token::Comma,
            Some(';') => Token::Semicolon,
            Some('(') => Token::LeftParen,
            Some(')') => Token::RightParen,
            Some('{') => Token::LeftBrace,
            Some('}') => Token::RightBrace,
            Some(ch) => {
                if ch.is_alphabetic() {
                    self.read_identifier(ch)
                } else if ch.is_digit(10) {
                    self.read_integer(ch)
                } else {
                    Token::Illegal(ch)
                }
            }
            None => Token::Eof,
        }
    }

    fn read_char(&mut self) -> Option<char> {
        self.input.next()
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn read_identifier(&mut self, ch: char) -> Token {
        let mut ident = String::new();
        ident.push(ch);

        while let Some(&ch) = self.peek_char() {
            if !Self::is_ident_char(ch) {
                break;
            }

            ident.push(self.read_char().unwrap());
        }

        Token::look_up_ident(ident)
    }

    fn read_integer(&mut self, ch: char) -> Token {
        let mut int_string = String::new();
        int_string.push(ch);

        while let Some(&ch) = self.peek_char() {
            if !ch.is_digit(10) {
                break;
            }

            int_string.push(self.read_char().unwrap())
        }

        Token::Int(int_string.parse::<usize>().unwrap())
    }

    fn peek_is_whitespace(&mut self) -> bool {
        match self.peek_char() {
            Some(&ch) => ch.is_whitespace(),
            None => false,
        }
    }

    fn skip_whitespace(&mut self) {
        while self.peek_is_whitespace() {
            self.read_char();
        }
    }

    fn is_ident_char(ch: char) -> bool {
        ch.is_alphanumeric() || ch == '_'
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use crate::token::Token;

    #[test]
    fn test_next_token() {
        let input = "let five = 5;
let ten = 10;

let add = fn(a, b) {
    a + b;
}

let result = add(five, ten);

!-*/5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;";

        let mut lexer = Lexer::new(&input);
        assert_eq!(lexer.next_token(), Token::Let);
        assert_eq!(lexer.next_token(), Token::Ident(String::from("five")));
        assert_eq!(lexer.next_token(), Token::Assign);
        assert_eq!(lexer.next_token(), Token::Int(5));
        assert_eq!(lexer.next_token(), Token::Semicolon);
        assert_eq!(lexer.next_token(), Token::Let);
        assert_eq!(lexer.next_token(), Token::Ident(String::from("ten")));
        assert_eq!(lexer.next_token(), Token::Assign);
        assert_eq!(lexer.next_token(), Token::Int(10));
        assert_eq!(lexer.next_token(), Token::Semicolon);
        assert_eq!(lexer.next_token(), Token::Let);
        assert_eq!(lexer.next_token(), Token::Ident(String::from("add")));
        assert_eq!(lexer.next_token(), Token::Assign);
        assert_eq!(lexer.next_token(), Token::Func);
        assert_eq!(lexer.next_token(), Token::LeftParen);
        assert_eq!(lexer.next_token(), Token::Ident(String::from("a")));
        assert_eq!(lexer.next_token(), Token::Comma);
        assert_eq!(lexer.next_token(), Token::Ident(String::from("b")));
        assert_eq!(lexer.next_token(), Token::RightParen);
        assert_eq!(lexer.next_token(), Token::LeftBrace);
        assert_eq!(lexer.next_token(), Token::Ident(String::from("a")));
        assert_eq!(lexer.next_token(), Token::Plus);
        assert_eq!(lexer.next_token(), Token::Ident(String::from("b")));
        assert_eq!(lexer.next_token(), Token::Semicolon);
        assert_eq!(lexer.next_token(), Token::RightBrace);
        assert_eq!(lexer.next_token(), Token::Let);
        assert_eq!(lexer.next_token(), Token::Ident(String::from("result")));
        assert_eq!(lexer.next_token(), Token::Assign);
        assert_eq!(lexer.next_token(), Token::Ident(String::from("add")));
        assert_eq!(lexer.next_token(), Token::LeftParen);
        assert_eq!(lexer.next_token(), Token::Ident(String::from("five")));
        assert_eq!(lexer.next_token(), Token::Comma);
        assert_eq!(lexer.next_token(), Token::Ident(String::from("ten")));
        assert_eq!(lexer.next_token(), Token::RightParen);
        assert_eq!(lexer.next_token(), Token::Semicolon);
        assert_eq!(lexer.next_token(), Token::Bang);
        assert_eq!(lexer.next_token(), Token::Minus);
        assert_eq!(lexer.next_token(), Token::Asterisk);
        assert_eq!(lexer.next_token(), Token::Slash);
        assert_eq!(lexer.next_token(), Token::Int(5));
        assert_eq!(lexer.next_token(), Token::Semicolon);
        assert_eq!(lexer.next_token(), Token::Int(5));
        assert_eq!(lexer.next_token(), Token::LessThan);
        assert_eq!(lexer.next_token(), Token::Int(10));
        assert_eq!(lexer.next_token(), Token::GreaterThan);
        assert_eq!(lexer.next_token(), Token::Int(5));
        assert_eq!(lexer.next_token(), Token::Semicolon);
        assert_eq!(lexer.next_token(), Token::If);
        assert_eq!(lexer.next_token(), Token::LeftParen);
        assert_eq!(lexer.next_token(), Token::Int(5));
        assert_eq!(lexer.next_token(), Token::LessThan);
        assert_eq!(lexer.next_token(), Token::Int(10));
        assert_eq!(lexer.next_token(), Token::RightParen);
        assert_eq!(lexer.next_token(), Token::LeftBrace);
        assert_eq!(lexer.next_token(), Token::Return);
        assert_eq!(lexer.next_token(), Token::Bool(true));
        assert_eq!(lexer.next_token(), Token::Semicolon);
        assert_eq!(lexer.next_token(), Token::RightBrace);
        assert_eq!(lexer.next_token(), Token::Else);
        assert_eq!(lexer.next_token(), Token::LeftBrace);
        assert_eq!(lexer.next_token(), Token::Return);
        assert_eq!(lexer.next_token(), Token::Bool(false));
        assert_eq!(lexer.next_token(), Token::Semicolon);
        assert_eq!(lexer.next_token(), Token::RightBrace);
        assert_eq!(lexer.next_token(), Token::Int(10));
        assert_eq!(lexer.next_token(), Token::Equal);
        assert_eq!(lexer.next_token(), Token::Int(10));
        assert_eq!(lexer.next_token(), Token::Semicolon);
        assert_eq!(lexer.next_token(), Token::Int(10));
        assert_eq!(lexer.next_token(), Token::NotEqual);
        assert_eq!(lexer.next_token(), Token::Int(9));
        assert_eq!(lexer.next_token(), Token::Semicolon);
    }
}
