use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    Assignment {
        name: String,
        value: Expression,
    },
    Show(Expression),
    If {
        condition: Expression,
        then_branch: Vec<Statement>,
        else_ifs: Vec<(Expression, Vec<Statement>)>,
        else_branch: Option<Vec<Statement>>,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    Repeat {
        count: Expression,
        body: Vec<Statement>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    Literal(f64),
    StringLiteral(String),
    BooleanLiteral(bool),
    Identifier(String),
    BinaryOp {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    UnaryOp {
        operator: UnaryOperator,
        operand: Box<Expression>,
    },
    FunctionCall {
        name: String,
        arguments: Vec<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BinaryOperator {
    // Arithmetic
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    
    // Comparison
    Equal,
    NotEqual,
    GreaterThan,
    Greater, // Alias for GreaterThan
    LessThan,
    Less, // Alias for LessThan
    GreaterEqual,
    LessEqual,
    
    // Logical
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnaryOperator {
    Not,
    Minus, // Negative numbers
}

impl Expression {
    /// Create a new binary operation expression
    pub fn binary_op(left: Expression, operator: BinaryOperator, right: Expression) -> Self {
        Expression::BinaryOp {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Assignment { name, value } => {
                write!(f, "let {} be {}", name, value)
            }
            Statement::Show(expr) => {
                write!(f, "show {}", expr)
            }
            Statement::If { condition, then_branch, else_ifs, else_branch } => {
                write!(f, "if {} then", condition)?;
                for stmt in then_branch {
                    write!(f, "\n  {}", stmt)?;
                }
                for (cond, stmts) in else_ifs {
                    write!(f, "\nelse if {} then", cond)?;
                    for stmt in stmts {
                        write!(f, "\n  {}", stmt)?;
                    }
                }
                if let Some(else_stmts) = else_branch {
                    write!(f, "\nelse")?;
                    for stmt in else_stmts {
                        write!(f, "\n  {}", stmt)?;
                    }
                }
                Ok(())
            }
            Statement::While { condition, body } => {
                write!(f, "while {}:", condition)?;
                for stmt in body {
                    write!(f, "\n  {}", stmt)?;
                }
                Ok(())
            }
            Statement::Repeat { count, body } => {
                write!(f, "repeat {} times:", count)?;
                for stmt in body {
                    write!(f, "\n  {}", stmt)?;
                }
                Ok(())
            }
        }
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Literal(n) => write!(f, "{}", n),
            Expression::StringLiteral(s) => write!(f, "\"{}\"", s),
            Expression::BooleanLiteral(b) => write!(f, "{}", b),
            Expression::Identifier(name) => write!(f, "{}", name),
            Expression::BinaryOp { left, operator, right } => {
                write!(f, "({} {} {})", left, operator, right)
            },
            Expression::UnaryOp { operator, operand } => {
                write!(f, "({}{})", operator, operand)
            },
            Expression::FunctionCall { name, arguments } => {
                write!(f, "{}(", name)?;
                for (i, arg) in arguments.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            },
        }
    }
}

impl std::fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOperator::Add => write!(f, "+"),
            BinaryOperator::Subtract => write!(f, "-"),
            BinaryOperator::Multiply => write!(f, "*"),
            BinaryOperator::Divide => write!(f, "/"),
            BinaryOperator::Modulo => write!(f, "%"),
            BinaryOperator::Equal => write!(f, "=="),
            BinaryOperator::NotEqual => write!(f, "!="),
            BinaryOperator::GreaterThan => write!(f, ">"),
            BinaryOperator::LessThan => write!(f, "<"),
            BinaryOperator::GreaterEqual => write!(f, ">="),
            BinaryOperator::LessEqual => write!(f, "<="),
            BinaryOperator::And => write!(f, "and"),
            BinaryOperator::Or => write!(f, "or"),
            BinaryOperator::Greater => write!(f, ">"),
            BinaryOperator::Less => write!(f, "<"),
        }
    }
}

impl std::fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOperator::Not => write!(f, "not "),
            UnaryOperator::Minus => write!(f, "-"),
        }
    }
}

