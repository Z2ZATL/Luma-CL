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
    let chunk = compiler.compile_with_source(&statements, source).map_err(|e| e.to_string())?;

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

// === Comment Tests ===

#[test]
fn test_comments_are_ignored() {
    // Read test file with various comment styles
    let source = std::fs::read_to_string("tests/test_comments.luma").unwrap();
    
    // Expected: should execute and return result of "show 20 + 5" = 25
    let result = run_code(&source).unwrap();
    assert_eq!(result, Value::Number(25.0));
}

#[test]
fn test_comment_at_end_of_expression() {
    let source = "show 42 # This is the answer";
    let result = run_code(source).unwrap();
    assert_eq!(result, Value::Number(42.0));
}

#[test]
fn test_single_line_comment() {
    let source = r#"
        # This is a comment line
        show 100
        # Another comment
    "#;
    let result = run_code(source).unwrap();
    assert_eq!(result, Value::Number(100.0));
}

#[test]
fn test_multiline_comment_blocks() {
    let source = r#"
        ##
        ## This is a multi-line comment block
        ## with multiple lines of text
        ##
        show 55
        ##
        ## Another comment block
        ##
    "#;
    let result = run_code(source).unwrap();
    assert_eq!(result, Value::Number(55.0));
}

#[test]
fn test_mixed_comment_styles() {
    let source = r#"
        ## Multi-line comment start
        let x be 10  # Single line comment after code
        ## Multi-line comment end
        # Pure single line comment
        show x * 2   ## Another comment style ## 
    "#;
    let result = run_code(source).unwrap();
    assert_eq!(result, Value::Number(20.0));
}