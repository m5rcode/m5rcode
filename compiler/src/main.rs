use m5rcode::{Lexer, Parser, Interpreter};
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: m5r <file.m5>");
        process::exit(1);
    }
    
    let filename = &args[1];
    let source = fs::read_to_string(filename)
        .unwrap_or_else(|e| {
            eprintln!("Error reading file '{}': {}", filename, e);
            process::exit(1);
        });
    
    // Lex
    let mut lexer = Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Lexer error: {}", e);
            process::exit(1);
        }
    };
    
    // Parse
    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Parser error: {}", e);
            process::exit(1);
        }
    };
    
    // Interpret
    let mut interpreter = Interpreter::new();
    if let Err(e) = interpreter.execute(&ast) {
        eprintln!("Runtime error: {}", e);
        process::exit(1);
    }
}
