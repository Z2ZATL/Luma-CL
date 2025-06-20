// Integration tests for Luma Bytecode VM
use luma::frontend::lexer::Lexer;
use luma::frontend::parser::Parser;
use luma::frontend::compiler::Compiler;
use luma::backend::vm::vm::VM;
use luma::shared::value::Value;

// Helper function to run code through the complete pipeline
fn run_code(source: &str) -> Result<Value, String> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().map_err(|e| e.to_string())?;

    let mut parser = Parser::new(tokens);
    let statements = parser.parse().map_err(|e| e.to_string())?;

    let mut compiler = Compiler::new();
    let chunk = compiler.compile(&statements).map_err(|e| e.to_string())?;

    let mut vm = VM::new();
    vm.interpret(chunk).map_err(|e| e.to_string())
}

#[test]
fn test_simple_number_expression() {
    let source = "show 123";
    let result = run_code(source).unwrap();
    assert_eq!(result, Value::Number(123.0));
}

#[test]
fn test_simple_arithmetic() {
    let source = "show 10 + 5";
    let result = run_code(source).unwrap();
    assert_eq!(result, Value::Number(15.0));
}

#[test]
fn test_operator_precedence() {
    // Test that 2 * 5 is done before 10 +
    let source = "show 10 + 2 * 5"; // 10 + 10 = 20
    let result = run_code(source).unwrap();
    assert_eq!(result, Value::Number(20.0));
}

#[test]
fn test_parentheses_expression() {
    // Test that parentheses override precedence
    let source = "show (10 + 2) * 5"; // 12 * 5 = 60
    let result = run_code(source).unwrap();
    assert_eq!(result, Value::Number(60.0));
}

#[test]
fn test_complex_expression() {
    let source = "show (100 - 20) / (2 + 2) + 5 * 2"; // 80 / 4 + 10 -> 20 + 10 = 30
    let result = run_code(source).unwrap();
    assert_eq!(result, Value::Number(30.0));
}

#[test]
fn test_subtraction() {
    let source = "show 100 - 25";
    let result = run_code(source).unwrap();
    assert_eq!(result, Value::Number(75.0));
}

#[test]
fn test_multiplication() {
    let source = "show 10 * 5";
    let result = run_code(source).unwrap();
    assert_eq!(result, Value::Number(50.0));
}

#[test]
fn test_division() {
    let source = "show 100 / 4";
    let result = run_code(source).unwrap();
    assert_eq!(result, Value::Number(25.0));
}

#[test]
fn test_division_before_subtraction() {
    let source = "show 20 - 8 / 2"; // 8/2=4, 20-4=16
    let result = run_code(source).unwrap();
    assert_eq!(result, Value::Number(16.0));
}

#[test]
fn test_zero() {
    let source = "show 0";
    let result = run_code(source).unwrap();
    assert_eq!(result, Value::Number(0.0));
}

// === Phase 2: Variable Tests ===

#[test]
fn test_variable_declaration() {
    let source = r#"
        let price be 2500
        show price
    "#;
    let result = run_code(source).unwrap();
    assert_eq!(result, Value::Number(2500.0));
}

#[test]
fn test_variable_in_expression() {
    let source = r#"
        let x be 10
        let y be 5
        show x + y
    "#;
    let result = run_code(source).unwrap();
    assert_eq!(result, Value::Number(15.0));
}

#[test]
fn test_complex_variable_expression() {
    let source = r#"
        let a be 5
        let b be 3
        show (a + b) * (a - b)
    "#;
    let result = run_code(source).unwrap();
    assert_eq!(result, Value::Number(16.0)); // (5+3) * (5-3) = 8 * 2 = 16
}