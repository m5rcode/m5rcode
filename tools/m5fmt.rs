/// m5fmt - Code formatter for m5rcode
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: m5fmt <file.m5>");
        std::process::exit(1);
    }
    
    let filename = &args[1];
    let source = fs::read_to_string(filename)
        .unwrap_or_else(|e| {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        });
    
    let formatted = format_code(&source);
    
    // Write back to file
    fs::write(filename, formatted)
        .unwrap_or_else(|e| {
            eprintln!("Error writing file: {}", e);
            std::process::exit(1);
        });
    
    println!("Formatted {}", filename);
}

fn format_code(source: &str) -> String {
    let mut result = String::new();
    let mut indent_level: i32 = 0;
    
    for line in source.lines() {
        let trimmed = line.trim();
        
        // Decrease indent for closing braces
        if trimmed.starts_with('}') {
            indent_level = indent_level.saturating_sub(1);
        }
        
        // Add indentation
        if !trimmed.is_empty() {
            result.push_str(&"    ".repeat(indent_level as usize));
            result.push_str(trimmed);
            result.push('\n');
        } else {
            result.push('\n');
        }
        
        // Increase indent for opening braces
        if trimmed.ends_with('{') {
            indent_level += 1;
        }
    }
    
    result
}
