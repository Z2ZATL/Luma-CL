#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Let,
    Be,
    Is,
    Show,
    True,
    False,
    And,
    Or,
    Not,
    
    // Control flow keywords
    If,
    Then,
    Else,
    ElseIf,
    While,
    Repeat,
    Times,
    Comma,
    
    // Special keyword combinations
    IsNot,   // "is not"
    
    // Identifiers and literals
    Identifier(String),
    Number(f64),
    String(String),
    
    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    
    // Assignment and comparison operators
    Assign,       // =
    Equal,        // ==
    NotEqual,     // !=
    GreaterThan,  // >
    LessThan,     // <
    GreaterEqual, // >=
    LessEqual,    // <=
    
    // Punctuation
    LeftParen,
    RightParen,
    
    // Special
    Newline,
    Eof,
}

impl Token {

}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Let => write!(f, "let"),
            Token::Be => write!(f, "be"),
            Token::Is => write!(f, "is"),
            Token::Show => write!(f, "show"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::And => write!(f, "and"),
            Token::Or => write!(f, "or"),
            Token::Not => write!(f, "not"),
            Token::If => write!(f, "if"),
            Token::Then => write!(f, "then"),
            Token::Else => write!(f, "else"),
            Token::ElseIf => write!(f, "else if"),
            Token::While => write!(f, "while"),
            Token::Repeat => write!(f, "repeat"),
            Token::Times => write!(f, "times"),
            Token::Comma => write!(f, ","),
            Token::IsNot => write!(f, "is not"),
            Token::Identifier(name) => write!(f, "{}", name),
            Token::Number(n) => write!(f, "{}", n),
            Token::String(s) => write!(f, "\"{}\"", s),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Multiply => write!(f, "*"),
            Token::Divide => write!(f, "/"),
            Token::Modulo => write!(f, "%"),
            Token::Assign => write!(f, "="),
            Token::Equal => write!(f, "=="),
            Token::NotEqual => write!(f, "!="),
            Token::GreaterThan => write!(f, ">"),
            Token::LessThan => write!(f, "<"),
            Token::GreaterEqual => write!(f, ">="),
            Token::LessEqual => write!(f, "<="),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::Newline => write!(f, "\\n"),
            Token::Eof => write!(f, "EOF"),
        }
    }
}

