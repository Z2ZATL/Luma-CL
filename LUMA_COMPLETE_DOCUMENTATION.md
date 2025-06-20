# Luma Custom Language - Complete Program Documentation

## Overview
Luma is a custom programming language interpreter built in Rust with a natural language syntax design. The language emphasizes readability and simplicity, featuring Thai-English mixed syntax for variable operations and control flow structures.

## Core Language Features

### 1. Variable Declaration and Assignment
```luma
# Initial declaration
let x be 42
let name be "Mori"

# Reassignment - Two methods supported
x = x + 1        # Use = for numeric operations
name is "New Name"  # Use is for string operations (both work for any type)
```

### 2. Output Display
```luma
show x           # Display variable value
show "Hello"     # Display string literal
show x + 10      # Display expression result
```

### 3. Arithmetic Operations
- Addition: `+`
- Subtraction: `-`
- Multiplication: `*`
- Division: `/`
- Modulo: `%`
- Parentheses for precedence: `()`

### 4. Boolean Logic and Comparisons
```luma
# Boolean literals
let flag be true
let status be false

# Comparison operators
x == 42          # Equal
x != 0           # Not equal
x > 10           # Greater than
x < 100          # Less than
x >= 42          # Greater or equal
x <= 50          # Less or equal

# Natural language comparisons
x is 42          # Equal (natural syntax)
x is not 0       # Not equal (natural syntax)

# Logical operators
true and false   # Logical AND
true or false    # Logical OR
not true         # Logical NOT
```

### 5. Control Flow Structures

#### If Statements
```luma
if x > 40 then
    show "Large number"
else if x > 20 then
    show "Medium number"
else
    show "Small number"
```

#### While Loops
```luma
let n be 5
while n > 0 then
    show n
    n = n - 1
```

#### Repeat Loops
```luma
repeat 5 times then
    show "Hello"
```

### 6. Comments
```luma
# Single line comment
show x  # End of line comment

## 
Multi-line comment
Can span multiple lines
##
```

### 7. String Operations
```luma
let greeting be "Hello"
let name be "World"
show greeting + " " + name  # String concatenation
```

## Technical Architecture

### Core Components

#### 1. Lexer (`src/lexer.rs`)
- Tokenizes source code character by character
- Handles keywords, operators, literals, and identifiers
- Supports both single-quote and double-quote strings
- Manages line/column tracking for error reporting
- Special handling for multi-character operators (==, !=, >=, <=)
- Advanced lookahead for "is not" tokenization

#### 2. Token System (`src/token.rs`)
**Keywords:**
- `Let`, `Be`, `Is`, `Show`
- `If`, `Then`, `Else`, `ElseIf`
- `While`, `Repeat`, `Times`
- `True`, `False`
- `And`, `Or`, `Not`

**Operators:**
- Arithmetic: `Plus`, `Minus`, `Multiply`, `Divide`, `Modulo`
- Assignment: `Assign` (=)
- Comparison: `Equal` (==), `NotEqual` (!=), `GreaterThan`, `LessThan`, `GreaterEqual`, `LessEqual`

**Literals:**
- `Number(f64)` - Floating point numbers
- `String(String)` - Text strings
- `Identifier(String)` - Variable names

#### 3. Abstract Syntax Tree (`src/ast.rs`)
**Statement Types:**
```rust
pub enum Statement {
    Assignment { name: String, value: Expression },
    Show(Expression),
    If {
        condition: Expression,
        then_statements: Vec<Statement>,
        else_statements: Option<Vec<Statement>>,
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
```

**Expression Types:**
```rust
pub enum Expression {
    Number(f64),
    String(String),
    Boolean(bool),
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
}
```

#### 4. Parser (`src/parser.rs`)
- Recursive descent parser
- Handles operator precedence correctly
- Supports block-based statement parsing
- Special handling for if/else if/else chains
- Proper termination of control structures

**Operator Precedence (highest to lowest):**
1. Unary operators (`not`, `-`)
2. Multiplication, Division, Modulo (`*`, `/`, `%`)
3. Addition, Subtraction (`+`, `-`)
4. Comparison (`>`, `<`, `>=`, `<=`, `==`, `!=`, `is`, `is not`)
5. Logical AND (`and`)
6. Logical OR (`or`)

#### 5. Interpreter (`src/interpreter.rs`)
- Tree-walking interpreter
- HashMap-based variable storage
- Infinite loop protection (10,000,000 iteration limit)
- Comprehensive error handling
- Type coercion for string concatenation

#### 6. Error System (`src/error.rs`)
**Error Types:**
- `LexError` - Tokenization errors
- `ParseError` - Syntax analysis errors
- `RuntimeError` - Execution errors
- `IoError` - File system errors

### Execution Modes

#### 1. REPL Mode
```bash
cargo run
```
Interactive Read-Eval-Print Loop with commands:
- `help` - Show syntax guide
- `vars` - Display all variables
- `exit` - Quit the interpreter

