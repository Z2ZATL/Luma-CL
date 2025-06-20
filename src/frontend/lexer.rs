use crate::frontend::Token;
use crate::shared::LumaError;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LumaError> {
        let mut tokens = Vec::new();
        
        while !self.is_at_end() {
            if let Some(token) = self.next_token()? {
                tokens.push(token);
            }
        }
        
        tokens.push(Token::Eof);
        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Option<Token>, LumaError> {
        self.skip_whitespace();
        
        if self.is_at_end() {
            return Ok(None);
        }

        let ch = self.current_char();
        self.advance();

        match ch {
            '\n' => {
                self.line += 1;
                self.column = 1;
                Ok(Some(Token::Newline))
            }
            '+' => Ok(Some(Token::Plus)),
            '-' => Ok(Some(Token::Minus)),
            '*' => Ok(Some(Token::Multiply)),
            '/' => Ok(Some(Token::Divide)),
            '%' => Ok(Some(Token::Modulo)),
            '(' => Ok(Some(Token::LeftParen)),
            ')' => Ok(Some(Token::RightParen)),
            ':' => {
                // Colon is no longer used in Luma syntax
                Err(LumaError::lex_error(
                    "Unexpected character ':'. Use 'then' instead for control structures.".to_string(),
                    self.line
                ))
            }
            ',' => Ok(Some(Token::Comma)),
            '=' => {
                if !self.is_at_end() && self.current_char() == '=' {
                    self.advance(); // Skip second =
                    Ok(Some(Token::Equal))
                } else {
                    Ok(Some(Token::Assign))
                }
            }
            '!' => {
                if !self.is_at_end() && self.current_char() == '=' {
                    self.advance(); // Skip =
                    Ok(Some(Token::NotEqual))
                } else {
                    Err(LumaError::lex_error(
                        "Unexpected character '!'. Did you mean '!='?".to_string(),
                        self.line
                    ))
                }
            }
            '>' => {
                if !self.is_at_end() && self.current_char() == '=' {
                    self.advance(); // Skip =
                    Ok(Some(Token::GreaterEqual))
                } else {
                    Ok(Some(Token::GreaterThan))
                }
            }
            '<' => {
                if !self.is_at_end() && self.current_char() == '=' {
                    self.advance(); // Skip =
                    Ok(Some(Token::LessEqual))
                } else {
                    Ok(Some(Token::LessThan))
                }
            }
            '"' => {
                self.position -= 1;
                self.column -= 1;
                Ok(Some(self.read_string('"')?))
            }
            '\'' => {
                self.position -= 1;
                self.column -= 1;
                Ok(Some(self.read_string('\'')?))
            }
            '#' => {
                // Check for multi-line comment ##
                if !self.is_at_end() && self.current_char() == '#' {
                    self.advance(); // Skip second #
                    self.skip_multiline_comment()?;
                } else {
                    // Single-line comment - skip to end of line
                    self.skip_comment();
                }
                self.next_token()
            }
            _ if ch.is_ascii_digit() => {
                self.position -= 1;
                self.column -= 1;
                Ok(Some(self.read_number()?))
            }
            _ if ch.is_alphabetic() || ch == '_' => {
                self.position -= 1;
                self.column -= 1;
                Ok(Some(self.read_identifier()))
            }
            '.' => {
                // Check if this is part of a number or standalone dot
                if !self.is_at_end() && self.current_char().is_ascii_digit() {
                    self.position -= 1;
                    self.column -= 1;
                    Ok(Some(self.read_number()?))
                } else {
                    // Skip standalone dots (often used in comments)
                    self.next_token()
                }
            }
            _ => {
                // Skip all non-recognized characters (including Thai text, punctuation, etc.)
                self.next_token()
            },
        }
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            let ch = self.current_char();
            if ch.is_whitespace() && ch != '\n' {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_comment(&mut self) {
        while !self.is_at_end() && self.current_char() != '\n' {
            self.advance();
        }
    }

    fn skip_multiline_comment(&mut self) -> Result<(), LumaError> {
        let start_line = self.line;
        
        while !self.is_at_end() {
            if self.current_char() == '#' {
                self.advance();
                if !self.is_at_end() && self.current_char() == '#' {
                    self.advance(); // Skip the closing ##
                    return Ok(());
                }
            } else if self.current_char() == '\n' {
                self.line += 1;
                self.column = 1;
                self.advance();
            } else {
                self.advance();
            }
        }
        
        Err(LumaError::lex_error(
            "Unterminated multi-line comment".to_string(),
            start_line
        ))
    }

    fn read_number(&mut self) -> Result<Token, LumaError> {
        let start_pos = self.position;
        
        while !self.is_at_end() && (self.current_char().is_ascii_digit() || self.current_char() == '.') {
            self.advance();
        }
        
        let number_str: String = self.input[start_pos..self.position].iter().collect();
        let number = number_str.parse::<f64>()
            .map_err(|_| LumaError::lex_error(
                format!("Invalid number '{}'", number_str), 
                self.line
            ))?;
        
        Ok(Token::Number(number))
    }

    fn read_identifier(&mut self) -> Token {
        let start_pos = self.position;
        
        while !self.is_at_end() {
            let ch = self.current_char();
            if ch.is_alphanumeric() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }
        
        let identifier: String = self.input[start_pos..self.position].iter().collect();
        
        // Check for keywords
        match identifier.as_str() {
            "let" => Token::Let,
            "be" => Token::Be,
            "is" => {
                // Look ahead for "not" to create "is not"
                if !self.is_at_end() {
                    let saved_pos = self.position;
                    let saved_col = self.column;
                    
                    // Skip whitespace
                    while !self.is_at_end() && self.current_char().is_whitespace() && self.current_char() != '\n' {
                        self.advance();
                    }
                    
                    // Check if next identifier is "not"
                    if !self.is_at_end() && self.current_char().is_alphabetic() {
                        let start_pos = self.position;
                        while !self.is_at_end() && self.current_char().is_alphanumeric() {
                            self.advance();
                        }
                        let next_word: String = self.input[start_pos..self.position].iter().collect();
                        
                        if next_word == "not" {
                            return Token::IsNot;
                        }
                    }
                    
                    // Reset position if "not" wasn't found
                    self.position = saved_pos;
                    self.column = saved_col;
                }
                Token::Is
            },
            "show" => Token::Show,
            "true" => Token::True,
            "false" => Token::False,
            "and" => Token::And,
            "or" => Token::Or,
            "not" => Token::Not,
            "if" => Token::If,
            "then" => Token::Then,
            "while" => Token::While,
            "repeat" => Token::Repeat,
            "times" => Token::Times,
            "else" => {
                // Look ahead for "if" to create "else if"
                if !self.is_at_end() {
                    let saved_pos = self.position;
                    let saved_col = self.column;
                    
                    // Skip whitespace
                    while !self.is_at_end() && self.current_char().is_whitespace() && self.current_char() != '\n' {
                        self.advance();
                    }
                    
                    // Check if next word is "if"
                    let next_start = self.position;
                    while !self.is_at_end() {
                        let ch = self.current_char();
                        if ch.is_alphanumeric() || ch == '_' {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    
                    let next_word: String = self.input[next_start..self.position].iter().collect();
                    if next_word == "if" {
                        Token::ElseIf
                    } else {
                        // Reset position and return just "else"
                        self.position = saved_pos;
                        self.column = saved_col;
                        Token::Else
                    }
                } else {
                    Token::Else
                }
            },
            _ => Token::Identifier(identifier),
        }
    }

    fn current_char(&self) -> char {
        self.input[self.position]
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.position += 1;
            self.column += 1;
        }
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }

    fn read_string(&mut self, quote_char: char) -> Result<Token, LumaError> {
        self.advance(); // Skip opening quote
        let start_pos = self.position;
        
        while !self.is_at_end() && self.current_char() != quote_char {
            if self.current_char() == '\n' {
                self.line += 1;
                self.column = 1;
            }
            self.advance();
        }
        
        if self.is_at_end() {
            return Err(LumaError::lex_error(
                "Unterminated string".to_string(),
                self.line
            ));
        }
        
        let string_content: String = self.input[start_pos..self.position].iter().collect();
        self.advance(); // Skip closing quote
        
        Ok(Token::String(string_content))
    }
}
