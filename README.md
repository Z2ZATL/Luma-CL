# Luma Custom Language v0.1

A simple, readable programming language interpreter built in Rust.

## Features

- Natural language syntax: `let name be value`
- Variable assignment: `variable is new_value`
- Output display: `show expression`
- String literals: both `'single'` and `"double"` quotes supported
- Boolean values: `true` and `false` literals
- Arithmetic operators: `+`, `-`, `*`, `/`, `%`
- Comparison operators: `==`, `!=`, `>`, `<`, `>=`, `<=` and natural language `is`, `is not`
- Logical operators: `and`, `or`, `not`
- Control flow: `if condition then ... else if ... else ...` statements
- Comment support: `# single-line comment` and `## multi-line comment ##`
- Interactive REPL mode
- Web interface for browser-based testing

## Quick Start

### Command Line
```bash
# Build the project
cargo build --release

# Run REPL mode
cargo run

# Execute a .luma file
cargo run examples/hello.luma
```

### Web Interface
```bash
# Start web server on port 5000
cd web && python3 simple_server.py
```

## Example Code

```luma
# Variable declaration
let product_name be "Rust-powered CPU"
let price be 2500
let quantity is 2

# Calculation
total_cost is price * quantity

# Output
show "Product:"
show product_name
show "Total cost:"
show total_cost
```

## Project Structure

- `src/` - Core interpreter source code
- `examples/` - Example .luma programs
- `tests/` - Test files and programs
- `scripts/` - Build and test scripts
- `web/` - Web interface files

## License

MIT License