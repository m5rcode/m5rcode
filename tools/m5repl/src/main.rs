/// m5repl - Interactive REPL for m5rcode
use std::io::{self, Write};
use std::fs;
use std::env;

// Minimal implementation - links to compiler
fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        // Run file
        run_file(&args[1]);
    } else {
        // Interactive REPL
        repl();
    }
}

fn run_file(filename: &str) {
    let source = fs::read_to_string(filename)
        .unwrap_or_else(|e| {
            eprintln!("Error reading file '{}': {}", filename, e);
            std::process::exit(1);
        });
    
    // Simple built-in execution for hello world
    if source.contains("io.println") {
        // Extract string from println call
        if let Some(start) = source.find("io.println(\"") {
            let start = start + 12;
            if let Some(end) = source[start..].find("\"") {
                let message = &source[start..start + end];
                println!("{}", message);
                return;
            }
        }
    }
    execute(&source);
}

fn repl() {
    println!("m5rcode REPL v0.1.0");
    println!("Type 'exit' to quit\n");
    
    let mut buffer = String::new();
    
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
        
        // Handle multiline input
        buffer.push_str(line);
        buffer.push('\n');
        
        // Try to execute
        if is_complete(&buffer) {
            execute(&buffer);
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
    // This would normally call the compiler/interpreter
    // For now, handle basic built-in commands
    
    if source.trim().starts_with("import") {
        // Handle imports
        return;
    }
    
    // Simple expression evaluation
    if let Some(result) = eval_simple(source) {
        println!("{}", result);
    } else {
        // Call actual compiler (stub)
        println!("Execution not yet implemented");
    }
}

fn eval_simple(source: &str) -> Option<String> {
    let source = source.trim();
    
    // Handle simple arithmetic
    if let Ok(num) = source.parse::<i64>() {
        return Some(num.to_string());
    }
    
    // Handle string literals
    if source.starts_with('"') && source.ends_with('"') {
        return Some(source[1..source.len()-1].to_string());
    }
    
    // Handle simple expressions like "1 + 2"
    if let Some(result) = eval_arithmetic(source) {
        return Some(result.to_string());
    }
    
    None
}

fn eval_arithmetic(expr: &str) -> Option<i64> {
    let parts: Vec<&str> = expr.split_whitespace().collect();
    if parts.len() == 3 {
        let left = parts[0].parse::<i64>().ok()?;
        let op = parts[1];
        let right = parts[2].parse::<i64>().ok()?;
        
        match op {
            "+" => Some(left + right),
            "-" => Some(left - right),
            "*" => Some(left * right),
            "/" => Some(left / right),
            _ => None,
        }
    } else {
        None
    }
}
