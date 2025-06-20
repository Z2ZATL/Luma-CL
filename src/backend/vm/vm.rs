use crate::backend::vm::{OpCode, Stack};
use crate::shared::{Chunk, Value, LumaError, Result};
use hashbrown::HashMap;
use std::time::Instant;

pub struct VM {
    chunk: Option<Chunk>,
    ip: usize, // Instruction pointer
    stack: Stack,
    globals: HashMap<String, Value>,
    
    // Performance monitoring
    execution_count: HashMap<usize, u64>, // instruction offset -> count
    hot_threshold: u64,
    start_time: Option<Instant>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            chunk: None,
            ip: 0,
            stack: Stack::new(),
            globals: HashMap::new(),
            execution_count: HashMap::new(),
            hot_threshold: 1000, // Mark as hot after 1000 executions
            start_time: None,
        }
    }

    pub fn interpret(&mut self, chunk: Chunk) -> Result<Value> {
        self.chunk = Some(chunk);
        self.ip = 0;
        self.start_time = Some(Instant::now());
        self.run()
    }

    fn run(&mut self) -> Result<Value> {
        loop {
            // Performance monitoring
            *self.execution_count.entry(self.ip).or_insert(0) += 1;
            
            let instruction = self.read_byte()?;
            let opcode = OpCode::from_byte(instruction)
                .ok_or_else(|| LumaError::RuntimeError(format!("Unknown opcode: {}", instruction)))?;

            match opcode {
                OpCode::OpConstant => {
                    let constant_index = self.read_byte()? as usize;
                    let value = self.get_constant(constant_index)?;
                    self.stack.push(value).map_err(LumaError::StackError)?;
                }
                
                OpCode::OpNil => {
                    self.stack.push(Value::Nil).map_err(LumaError::StackError)?;
                }
                
                OpCode::OpTrue => {
                    self.stack.push(Value::Boolean(true)).map_err(LumaError::StackError)?;
                }
                
                OpCode::OpFalse => {
                    self.stack.push(Value::Boolean(false)).map_err(LumaError::StackError)?;
                }
                
                OpCode::OpAdd => {
                    let b = self.stack.pop().map_err(LumaError::StackError)?;
                    let a = self.stack.pop().map_err(LumaError::StackError)?;
                    let result = self.add_values(a, b)?;
                    self.stack.push(result).map_err(LumaError::StackError)?;
                }
                OpCode::OpSubtract => {
                    let b = self.stack.pop().map_err(LumaError::StackError)?;
                    let a = self.stack.pop().map_err(LumaError::StackError)?;
                    let result = self.subtract_values(a, b)?;
                    self.stack.push(result).map_err(LumaError::StackError)?;
                }
                OpCode::OpMultiply => {
                    let b = self.stack.pop().map_err(LumaError::StackError)?;
                    let a = self.stack.pop().map_err(LumaError::StackError)?;
                    let result = self.multiply_values(a, b)?;
                    self.stack.push(result).map_err(LumaError::StackError)?;
                }
                OpCode::OpDivide => {
                    let b = self.stack.pop().map_err(LumaError::StackError)?;
                    let a = self.stack.pop().map_err(LumaError::StackError)?;
                    let result = self.divide_values(a, b)?;
                    self.stack.push(result).map_err(LumaError::StackError)?;
                }
                OpCode::OpModulo => {
                    let b = self.stack.pop().map_err(LumaError::StackError)?;
                    let a = self.stack.pop().map_err(LumaError::StackError)?;
                    let result = self.modulo_values(a, b)?;
                    self.stack.push(result).map_err(LumaError::StackError)?;
                }
                
                OpCode::OpNegate => {
                    let value = self.stack.pop().map_err(LumaError::StackError)?;
                    let result = self.negate_value(value)?;
                    self.stack.push(result).map_err(LumaError::StackError)?;
                }
                
                OpCode::OpEqual => self.binary_op(|a, b| Ok(Value::Boolean(a == b)))?,
                OpCode::OpGreater => self.comparison_op(|a, b| a > b)?,
                OpCode::OpLess => self.comparison_op(|a, b| a < b)?,
                OpCode::OpGreaterEqual => self.comparison_op(|a, b| a >= b)?,
                OpCode::OpLessEqual => self.comparison_op(|a, b| a <= b)?,
                OpCode::OpNotEqual => self.binary_op(|a, b| Ok(Value::Boolean(a != b)))?,
                
                OpCode::OpNot => {
                    let value = self.stack.pop().map_err(LumaError::StackError)?;
                    let result = Value::Boolean(!value.is_truthy());
                    self.stack.push(result).map_err(LumaError::StackError)?;
                }
                
                OpCode::OpPrint => {
                    let value = self.stack.pop().map_err(LumaError::StackError)?;
                    println!("{}", value);
                }
                
                OpCode::OpPop => {
                    self.stack.pop().map_err(LumaError::StackError)?;
                }
                
                OpCode::OpDefineGlobal => {
                    let name_index = self.read_byte()? as usize;
                    let name = self.get_constant_string(name_index)?;
                    let value = self.stack.pop().map_err(LumaError::StackError)?;
                    self.globals.insert(name, value);
                }
                
                OpCode::OpGetGlobal => {
                    let name_index = self.read_byte()? as usize;
                    let name = self.get_constant_string(name_index)?;
                    let value = self.globals.get(&name)
                        .cloned()
                        .ok_or_else(|| {
                            let line = self.get_current_line();
                            LumaError::RuntimeError(format!("Undefined variable '{}' at line {}", name, line))
                        })?;
                    self.stack.push(value).map_err(LumaError::StackError)?;
                }
                
                OpCode::OpSetGlobal => {
                    let name_index = self.read_byte()? as usize;
                    let name = self.get_constant_string(name_index)?;
                    let value = self.stack.peek(0).map_err(LumaError::StackError)?.clone();
                    
                    // Allow setting existing or new global variables
                    self.globals.insert(name, value);
                }
                
                OpCode::OpJump => {
                    let offset = self.read_byte()? as usize;
                    self.ip += offset;
                }
                
                OpCode::OpJumpIfFalse => {
                    let offset = self.read_byte()? as usize;
                    let value = self.stack.peek(0).map_err(LumaError::StackError)?;
                    if !value.is_truthy() {
                        self.ip += offset;
                    }
                }
                
                OpCode::OpLoop => {
                    let offset = self.read_byte()? as usize;
                    self.ip -= offset;
                }
                
                OpCode::OpReturn => {
                    // Return the top value from stack or Nil if empty
                    if !self.stack.is_empty() {
                        let result = self.stack.pop().map_err(LumaError::StackError)?;
                        return Ok(result);
                    } else {
                        return Ok(Value::Nil);
                    }
                }
                
                OpCode::OpConcat => {
                    let b = self.stack.pop().map_err(LumaError::StackError)?;
                    let a = self.stack.pop().map_err(LumaError::StackError)?;
                    let result = Value::String(format!("{}{}", a.to_string(), b.to_string()));
                    self.stack.push(result).map_err(LumaError::StackError)?;
                }
                
                OpCode::OpLoopStart => {
                    // Mark start of potentially hot loop for JIT
                    // Implementation for JIT detection
                }
                
                OpCode::OpLoopEnd => {
                    // Mark end of potentially hot loop for JIT
                    // Check if this loop should be JIT compiled
                    if let Some(count) = self.execution_count.get(&self.ip) {
                        if *count > self.hot_threshold {
                            // TODO: Trigger JIT compilation
                            println!("Hot loop detected at instruction {}", self.ip);
                        }
                    }
                }
                
                _ => {
                    return Err(LumaError::RuntimeError(format!("Unimplemented opcode: {:?}", opcode)));
                }
            }
            
            // Check if we've reached the end
            if self.ip >= self.get_code_len() {
                break;
            }
        }
        
        // If we reach here without return, return the top value or Nil
        if !self.stack.is_empty() {
            Ok(self.stack.pop().map_err(LumaError::StackError)?)
        } else {
            Ok(Value::Nil)
        }
    }

    fn read_byte(&mut self) -> Result<u8> {
        if self.ip >= self.get_code_len() {
            return Err(LumaError::RuntimeError("Instruction pointer out of bounds".into()));
        }
        
        let byte = self.get_chunk().code[self.ip];
        self.ip += 1;
        Ok(byte)
    }

    fn get_constant(&self, index: usize) -> Result<Value> {
        self.get_chunk().constants.get(index)
            .cloned()
            .ok_or_else(|| LumaError::RuntimeError(format!("Constant index {} out of bounds", index)))
    }

    fn get_constant_string(&self, index: usize) -> Result<String> {
        match self.get_constant(index)? {
            Value::String(s) => Ok(s),
            _ => Err(LumaError::RuntimeError("Expected string constant".into())),
        }
    }

    fn get_chunk(&self) -> &Chunk {
        self.chunk.as_ref().expect("No chunk loaded")
    }

    fn get_code_len(&self) -> usize {
        self.get_chunk().code.len()
    }

    fn binary_op<F>(&mut self, op: F) -> Result<()>
    where
        F: FnOnce(Value, Value) -> Result<Value>,
    {
        let b = self.stack.pop().map_err(LumaError::StackError)?;
        let a = self.stack.pop().map_err(LumaError::StackError)?;
        let result = op(a, b)?;
        self.stack.push(result).map_err(LumaError::StackError)?;
        Ok(())
    }

    fn comparison_op<F>(&mut self, op: F) -> Result<()>
    where
        F: FnOnce(f64, f64) -> bool,
    {
        self.binary_op(|a, b| {
            let a_num = a.to_number().map_err(|e| LumaError::RuntimeError(e))?;
            let b_num = b.to_number().map_err(|e| LumaError::RuntimeError(e))?;
            Ok(Value::Boolean(op(a_num, b_num)))
        })
    }

    fn add_values(&self, a: Value, b: Value) -> Result<Value> {
        match (&a, &b) {
            (Value::String(_), _) | (_, Value::String(_)) => {
                Ok(Value::String(format!("{}{}", a.to_string(), b.to_string())))
            }
            _ => {
                let a_num = a.to_number().map_err(|e| LumaError::RuntimeError(e))?;
                let b_num = b.to_number().map_err(|e| LumaError::RuntimeError(e))?;
                Ok(Value::Number(a_num + b_num))
            }
        }
    }

    fn subtract_values(&self, a: Value, b: Value) -> Result<Value> {
        let a_num = a.to_number().map_err(|e| LumaError::RuntimeError(e))?;
        let b_num = b.to_number().map_err(|e| LumaError::RuntimeError(e))?;
        Ok(Value::Number(a_num - b_num))
    }

    fn multiply_values(&self, a: Value, b: Value) -> Result<Value> {
        let a_num = a.to_number().map_err(|e| LumaError::RuntimeError(e))?;
        let b_num = b.to_number().map_err(|e| LumaError::RuntimeError(e))?;
        Ok(Value::Number(a_num * b_num))
    }

    fn divide_values(&self, a: Value, b: Value) -> Result<Value> {
        let a_num = a.to_number().map_err(|e| LumaError::RuntimeError(e))?;
        let b_num = b.to_number().map_err(|e| LumaError::RuntimeError(e))?;
        
        if b_num == 0.0 {
            return Err(LumaError::RuntimeError("Division by zero".into()));
        }
        
        Ok(Value::Number(a_num / b_num))
    }

    fn modulo_values(&self, a: Value, b: Value) -> Result<Value> {
        let a_num = a.to_number().map_err(|e| LumaError::RuntimeError(e))?;
        let b_num = b.to_number().map_err(|e| LumaError::RuntimeError(e))?;
        
        if b_num == 0.0 {
            return Err(LumaError::RuntimeError("Modulo by zero".into()));
        }
        
        Ok(Value::Number(a_num % b_num))
    }

    fn negate_value(&self, value: Value) -> Result<Value> {
        let num = value.to_number().map_err(|e| LumaError::RuntimeError(e))?;
        Ok(Value::Number(-num))
    }

    pub fn get_execution_stats(&self) -> Vec<(usize, u64)> {
        let mut stats: Vec<_> = self.execution_count.iter()
            .map(|(&offset, &count)| (offset, count))
            .collect();
        stats.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by execution count, descending
        stats
    }

    fn get_current_line(&self) -> usize {
        if let Some(chunk) = &self.chunk {
            // Find the closest line number for current instruction pointer
            if self.ip < chunk.lines.len() && chunk.lines[self.ip] > 0 {
                chunk.lines[self.ip]
            } else {
                // Look backwards for a valid line number
                for i in (0..self.ip.min(chunk.lines.len())).rev() {
                    if chunk.lines[i] > 0 {
                        return chunk.lines[i];
                    }
                }
                // If no valid line found, estimate based on instruction position
                (self.ip / 3) + 1  // Rough estimate: ~3 instructions per line
            }
        } else {
            1 // Default to line 1 if no chunk
        }
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.chunk = None;
        self.ip = 0;
        self.stack.clear();
        self.globals.clear();
        self.execution_count.clear();
        self.start_time = None;
    }
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}