#### 2. File Execution Mode
```bash
cargo run filename.luma
```
Execute Luma source files directly.

#### 3. Web Interface Mode
```bash
cd web && python3 simple_server.py
```
Browser-based code editor with:
- Syntax highlighting
- Auto-indentation
- Smart backspace (remove all indentation at once)
- Real-time execution timing
- Example code library

## Performance Characteristics

### Timing Measurements
- Real execution times: 3-8ms for typical programs
- Sub-millisecond core processing for simple operations
- 7 decimal place precision timing display
- Infinite loop protection prevents system lockup

### Optimization Features
- Zero external dependencies (pure Rust stdlib)
- Compiled to native machine code
- Efficient HashMap-based variable storage
- Minimal memory allocation during execution

## File Structure
```
luma-interpreter/
├── src/                    # Core interpreter source
│   ├── main.rs            # Entry point and CLI
│   ├── token.rs           # Token definitions
│   ├── lexer.rs           # Lexical analysis
│   ├── parser.rs          # Syntax analysis  
│   ├── ast.rs             # Abstract syntax tree
│   ├── interpreter.rs     # Execution engine
│   ├── error.rs           # Error handling
│   └── repl.rs            # Interactive mode
├── web/                   # Web interface
│   ├── index.html         # Browser editor
│   ├── simple_server.py   # Python web server
│   └── server.py          # Alternative server
├── examples/              # Example programs
├── tests/                 # Test programs
├── scripts/               # Automation scripts
├── target/                # Compiled binaries
├── Cargo.toml            # Rust configuration
└── README.md             # Project documentation
```

## Example Programs

### 1. Basic Operations
```luma
let x be 42
let y be 10
show x + y * 2    # Output: 62
```

### 2. Conditional Logic
```luma
let age be 25
if age >= 18 then
    show "Adult"
else
    show "Minor"
```

### 3. Loop Structures
```luma
# While loop countdown
let n be 5
while n > 0 then
    show n
    n = n - 1

# Repeat loop
repeat 3 times then
    show "Hello World"
```

### 4. Boolean Logic
```luma
let x be 15
if x > 10 and x < 20 then
    show "In range"

let flag be true
if not flag then
    show "Flag is false"
```

### 5. String Operations
```luma
let first be "Hello"
let second be "World"
let combined be first + " " + second
show combined    # Output: Hello World
```

## Build and Deployment

### Development Build
```bash
cargo build
```

### Release Build
```bash
cargo build --release
```

### Running Tests
```bash
cargo test
```

### Web Server
```bash
cd web
python3 simple_server.py
# Access at http://localhost:5000
```

## Language Grammar (EBNF-style)

```ebnf
Program = Statement*

Statement = Assignment | Show | If | While | Repeat

Assignment = "let" Identifier ("be" | "is") Expression
           | Identifier ("is" | "=") Expression

Show = "show" Expression

If = "if" Expression "then" Statement* 
     ("else" "if" Expression "then" Statement*)*
     ("else" Statement*)?

While = "while" Expression "then" Statement*

Repeat = "repeat" Expression "times" "then" Statement*

Expression = OrExpression

OrExpression = AndExpression ("or" AndExpression)*

AndExpression = ComparisonExpression ("and" ComparisonExpression)*

ComparisonExpression = ArithmeticExpression 
                     (("==" | "!=" | ">" | "<" | ">=" | "<=" | "is" | "is not") 
                      ArithmeticExpression)*

ArithmeticExpression = MultiplicativeExpression 
                     (("+" | "-") MultiplicativeExpression)*

MultiplicativeExpression = UnaryExpression 
                         (("*" | "/" | "%") UnaryExpression)*

UnaryExpression = ("not" | "-") UnaryExpression | Primary

Primary = Number | String | Boolean | Identifier | "(" Expression ")"

Number = [0-9]+ ("." [0-9]+)?
String = '"' [^"]* '"' | "'" [^']* "'"
Boolean = "true" | "false"
Identifier = [a-zA-Z_][a-zA-Z0-9_]*
```

## Recent Updates (June 19, 2025)

1. **Real Timing Measurement** - Fixed fake timing display, now shows authentic execution times
2. **Flexible Assignment Syntax** - Added support for `=` operator alongside `is` for variable assignment
3. **Smart Editing Features** - Auto-indentation and smart backspace for web interface
4. **Syntax Consistency** - Unified all control structures to use `then` keyword
5. **High Precision Timing** - 7 decimal place timing display for microsecond accuracy
6. **Critical Parser Fix** - Resolved infinite loop issues in nested control structures
7. **Complete v0.2 Implementation** - All priority features implemented and tested

## Security and Safety

- Infinite loop protection with configurable iteration limits
- Input validation and sanitization
- Memory-safe Rust implementation
- No external network access in core interpreter
- Comprehensive error handling prevents crashes

This documentation represents the complete technical specification and implementation details of the Luma Custom Language as of June 19, 2025.