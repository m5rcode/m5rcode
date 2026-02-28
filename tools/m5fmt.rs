/// m5fmt - Enhanced code formatter for m5rcode v0.3.0
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_help();
        std::process::exit(1);
    }
    
    match args[1].as_str() {
        "--help" | "-h" => print_help(),
        "--version" | "-v" => println!("m5fmt v0.3.0"),
        "--check" | "-c" => check_format(&args[2..]),
        _ => format_files(&args[1..]),
    }
}

fn print_help() {
    println!("m5fmt - m5rcode Code Formatter v0.3.0");
    println!("\nUsage:");
    println!("  m5fmt <file.m5>           Format file in-place");
    println!("  m5fmt <file1> <file2>...  Format multiple files");
    println!("  m5fmt --check <file>      Check if formatted");
    println!("  m5fmt --help              Show this help");
}

fn check_format(files: &[String]) {
    for filename in files {
        let source = match fs::read_to_string(filename) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("❌ Error reading {}: {}", filename, e);
                continue;
            }
        };
        
        let formatted = format_code(&source);
        if source == formatted {
            println!("✅ {} is formatted", filename);
        } else {
            println!("❌ {} needs formatting", filename);
        }
    }
}

fn format_files(files: &[String]) {
    for filename in files {
        let source = match fs::read_to_string(filename) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("❌ Error reading {}: {}", filename, e);
                continue;
            }
        };
        
        let formatted = format_code(&source);
        
        if source == formatted {
            println!("✅ {} already formatted", filename);
            continue;
        }
        
        match fs::write(filename, formatted) {
            Ok(_) => println!("✅ Formatted {}", filename),
            Err(e) => eprintln!("❌ Error writing {}: {}", filename, e),
        }
    }
}

fn format_code(source: &str) -> String {
    let mut result = String::new();
    let mut indent_level: i32 = 0;
    let mut in_string = false;
    
    for line in source.lines() {
        let trimmed = line.trim();
        
        if trimmed.is_empty() {
            result.push('\n');
            continue;
        }
        
        // Handle closing braces
        if trimmed.starts_with('}') && !in_string {
            indent_level = indent_level.saturating_sub(1);
        }
        
        // Add indentation
        result.push_str(&"    ".repeat(indent_level as usize));
        result.push_str(trimmed);
        result.push('\n');
        
        // Handle opening braces
        if trimmed.ends_with('{') && !in_string {
            indent_level += 1;
        }
        
        // Track strings (simple)
        in_string = trimmed.matches('"').count() % 2 != 0;
    }
    
    result
}
