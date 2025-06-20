use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpCode {
    // Constants and literals
    OpConstant,     // Load constant from chunk
    OpNil,          // Load nil value
    OpTrue,         // Load true value  
    OpFalse,        // Load false value

    // Arithmetic operations
    OpAdd,          // Pop two values, add, push result
    OpSubtract,     // Pop two values, subtract, push result
    OpMultiply,     // Pop two values, multiply, push result
    OpDivide,       // Pop two values, divide, push result
    OpModulo,       // Pop two values, modulo, push result
    OpNegate,       // Pop value, negate, push result

    // Comparison operations
    OpEqual,        // Pop two values, compare equality, push bool
    OpGreater,      // Pop two values, compare >, push bool
    OpLess,         // Pop two values, compare <, push bool
    OpGreaterEqual, // Pop two values, compare >=, push bool
    OpLessEqual,    // Pop two values, compare <=, push bool
    OpNotEqual,     // Pop two values, compare !=, push bool

    // Logical operations
    OpNot,          // Pop value, logical not, push result
    OpAnd,          // Pop two values, logical and, push result
    OpOr,           // Pop two values, logical or, push result

    // Variable operations
    OpDefineGlobal, // Define global variable
    OpGetGlobal,    // Get global variable value
    OpSetGlobal,    // Set global variable value
    OpGetLocal,     // Get local variable value
    OpSetLocal,     // Set local variable value

    // Control flow
    OpJump,         // Unconditional jump
    OpJumpIfFalse,  // Jump if top of stack is falsy
    OpLoop,         // Jump backwards (for loops)

    // Function operations
    OpCall,         // Call function
    OpReturn,       // Return from function

    // Stack operations
    OpPop,          // Pop top value from stack
    OpDup,          // Duplicate top value on stack
    OpSwap,         // Swap top two values on stack

    // Output
    OpPrint,        // Pop and print top value

    // String operations
    OpConcat,       // Pop two values, concatenate as strings, push result

    // Loop optimization markers
    OpLoopStart,    // Mark beginning of hot loop
    OpLoopEnd,      // Mark end of hot loop
}

impl OpCode {
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0 => Some(OpCode::OpConstant),
            1 => Some(OpCode::OpNil),
            2 => Some(OpCode::OpTrue),
            3 => Some(OpCode::OpFalse),
            4 => Some(OpCode::OpAdd),
            5 => Some(OpCode::OpSubtract),
            6 => Some(OpCode::OpMultiply),
            7 => Some(OpCode::OpDivide),
            8 => Some(OpCode::OpModulo),
            9 => Some(OpCode::OpNegate),
            10 => Some(OpCode::OpEqual),
            11 => Some(OpCode::OpGreater),
            12 => Some(OpCode::OpLess),
            13 => Some(OpCode::OpGreaterEqual),
            14 => Some(OpCode::OpLessEqual),
            15 => Some(OpCode::OpNotEqual),
            16 => Some(OpCode::OpNot),
            17 => Some(OpCode::OpAnd),
            18 => Some(OpCode::OpOr),
            19 => Some(OpCode::OpDefineGlobal),
            20 => Some(OpCode::OpGetGlobal),
            21 => Some(OpCode::OpSetGlobal),
            22 => Some(OpCode::OpGetLocal),
            23 => Some(OpCode::OpSetLocal),
            24 => Some(OpCode::OpJump),
            25 => Some(OpCode::OpJumpIfFalse),
            26 => Some(OpCode::OpLoop),
            27 => Some(OpCode::OpCall),
            28 => Some(OpCode::OpReturn),
            29 => Some(OpCode::OpPop),
            30 => Some(OpCode::OpDup),
            31 => Some(OpCode::OpSwap),
            32 => Some(OpCode::OpPrint),
            33 => Some(OpCode::OpConcat),
            34 => Some(OpCode::OpLoopStart),
            35 => Some(OpCode::OpLoopEnd),
            _ => None,
        }
    }

    pub fn to_byte(self) -> u8 {
        self as u8
    }
}