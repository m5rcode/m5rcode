/// m5lint - Linter for m5rcode
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: m5lint <file.m5>");
        std::process::exit(1);
    }
    
    let filename = &args[1];
    let source = fs::read_to_string(filename)
        .unwrap_or_else(|e| {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        });
    
    let issues = lint_code(&source);
    
    if issues.is_empty() {
        println!("No issues found in {}", filename);
    } else {
        println!("Found {} issue(s) in {}:", issues.len(), filename);
        for issue in issues {
            println!("  {}", issue);
        }
        std::process::exit(1);
    }
}

fn lint_code(source: &str) -> Vec<String> {
    let mut issues = Vec::new();
    
    for (line_num, line) in source.lines().enumerate() {
        let line_num = line_num + 1;
        
        // Rule: Line too long
        if line.len() > 100 {
            issues.push(format!("Line {}: Line too long ({} > 100 chars)", line_num, line.len()));
        }
        
        // Rule: Trailing whitespace
        if line.ends_with(' ') || line.ends_with('\t') {
            issues.push(format!("Line {}: Trailing whitespace", line_num));
        }
        
        // Rule: Unused variable (simple check)
        if line.trim().starts_with("let ") && !source.contains(&extract_var_name(line)) {
            issues.push(format!("Line {}: Unused variable", line_num));
        }
        
        // Rule: Missing space after comma
        if line.contains(",") && !line.contains(", ") {
            issues.push(format!("Line {}: Missing space after comma", line_num));
        }
        
        // Rule: TODO comments
        if line.contains("TODO") || line.contains("FIXME") {
            issues.push(format!("Line {}: TODO/FIXME comment", line_num));
        }
    }
    
    issues
}

fn extract_var_name(line: &str) -> String {
    line.trim()
        .strip_prefix("let ")
        .unwrap_or("")
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_string()
}
