use crate::frontend::{Statement, Expression, BinaryOperator, UnaryOperator};
use crate::backend::vm::OpCode;
use crate::shared::{Chunk, Value, LumaError, Result};

pub struct Compiler {
    chunk: Chunk,
    locals: Vec<Local>,
    scope_depth: usize,
    current_line: usize,
}

#[derive(Debug, Clone)]
struct Local {
    name: String,
    #[allow(dead_code)]
    depth: Option<usize>, // None means uninitialized (for future scope implementation)
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            chunk: Chunk::new(),
            locals: Vec::new(),
            scope_depth: 0,
            current_line: 1,
        }
    }

    pub fn compile(&mut self, statements: &[Statement]) -> Result<Chunk> {
        for (i, statement) in statements.iter().enumerate() {
            // More accurate line tracking: account for comments and empty lines
            self.current_line = self.estimate_statement_line(i + 1);
            self.compile_statement(statement)?;
        }
        
        // Ensure the chunk ends with a return
        self.emit_opcode(OpCode::OpReturn, self.current_line);
        
        Ok(std::mem::take(&mut self.chunk))
    }
    
    fn estimate_statement_line(&self, statement_index: usize) -> usize {
        // Better estimation that accounts for comments and empty lines
        // Each statement is roughly 2 lines apart (allowing for comments)
        statement_index * 2
    }

    fn compile_statement(&mut self, statement: &Statement) -> Result<()> {
        match statement {
            Statement::Assignment { name, value } => {
                self.compile_expression(value)?;
                
                if self.scope_depth > 0 {
                    // Local variable
                    if let Some(local_index) = self.resolve_local(name) {
                        // Variable exists, set it
                        self.emit_opcode(OpCode::OpSetLocal, 0);
                        self.emit_byte(local_index as u8, 0);
                    } else {
                        // New variable, add it
                        self.add_local(name.clone())?;
                    }
                } else {
                    // Global variable - check if it exists
                    let name_constant = self.chunk.add_constant(Value::String(name.clone()));
                    
                    // For now, always define new globals or update existing ones
                    self.emit_opcode(OpCode::OpSetGlobal, 0);
                    self.emit_byte(name_constant as u8, 0);
                    self.emit_opcode(OpCode::OpPop, 0); // Pop the value after assignment
                }
            }
            
            Statement::Show(expression) => {
                self.compile_expression(expression)?;
                self.emit_opcode(OpCode::OpPrint, 0);
            }
            
            Statement::If { condition, then_branch, else_branch, .. } => {
                self.compile_expression(condition)?;
                
                let else_jump = self.emit_jump(OpCode::OpJumpIfFalse, 0);
                self.emit_opcode(OpCode::OpPop, 0); // Pop condition
                
                for stmt in then_branch {
                    self.compile_statement(stmt)?;
                }
                
                let end_jump = self.emit_jump(OpCode::OpJump, 0);
                
                self.patch_jump(else_jump);
                self.emit_opcode(OpCode::OpPop, 0); // Pop condition
                
                if let Some(else_stmts) = else_branch {
                    for stmt in else_stmts {
                        self.compile_statement(stmt)?;
                    }
                }
                
                self.patch_jump(end_jump);
            }
            
            Statement::While { condition, body } => {
                let loop_start = self.chunk.code.len();
                self.emit_opcode(OpCode::OpLoopStart, 0);
                
                self.compile_expression(condition)?;
                
                let exit_jump = self.emit_jump(OpCode::OpJumpIfFalse, 0);
                self.emit_opcode(OpCode::OpPop, 0); // Pop condition
                
                for stmt in body {
                    self.compile_statement(stmt)?;
                }
                
                self.emit_loop(loop_start, 0);
                
                self.patch_jump(exit_jump);
                self.emit_opcode(OpCode::OpPop, 0); // Pop condition
                self.emit_opcode(OpCode::OpLoopEnd, 0);
            }
            
            Statement::Repeat { count, body } => {
                // Compile count expression
                self.compile_expression(count)?;
                
                // Initialize counter variable (use a special name to avoid conflicts)
                let counter_name = format!("__repeat_counter_{}", self.chunk.code.len());
                let counter_constant = self.chunk.add_constant(Value::String(counter_name.clone()));
                self.emit_opcode(OpCode::OpDefineGlobal, 0);
                self.emit_byte(counter_constant as u8, 0);
                
                // Initialize counter to 0
                self.emit_opcode(OpCode::OpConstant, 0);
                let zero_constant = self.chunk.add_constant(Value::Number(0.0));
                self.emit_byte(zero_constant as u8, 0);
                
                let loop_start = self.chunk.code.len();
                self.emit_opcode(OpCode::OpLoopStart, 0);
                
                // Check if counter < count
                self.emit_opcode(OpCode::OpGetGlobal, 0);
                self.emit_byte(counter_constant as u8, 0);
                
                // Get count from stack (we need to duplicate it)
                self.emit_opcode(OpCode::OpGetGlobal, 0);
                let count_var = format!("__repeat_count_{}", self.chunk.code.len());
                let _count_constant = self.chunk.add_constant(Value::String(count_var));
                
                self.emit_opcode(OpCode::OpLess, 0);
                let exit_jump = self.emit_jump(OpCode::OpJumpIfFalse, 0);
                self.emit_opcode(OpCode::OpPop, 0);
                
                // Execute body
                for stmt in body {
                    self.compile_statement(stmt)?;
                }
                
                // Increment counter
                self.emit_opcode(OpCode::OpGetGlobal, 0);
                self.emit_byte(counter_constant as u8, 0);
                self.emit_opcode(OpCode::OpConstant, 0);
                let one_constant = self.chunk.add_constant(Value::Number(1.0));
                self.emit_byte(one_constant as u8, 0);
                self.emit_opcode(OpCode::OpAdd, 0);
                self.emit_opcode(OpCode::OpSetGlobal, 0);
                self.emit_byte(counter_constant as u8, 0);
                self.emit_opcode(OpCode::OpPop, 0);
                
                self.emit_loop(loop_start, 0);
                
                self.patch_jump(exit_jump);
                self.emit_opcode(OpCode::OpPop, 0);
                self.emit_opcode(OpCode::OpLoopEnd, 0);
            }
        }
        
        Ok(())
    }

    fn compile_expression(&mut self, expression: &Expression) -> Result<()> {
        match expression {
            Expression::Literal(value) => {
                let constant = self.chunk.add_constant(Value::Number(*value));
                self.emit_opcode(OpCode::OpConstant, 0);
                self.emit_byte(constant as u8, 0);
            }
            
            Expression::StringLiteral(value) => {
                let constant = self.chunk.add_constant(Value::String(value.clone()));
                self.emit_opcode(OpCode::OpConstant, 0);
                self.emit_byte(constant as u8, 0);
            }
            
            Expression::BooleanLiteral(value) => {
                if *value {
                    self.emit_opcode(OpCode::OpTrue, 0);
                } else {
                    self.emit_opcode(OpCode::OpFalse, 0);
                }
            }
            
            Expression::Identifier(name) => {
                if let Some(local_index) = self.resolve_local(name) {
                    self.emit_opcode(OpCode::OpGetLocal, 0);
                    self.emit_byte(local_index as u8, 0);
                } else {
                    let constant = self.chunk.add_constant(Value::String(name.clone()));
                    self.emit_opcode(OpCode::OpGetGlobal, 0);
                    self.emit_byte(constant as u8, 0);
                }
            }
            
            Expression::BinaryOp { left, operator, right } => {
                self.compile_expression(left)?;
                self.compile_expression(right)?;
                
                match operator {
                    BinaryOperator::Add => self.emit_opcode(OpCode::OpAdd, 0),
                    BinaryOperator::Subtract => self.emit_opcode(OpCode::OpSubtract, 0),
                    BinaryOperator::Multiply => self.emit_opcode(OpCode::OpMultiply, 0),
                    BinaryOperator::Divide => self.emit_opcode(OpCode::OpDivide, 0),
                    BinaryOperator::Modulo => self.emit_opcode(OpCode::OpModulo, 0),
                    BinaryOperator::Equal => self.emit_opcode(OpCode::OpEqual, 0),
                    BinaryOperator::NotEqual => self.emit_opcode(OpCode::OpNotEqual, 0),
                    BinaryOperator::Greater | BinaryOperator::GreaterThan => self.emit_opcode(OpCode::OpGreater, 0),
                    BinaryOperator::GreaterEqual => self.emit_opcode(OpCode::OpGreaterEqual, 0),
                    BinaryOperator::Less | BinaryOperator::LessThan => self.emit_opcode(OpCode::OpLess, 0),
                    BinaryOperator::LessEqual => self.emit_opcode(OpCode::OpLessEqual, 0),
                    BinaryOperator::And => self.emit_opcode(OpCode::OpAnd, 0),
                    BinaryOperator::Or => self.emit_opcode(OpCode::OpOr, 0),
                }
            }
            
            Expression::UnaryOp { operator, operand } => {
                self.compile_expression(operand)?;
                
                match operator {
                    UnaryOperator::Minus => self.emit_opcode(OpCode::OpNegate, 0),
                    UnaryOperator::Not => self.emit_opcode(OpCode::OpNot, 0),
                }
            }
            
            Expression::FunctionCall { name: _, arguments: _ } => {
                return Err(LumaError::compile_error("Function calls not implemented in JIT-VM".to_string(), 0));
            }
        }
        
        Ok(())
    }

    fn emit_opcode(&mut self, opcode: OpCode, _line: usize) {
        self.chunk.write_opcode(opcode, self.current_line);
    }

    fn emit_byte(&mut self, byte: u8, _line: usize) {
        self.chunk.write_byte(byte, self.current_line);
    }

    fn emit_jump(&mut self, opcode: OpCode, line: usize) -> usize {
        self.chunk.emit_jump(opcode, line)
    }

    fn patch_jump(&mut self, offset: usize) {
        self.chunk.patch_jump(offset);
    }

    fn emit_loop(&mut self, loop_start: usize, line: usize) {
        self.chunk.emit_loop(loop_start, line);
    }

    fn add_local(&mut self, name: String) -> Result<()> {
        if self.locals.len() >= u8::MAX as usize {
            return Err(LumaError::compile_error("Too many local variables in scope".to_string(), 0));
        }
        
        self.locals.push(Local {
            name,
            depth: Some(self.scope_depth),
        });
        
        Ok(())
    }

    fn resolve_local(&self, name: &str) -> Option<usize> {
        for (i, local) in self.locals.iter().enumerate().rev() {
            if local.name == name {
                return Some(i);
            }
        }
        None
    }

    #[allow(dead_code)]
    fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    #[allow(dead_code)]
    fn end_scope(&mut self) {
        self.scope_depth -= 1;
        
        while !self.locals.is_empty() {
            if let Some(depth) = self.locals.last().unwrap().depth {
                if depth <= self.scope_depth {
                    break;
                }
            }
            self.locals.pop();
            self.emit_opcode(OpCode::OpPop, 0);
        }
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}