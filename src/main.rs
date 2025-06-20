use std::env;
use std::fs;
use std::time::Instant;

mod frontend;
mod backend;
mod shared;
mod ffi;

use frontend::{Lexer, Parser, Compiler};
use backend::vm::VM;
use shared::{LumaError, Result};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 1 {
        run_repl();
    } else if args.len() == 2 {
        let filename = &args[1];
        if let Err(e) = execute_file(filename) {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    } else {
        eprintln!("Usage: {} [script]", args[0]);
        std::process::exit(1);
    }
}

fn run_repl() {
    println!("Luma JIT-VM Language v0.2.0");
    println!("Type 'exit' to quit, 'help' for commands");
    
    let mut vm = VM::new();
    
    loop {
        print!("luma> ");
        if let Err(_) = std::io::Write::flush(&mut std::io::stdout()) {
            break;
        }
        
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();
                
                if input == "exit" || input == "quit" || input == ":q" {
                    println!("Goodbye!");
                    break;
                }
                
                if input == "help" || input == ":help" {
                    print_help();
                    continue;
                }
                
                if input == "stats" || input == ":stats" {
                    print_stats(&vm);
                    continue;
                }
                
                if input.is_empty() {
                    continue;
                }
                
                if let Err(e) = execute_source_vm(input, &mut vm) {
                    eprintln!("Error: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }
}

fn execute_file(filename: &str) -> Result<()> {
    let source = fs::read_to_string(filename)
        .map_err(|e| LumaError::IoError(e))?;
    
    if source.trim().is_empty() {
        println!("Code executed successfully!");
        return Ok(());
    }
    
    let start_time = Instant::now();
    let mut vm = VM::new();
    let result = execute_source_vm(&source, &mut vm);
    let execution_time = start_time.elapsed();
    
    // Print performance info
    println!("\n⚡ Execution time: {:.7}ms", execution_time.as_secs_f64() * 1000.0);
    
    result
}

fn execute_source_vm(source: &str, vm: &mut VM) -> Result<()> {
    // Frontend: Compile to bytecode
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize()?;
    
    let mut parser = Parser::new(tokens);
    let statements = parser.parse()?;
    
    let mut compiler = Compiler::new();
    let chunk = compiler.compile(&statements)?;
    
    // Backend: Execute on VM
    vm.interpret(chunk)?;
    
    Ok(())
}

fn print_help() {
    println!("Luma JIT-VM Language REPL Commands:");
    println!("  help, :help    - Show this help message");
    println!("  stats, :stats  - Show execution statistics");
    println!("  exit, quit, :q - Exit the REPL");
    println!();
    println!("Language Syntax:");
    println!("  let <name> be <value>  - Assign value to variable");
    println!("  <name> is <value>      - Reassign variable (strings)");
    println!("  <name> = <value>       - Reassign variable (numbers)");
    println!("  show <expression>      - Display result of expression");
    println!("  # <comment>            - Comment (ignored)");
    println!();
    println!("Control Flow:");
    println!("  if <condition> then ... else ... - Conditional statements");
    println!("  while <condition> then ... - Loop while condition is true");
    println!("  repeat <count> times then ... - Loop specific number of times");
    println!();
    println!("Operators: + - * / ( ) == != > < >= <= and or not");
    println!();
    println!("Examples:");
    println!("  let x be 42");
    println!("  x = x + 1");
    println!("  show x");
    println!("  if x > 40 then");
    println!("      show \"Large number\"");
}

fn print_stats(vm: &VM) {
    let stats = vm.get_execution_stats();
    
    if stats.is_empty() {
        println!("No execution statistics available.");
        return;
    }
    
    println!("Execution Statistics (Top 10 Hot Spots):");
    println!("{:<10} {:<15} {}", "Offset", "Executions", "Status");
    println!("{:-<40}", "");
    
    for (_i, (offset, count)) in stats.iter().take(10).enumerate() {
        let status = if *count > 1000 {
            "HOT - JIT Candidate"
        } else if *count > 100 {
            "Warm"
        } else {
            "Cold"
        };
        
        println!("{:<10} {:<15} {}", offset, count, status);
    }
}