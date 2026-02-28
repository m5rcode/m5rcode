/// m5repl - Interactive REPL for m5rcode
use std::io::{self, Write};
use std::fs;
use std::env;

// Import the compiler library
extern crate m5rcode;
use m5rcode::{Lexer, Parser, Interpreter};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        run_file(&args[1]);
    } else {
        repl();
    }
}

fn run_file(filename: &str) {
    let source = fs::read_to_string(filename)
        .unwrap_or_else(|e| {
            eprintln!("Error reading file '{}': {}", filename, e);
            std::process::exit(1);
        });
    
    execute(&source);
}

fn repl() {
    println!("m5rcode REPL v0.1.0");
    println!("Type 'exit' to quit\n");
    
    let mut buffer = String::new();
    let mut interpreter = Interpreter::new();
    
    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();
        
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        
        let line = line.trim();
        if line == "exit" || line == "quit" {
            break;
        }
        
        if line.is_empty() {
            continue;
        }
        
        buffer.push_str(line);
        buffer.push('\n');
        
        if is_complete(&buffer) {
            execute_with_interpreter(&buffer, &mut interpreter);
            buffer.clear();
        } else {
            print!("... ");
            io::stdout().flush().unwrap();
        }
    }
}

fn is_complete(code: &str) -> bool {
    let open_braces = code.matches('{').count();
    let close_braces = code.matches('}').count();
    let open_parens = code.matches('(').count();
    let close_parens = code.matches(')').count();
    
    open_braces == close_braces && open_parens == close_parens
}

fn execute(source: &str) {
    let mut interpreter = Interpreter::new();
    execute_with_interpreter(source, &mut interpreter);
}

fn execute_with_interpreter(source: &str, interpreter: &mut Interpreter) {
    let mut lexer = Lexer::new(source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Lexer error: {}", e);
            return;
        }
    };
    
    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Parser error: {}", e);
            return;
        }
    };
    
    if let Err(e) = interpreter.execute(&ast) {
        eprintln!("Runtime error: {}", e);
    }
}
