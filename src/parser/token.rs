use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    Real,
    Integer,
    Plus,
    Minus,
    Star,
    Slash,
    Equals,
    Bang,
    EqualsEquals,
    BangEquals,
    Less,
    Greater,
    LessEquals,
    GreaterEquals,
    LeftParen,
    RightParen,
    LeftSquareBracket,
    RightSquareBracket,
    LeftCurlyBracket,
    RightCurlyBracket,
    Identifier,
    Comma,
    Dot,
    DotDot,
    Let,
    True,
    False,
    Newline,
    Print,
    Or,
    And,
    If,
    Else,
    For,
    In,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let printable = match self {
            TokenType::Real => "Real",
            TokenType::Integer => "Integer",
            TokenType::Plus => "Plus",
            TokenType::Minus => "Minus",
            TokenType::Star => "Star",
            TokenType::Slash => "Slash",
            TokenType::Equals => "Equals",
            TokenType::Bang => "Bang",
            TokenType::EqualsEquals => "EqualsEquals",
            TokenType::BangEquals => "BangEquals",
            TokenType::Less => "Less",
            TokenType::Greater => "Greater",
            TokenType::LessEquals => "LessEquals",
            TokenType::GreaterEquals => "GreaterEquals",
            TokenType::LeftParen => "LeftParen",
            TokenType::RightParen => "RightParen",
            TokenType::LeftSquareBracket => "LeftSquareBracket",
            TokenType::RightSquareBracket => "RightSquareBracket",
            TokenType::LeftCurlyBracket => "LeftCurlyBracket",
            TokenType::RightCurlyBracket => "RightCurlyBracket",
            TokenType::Identifier => "Identifier",
            TokenType::Comma => "Comma",
            TokenType::Dot => "Dot",
            TokenType::DotDot => "DotDot",
            TokenType::Let => "Let",
            TokenType::True => "True",
            TokenType::False => "False",
            TokenType::Newline => "Newline",
            TokenType::Print => "Print",
            TokenType::Or => "Or",
            TokenType::And => "And",
            TokenType::If => "If",
            TokenType::Else => "Else",
            TokenType::For => "For",
            TokenType::In => "In",
        };
        write!(f, "{}", printable)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String) -> Self {
        Self { token_type, lexeme }
    }
}
