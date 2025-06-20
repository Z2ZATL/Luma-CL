use crate::backend::vm::OpCode;
use crate::shared::Value;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
    pub lines: Vec<usize>,
    pub globals: HashMap<String, usize>, // Variable name -> constant pool index
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
            globals: HashMap::new(),
        }
    }

    pub fn write_byte(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn write_opcode(&mut self, opcode: OpCode, line: usize) {
        self.write_byte(opcode.to_byte(), line);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        // Check if constant already exists to avoid duplicates
        for (i, existing) in self.constants.iter().enumerate() {
            if *existing == value {
                return i;
            }
        }
        
        self.constants.push(value);
        self.constants.len() - 1
    }

    #[allow(dead_code)]
    pub fn write_constant(&mut self, value: Value, line: usize) -> usize {
        let constant_index = self.add_constant(value);
        self.write_opcode(OpCode::OpConstant, line);
        self.write_byte(constant_index as u8, line);
        constant_index
    }

    pub fn patch_jump(&mut self, offset: usize) {
        let jump = self.code.len() - offset - 1;
        if jump > u8::MAX as usize {
            panic!("Too much code to jump over");
        }
        self.code[offset] = jump as u8;
    }

    pub fn emit_jump(&mut self, opcode: OpCode, line: usize) -> usize {
        self.write_opcode(opcode, line);
        self.write_byte(0, line); // Placeholder for jump offset
        self.code.len() - 1
    }

    pub fn emit_loop(&mut self, loop_start: usize, line: usize) {
        self.write_opcode(OpCode::OpLoop, line);
        let offset = self.code.len() - loop_start + 1;
        if offset > u8::MAX as usize {
            panic!("Loop body too large");
        }
        self.write_byte(offset as u8, line);
    }

    #[allow(dead_code)]
    pub fn define_global(&mut self, name: String, value: Value) -> usize {
        let constant_index = self.add_constant(value);
        self.globals.insert(name, constant_index);
        constant_index
    }

    #[allow(dead_code)]
    pub fn get_line(&self, instruction: usize) -> usize {
        if instruction < self.lines.len() {
            self.lines[instruction]
        } else {
            0
        }
    }

    #[allow(dead_code)]
    pub fn disassemble(&self, name: &str) -> String {
        let mut result = format!("== {} ==\n", name);
        let mut offset = 0;
        
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset, &mut result);
        }
        
        result
    }

    #[allow(dead_code)]
    fn disassemble_instruction(&self, offset: usize, result: &mut String) -> usize {
        result.push_str(&format!("{:04} ", offset));
        
        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            result.push_str("   | ");
        } else {
            result.push_str(&format!("{:4} ", self.lines[offset]));
        }

        let instruction = self.code[offset];
        match OpCode::from_byte(instruction) {
            Some(OpCode::OpConstant) => self.constant_instruction("OpConstant", offset, result),
            Some(OpCode::OpNil) => self.simple_instruction("OpNil", offset, result),
            Some(OpCode::OpTrue) => self.simple_instruction("OpTrue", offset, result),
            Some(OpCode::OpFalse) => self.simple_instruction("OpFalse", offset, result),
            Some(OpCode::OpAdd) => self.simple_instruction("OpAdd", offset, result),
            Some(OpCode::OpSubtract) => self.simple_instruction("OpSubtract", offset, result),
            Some(OpCode::OpMultiply) => self.simple_instruction("OpMultiply", offset, result),
            Some(OpCode::OpDivide) => self.simple_instruction("OpDivide", offset, result),
            Some(OpCode::OpNegate) => self.simple_instruction("OpNegate", offset, result),
            Some(OpCode::OpPrint) => self.simple_instruction("OpPrint", offset, result),
            Some(OpCode::OpJump) => self.jump_instruction("OpJump", 1, offset, result),
            Some(OpCode::OpJumpIfFalse) => self.jump_instruction("OpJumpIfFalse", 1, offset, result),
            Some(OpCode::OpLoop) => self.jump_instruction("OpLoop", -1, offset, result),
            Some(op) => {
                result.push_str(&format!("{:?}\n", op));
                offset + 1
            }
            None => {
                result.push_str(&format!("Unknown opcode {}\n", instruction));
                offset + 1
            }
        }
    }

    #[allow(dead_code)]
    fn simple_instruction(&self, name: &str, offset: usize, result: &mut String) -> usize {
        result.push_str(&format!("{}\n", name));
        offset + 1
    }

    #[allow(dead_code)]
    fn constant_instruction(&self, name: &str, offset: usize, result: &mut String) -> usize {
        let constant = self.code[offset + 1];
        result.push_str(&format!("{:<16} {:4} '", name, constant));
        if let Some(value) = self.constants.get(constant as usize) {
            result.push_str(&value.to_string());
        }
        result.push_str("'\n");
        offset + 2
    }

    #[allow(dead_code)]
    fn jump_instruction(&self, name: &str, sign: i32, offset: usize, result: &mut String) -> usize {
        let jump = self.code[offset + 1] as i32;
        let target = offset as i32 + 2 + sign * jump;
        result.push_str(&format!("{:<16} {:4} -> {}\n", name, offset, target));
        offset + 2
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new()
    }
}