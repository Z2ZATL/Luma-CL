use thiserror::Error;

#[derive(Error, Debug)]
pub enum LumaError {
    #[error("Lexical error at line {line}: {message}")]
    LexError { message: String, line: usize },

    #[error("Parse error at line {line}: {message}")]
    ParseError { message: String, line: usize },

    #[error("Compile error at line {line}: {message}")]
    CompileError { message: String, line: usize },

    #[error("Runtime error: {0}")]
    RuntimeError(String),

    #[error("Stack error: {0}")]
    StackError(String),

    #[error("JIT error: {0}")]
    #[allow(dead_code)]
    JitError(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] bincode::Error),
}

impl LumaError {
    pub fn parse_error(message: String, line: usize) -> Self {
        LumaError::ParseError { message, line }
    }
    
    pub fn lex_error(message: String, line: usize) -> Self {
        LumaError::LexError { message, line }
    }
    
    pub fn compile_error(message: String, line: usize) -> Self {
        LumaError::CompileError { message, line }
    }
}

impl From<String> for LumaError {
    fn from(msg: String) -> Self {
        LumaError::RuntimeError(msg)
    }
}

impl From<&str> for LumaError {
    fn from(msg: &str) -> Self {
        LumaError::RuntimeError(msg.to_string())
    }
}

pub type Result<T> = std::result::Result<T, LumaError>;