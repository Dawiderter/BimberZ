use phf::phf_map;

use super::{
    error::Error,
    token::{Token, TokenType},
};

static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "let" => TokenType::Let,
    "true" => TokenType::True,
    "false" => TokenType::False,
    "print" => TokenType::Print,
    "or" => TokenType::Or,
    "and" => TokenType::And,
    "if" => TokenType::If,
    "else" => TokenType::Else,
    "for" => TokenType::For,
    "in" => TokenType::In,
};

pub struct Lexer<'a> {
    content: &'a [char],
    current_line: u64,
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a [char]) -> Self {
        Self {
            content,
            current_line: 0,
        }
    }
    fn chop(&mut self, len: usize) -> String {
        let lexeme = self.content[0..len].iter().collect();
        self.content = &self.content[len..];
        lexeme
    }

    fn chop_while<P>(&mut self, mut predicate: P) -> String
    where
        P: FnMut(&char) -> bool,
    {
        let mut i = 0;
        while i < self.content.len() && predicate(&self.content[i]) {
            i += 1;
        }
        self.chop(i)
    }

    fn trim_while<P>(&mut self, mut predicate: P)
    where
        P: FnMut(&char) -> bool,
    {
        while !self.content.is_empty() && predicate(&self.content[0]) {
            self.content = &self.content[1..]
        }
    }

    fn double_opt_token_helper(
        &mut self,
        token_type_single: TokenType,
        token_type_double: TokenType,
        next_char: char,
    ) -> Result<Token, Error> {
        if self.peek(1).is_some_and(|x| x == next_char) {
            return Ok(Token::new(token_type_double, self.chop(2)));
        }
        Ok(Token::new(token_type_single, self.chop(1)))
    }

    fn peek(&self, offset: usize) -> Option<char> {
        if self.content.len() <= offset {
            return None;
        }
        Some(self.content[offset])
    }

    fn parse_number(&mut self) -> (TokenType, String) {
        (TokenType::Integer, self.chop_while(|c| c.is_ascii_digit()))
    }

    pub fn next_token(&mut self) -> Option<Result<Token, Error>> {
        self.trim_while(|x| *x != '\n' && x.is_whitespace());

        if self.content.is_empty() {
            return None;
        }

        if self.content[0].is_numeric() {
            let (token_type, num) = self.parse_number();
            return Some(Ok(Token::new(token_type, num)));
        }

        if self.content[0].is_alphabetic() {
            let str = self.chop_while(|x| x.is_alphabetic());
            if let Some(keyword) = KEYWORDS.get(&str).cloned() {
                return Some(Ok(Token::new(keyword, str)));
            }
            return Some(Ok(Token::new(TokenType::Identifier, str)));
        }

        match self.content[0] {
            '+' => Some(Ok(Token::new(TokenType::Plus, self.chop(1)))),
            '-' => Some(Ok(Token::new(TokenType::Minus, self.chop(1)))),
            '*' => Some(Ok(Token::new(TokenType::Star, self.chop(1)))),
            '/' => Some(Ok(Token::new(TokenType::Slash, self.chop(1)))),
            '=' => {
                Some(self.double_opt_token_helper(TokenType::Equals, TokenType::EqualsEquals, '='))
            }
            '!' => Some(self.double_opt_token_helper(TokenType::Bang, TokenType::BangEquals, '=')),
            '<' => Some(self.double_opt_token_helper(TokenType::Less, TokenType::LessEquals, '=')),
            '>' => Some(self.double_opt_token_helper(
                TokenType::Greater,
                TokenType::GreaterEquals,
                '=',
            )),
            '(' => Some(Ok(Token::new(TokenType::LeftParen, self.chop(1)))),
            ')' => Some(Ok(Token::new(TokenType::RightParen, self.chop(1)))),
            '[' => Some(Ok(Token::new(TokenType::LeftSquareBracket, self.chop(1)))),
            ']' => Some(Ok(Token::new(TokenType::RightSquareBracket, self.chop(1)))),
            '{' => Some(Ok(Token::new(TokenType::LeftCurlyBracket, self.chop(1)))),
            '}' => Some(Ok(Token::new(TokenType::RightCurlyBracket, self.chop(1)))),
            ',' => Some(Ok(Token::new(TokenType::Comma, self.chop(1)))),
            '.' => Some(self.double_opt_token_helper(TokenType::Dot, TokenType::DotDot, '.')),
            '\n' => {
                self.current_line += 1;
                Some(Ok(Token::new(TokenType::Newline, self.chop(1))))
            }
            _ => Some(Err(Error {
                message: format!("Unknown token '{}'", self.chop(1)),
            })),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let code = "let x = 5;".chars().collect::<Vec<char>>();
        let mut lexer = Lexer {
            content: code.as_slice(),
            current_line: 0,
        };

        assert_eq!(
            lexer.next_token().unwrap().unwrap(),
            Token::new(TokenType::Let, "let".to_string())
        );
        assert_eq!(
            lexer.next_token().unwrap().unwrap(),
            Token::new(TokenType::Identifier, "x".to_string())
        );
        assert_eq!(
            lexer.next_token().unwrap().unwrap(),
            Token::new(TokenType::Equals, "=".to_string())
        );
        assert_eq!(
            lexer.next_token().unwrap().unwrap(),
            Token::new(TokenType::Integer, "5".to_string())
        );
    }
}
