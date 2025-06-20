use crate::shared::Value;

const STACK_MAX: usize = 256;

#[derive(Debug)]
pub struct Stack {
    values: Vec<Value>,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            values: Vec::with_capacity(STACK_MAX),
        }
    }

    pub fn push(&mut self, value: Value) -> Result<(), String> {
        if self.values.len() >= STACK_MAX {
            return Err("Stack overflow".to_string());
        }
        self.values.push(value);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<Value, String> {
        self.values.pop().ok_or_else(|| "Stack underflow".to_string())
    }

    pub fn peek(&self, distance: usize) -> Result<&Value, String> {
        if distance >= self.values.len() {
            return Err("Stack underflow".to_string());
        }
        let index = self.values.len() - 1 - distance;
        Ok(&self.values[index])
    }

    #[allow(dead_code)]
    pub fn peek_mut(&mut self, distance: usize) -> Result<&mut Value, String> {
        if distance >= self.values.len() {
            return Err("Stack underflow".to_string());
        }
        let index = self.values.len() - 1 - distance;
        Ok(&mut self.values[index])
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.values.clear();
    }

    #[allow(dead_code)]
    pub fn reset_to(&mut self, slot: usize) {
        if slot <= self.values.len() {
            self.values.truncate(slot);
        }
    }

    // For debugging
    #[allow(dead_code)]
    pub fn print_stack(&self) {
        print!("          ");
        for value in &self.values {
            print!("[ {} ]", value);
        }
        println!();
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}