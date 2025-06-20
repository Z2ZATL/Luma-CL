use crate::frontend::{Token, Statement, Expression, BinaryOperator, UnaryOperator};
use crate::shared::LumaError;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, LumaError> {
        let mut statements = Vec::new();
        
        while !self.is_at_end() {
            // Skip newlines at the beginning
            if self.check(&Token::Newline) {
                self.advance();
                continue;
            }
            
            if self.check(&Token::Eof) {
                break;
            }
            
            statements.push(self.parse_statement()?);
            
            // Consume optional newline after statement
            if self.check(&Token::Newline) {
                self.advance();
            }
        }
        
        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Statement, LumaError> {
        if self.check(&Token::Let) {
            self.parse_assignment()
        } else if self.check(&Token::Show) {
            self.parse_show()
        } else if self.check(&Token::If) {
            self.parse_if_statement()
        } else if self.check(&Token::While) {
            self.parse_while_statement()
        } else if self.check(&Token::Repeat) {
            self.parse_repeat_statement()
        } else if let Token::Identifier(_) = self.peek() {
            // Handle variable assignment with "is" syntax
            self.parse_variable_reassignment()
        } else {
            Err(LumaError::ParseError(format!(
                "Expected statement, found '{}'", 
                self.peek()
            )))
        }
    }

    fn parse_assignment(&mut self) -> Result<Statement, LumaError> {
        self.consume(&Token::Let, "Expected 'let'")?;
        
        let name = if let Token::Identifier(name) = self.advance() {
            name.clone()
        } else {
            return Err(LumaError::ParseError("Expected identifier after 'let'".to_string()));
        };
        
        // Support both "be" and "is" after let
        if self.check(&Token::Be) {
            self.consume(&Token::Be, "Expected 'be' after identifier")?;
        } else if self.check(&Token::Is) {
            self.consume(&Token::Is, "Expected 'is' after identifier")?;
        } else {
            return Err(LumaError::ParseError("Expected 'be' or 'is' after identifier".to_string()));
        }
        
        let value = self.parse_expression()?;
        
        Ok(Statement::Assignment { name, value })
    }

    fn parse_variable_reassignment(&mut self) -> Result<Statement, LumaError> {
        let name = if let Token::Identifier(name) = self.advance() {
            name.clone()
        } else {
            return Err(LumaError::ParseError("Expected identifier".to_string()));
        };
        
        // Support both "is" and "=" for reassignment
        if self.check(&Token::Is) {
            self.consume(&Token::Is, "Expected 'is' after identifier")?;
        } else if self.check(&Token::Assign) {
            self.consume(&Token::Assign, "Expected '=' after identifier")?;
        } else {
            return Err(LumaError::ParseError("Expected 'is' or '=' after identifier".to_string()));
        }
        
        let value = self.parse_expression()?;
        
        Ok(Statement::Assignment { name, value })
    }

    fn parse_show(&mut self) -> Result<Statement, LumaError> {
        self.consume(&Token::Show, "Expected 'show'")?;
        let expression = self.parse_expression()?;
        Ok(Statement::Show(expression))
    }

    fn parse_if_statement(&mut self) -> Result<Statement, LumaError> {
        self.consume(&Token::If, "Expected 'if'")?;
        let condition = self.parse_expression()?;
        self.consume(&Token::Then, "Expected 'then' after if condition")?;
        
        // Consume optional newline after "then"
        self.skip_newlines();
        
        // Parse then branch statements using a limited block parser
        let then_branch = self.parse_if_block()?;
        
        // Parse else if branches
        let mut else_ifs = Vec::new();
        while self.check(&Token::ElseIf) {
            self.advance(); // consume "else if"
            let else_if_condition = self.parse_expression()?;
            self.consume(&Token::Then, "Expected 'then' after else if condition")?;
            
            self.skip_newlines();
            let else_if_statements = self.parse_if_block()?;
            else_ifs.push((else_if_condition, else_if_statements));
        }
        
        // Parse else branch
        let else_branch = if self.check(&Token::Else) {
            self.advance(); // consume "else"
            self.skip_newlines();
            Some(self.parse_if_block()?)
        } else {
            None
        };
        
        Ok(Statement::If {
            condition,
            then_branch,
            else_ifs,
            else_branch,
        })
    }

    fn parse_if_block(&mut self) -> Result<Vec<Statement>, LumaError> {
        let mut statements = Vec::new();
        
        while !self.is_at_end() {
            self.skip_newlines();
            
            // Check for end of file
            if self.check(&Token::Eof) {
                break;
            }
            
            // Break on control flow keywords that end the if block
            if self.check(&Token::Else) || self.check(&Token::ElseIf) {
                break;
            }
            
            // Parse statements that belong to this if block
            if self.check(&Token::Let) || self.check(&Token::Show) || 
               matches!(self.peek(), Token::Identifier(_)) {
                statements.push(self.parse_statement()?);
            } else if self.check(&Token::If) || self.check(&Token::While) || self.check(&Token::Repeat) {
                // These create their own nested blocks
                statements.push(self.parse_statement()?);
            } else {
                // End of this if block
                break;
            }
            
            self.skip_newlines();
        }
        
        Ok(statements)
    }

    fn parse_block(&mut self) -> Result<Vec<Statement>, LumaError> {
        let mut statements = Vec::new();
        
        while !self.is_at_end() {
            self.skip_newlines();
            
            // Check for end of file
            if self.check(&Token::Eof) {
                break;
            }
            
            // Parse any valid statement - let the statement parser handle its own logic
            if self.check(&Token::Let) || self.check(&Token::Show) || 
               self.check(&Token::If) || self.check(&Token::While) || 
               self.check(&Token::Repeat) || 
               matches!(self.peek(), Token::Identifier(_)) {
                
                statements.push(self.parse_statement()?);
            } else {
                // If we encounter an unrecognized token, break
                break;
            }
            
            self.skip_newlines();
        }
        
        Ok(statements)
    }

    fn skip_newlines(&mut self) {
        while self.check(&Token::Newline) {
            self.advance();
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, LumaError> {
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> Result<Expression, LumaError> {
        let mut expr = self.parse_logical_and()?;
        
        while self.check(&Token::Or) {
            self.advance();
            let right = self.parse_logical_and()?;
            expr = Expression::binary_op(expr, BinaryOperator::Or, right);
        }
        
        Ok(expr)
    }

    fn parse_logical_and(&mut self) -> Result<Expression, LumaError> {
        let mut expr = self.parse_equality()?;
        
        while self.check(&Token::And) {
            self.advance();
            let right = self.parse_equality()?;
            expr = Expression::binary_op(expr, BinaryOperator::And, right);
        }
        
        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<Expression, LumaError> {
        let mut expr = self.parse_comparison()?;
        
        while self.check(&Token::Equal) || self.check(&Token::NotEqual) || 
              self.check(&Token::Is) || self.check(&Token::IsNot) {
            let operator = match self.advance() {
                Token::Equal => BinaryOperator::Equal,
                Token::NotEqual => BinaryOperator::NotEqual,
                Token::Is => BinaryOperator::Equal,
                Token::IsNot => BinaryOperator::NotEqual,
                _ => unreachable!(),
            };
            let right = self.parse_comparison()?;
            expr = Expression::binary_op(expr, operator, right);
        }
        
        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expression, LumaError> {
        let mut expr = self.parse_addition()?;
        
        while self.check(&Token::GreaterThan) || self.check(&Token::LessThan) || 
              self.check(&Token::GreaterEqual) || self.check(&Token::LessEqual) {
            let operator = match self.advance() {
                Token::GreaterThan => BinaryOperator::GreaterThan,
                Token::LessThan => BinaryOperator::LessThan,
                Token::GreaterEqual => BinaryOperator::GreaterEqual,
                Token::LessEqual => BinaryOperator::LessEqual,
                _ => unreachable!(),
            };
            let right = self.parse_addition()?;
            expr = Expression::binary_op(expr, operator, right);
        }
        
        Ok(expr)
    }

    fn parse_addition(&mut self) -> Result<Expression, LumaError> {
        let mut expr = self.parse_multiplication()?;
        
        while self.check(&Token::Plus) || self.check(&Token::Minus) {
            let operator = match self.advance() {
                Token::Plus => BinaryOperator::Add,
                Token::Minus => BinaryOperator::Subtract,
                _ => unreachable!(),
            };
            let right = self.parse_multiplication()?;
            expr = Expression::binary_op(expr, operator, right);
        }
        
        Ok(expr)
    }

    fn parse_multiplication(&mut self) -> Result<Expression, LumaError> {
        let mut expr = self.parse_unary()?;
        
        while self.check(&Token::Multiply) || self.check(&Token::Divide) || self.check(&Token::Modulo) {
            let operator = match self.advance() {
                Token::Multiply => BinaryOperator::Multiply,
                Token::Divide => BinaryOperator::Divide,
                Token::Modulo => BinaryOperator::Modulo,
                _ => unreachable!(),
            };
            let right = self.parse_unary()?;
            expr = Expression::binary_op(expr, operator, right);
        }
        
        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expression, LumaError> {
        if self.check(&Token::Not) {
            self.advance();
            let operand = self.parse_unary()?;
            return Ok(Expression::UnaryOp {
                operator: UnaryOperator::Not,
                operand: Box::new(operand),
            });
        }
        
        if self.check(&Token::Minus) {
            self.advance();
            let operand = self.parse_unary()?;
            return Ok(Expression::UnaryOp {
                operator: UnaryOperator::Minus,
                operand: Box::new(operand),
            });
        }
        
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expression, LumaError> {
        if let Token::Number(n) = self.peek() {
            let number = *n;
            self.advance();
            return Ok(Expression::Literal(number));
        }
        
        if let Token::String(s) = self.peek() {
            let string = s.clone();
            self.advance();
            return Ok(Expression::StringLiteral(string));
        }
        
        if self.check(&Token::True) {
            self.advance();
            return Ok(Expression::BooleanLiteral(true));
        }
        
        if self.check(&Token::False) {
            self.advance();
            return Ok(Expression::BooleanLiteral(false));
        }
        
        if let Token::Identifier(name) = self.peek() {
            let name = name.clone();
            self.advance();
            
            // Check for function call
            if self.check(&Token::LeftParen) {
                self.advance(); // consume '('
                let mut arguments = Vec::new();
                
                if !self.check(&Token::RightParen) {
                    loop {
                        arguments.push(self.parse_expression()?);
                        if !self.check(&Token::Comma) {
                            break;
                        }
                        self.advance(); // consume ','
                    }
                }
                
                self.consume(&Token::RightParen, "Expected ')' after function arguments")?;
                return Ok(Expression::FunctionCall { name, arguments });
            } else {
                return Ok(Expression::Identifier(name));
            }
        }
        
        if self.check(&Token::LeftParen) {
            self.advance(); // consume '('
            let expr = self.parse_expression()?;
            self.consume(&Token::RightParen, "Expected ')' after expression")?;
            return Ok(expr);
        }
        
        Err(LumaError::ParseError(format!(
            "Expected expression, found '{}'", 
            self.peek()
        )))
    }

    fn check(&self, token_type: &Token) -> bool {
        if self.is_at_end() {
            false
        } else {
            std::mem::discriminant(self.peek()) == std::mem::discriminant(token_type)
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek(), Token::Eof) || self.current >= self.tokens.len()
    }

    fn peek(&self) -> &Token {
        if self.current >= self.tokens.len() {
            &Token::Eof
        } else {
            &self.tokens[self.current]
        }
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, token_type: &Token, message: &str) -> Result<&Token, LumaError> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(LumaError::ParseError(format!(
                "{}: expected '{}', found '{}'", 
                message, token_type, self.peek()
            )))
        }
    }

    fn parse_while_statement(&mut self) -> Result<Statement, LumaError> {
        self.consume(&Token::While, "while statement")?;
        let condition = self.parse_expression()?;
        self.consume(&Token::Then, "while statement (expected 'then' after condition)")?;
        
        // Consume newline after then
        if self.check(&Token::Newline) {
            self.advance();
        }
        
        let body = self.parse_block()?;
        
        Ok(Statement::While {
            condition,
            body,
        })
    }

    fn parse_repeat_statement(&mut self) -> Result<Statement, LumaError> {
        self.consume(&Token::Repeat, "repeat statement")?;
        let count = self.parse_expression()?;
        self.consume(&Token::Times, "repeat statement (expected 'times' after count)")?;
        self.consume(&Token::Then, "repeat statement (expected 'then' after 'times')")?;
        
        // Consume newline after then
        if self.check(&Token::Newline) {
            self.advance();
        }
        
        let body = self.parse_block()?;
        
        Ok(Statement::Repeat {
            count,
            body,
        })
    }
}

