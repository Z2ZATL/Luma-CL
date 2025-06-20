use thiserror::Error;

#[derive(Error, Debug)]
pub enum LumaError {
    #[error("Lexical error: {0}")]
    LexError(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Compile error: {0}")]
    CompileError(String),

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