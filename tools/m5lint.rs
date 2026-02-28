/// m5lint - Enhanced linter for m5rcode v0.3.0
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
        "--version" | "-v" => println!("m5lint v0.3.0"),
        "--strict" => lint_files(&args[2..], true),
        _ => lint_files(&args[1..], false),
    }
}

fn print_help() {
    println!("m5lint - m5rcode Linter v0.3.0");
    println!("\nUsage:");
    println!("  m5lint <file.m5>          Lint file");
    println!("  m5lint <file1> <file2>... Lint multiple files");
    println!("  m5lint --strict <file>    Strict mode");
    println!("  m5lint --help             Show this help");
}

fn lint_files(files: &[String], strict: bool) {
    let mut total_issues = 0;
    
    for filename in files {
        let source = match fs::read_to_string(filename) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("❌ Error reading {}: {}", filename, e);
                continue;
            }
        };
        
        let issues = lint_code(&source, strict);
        
        if issues.is_empty() {
            println!("✅ {} - No issues", filename);
        } else {
            println!("\n📋 {} - {} issue(s):", filename, issues.len());
            for issue in &issues {
                println!("  {}", issue);
            }
            total_issues += issues.len();
        }
    }
    
    if total_issues > 0 {
        println!("\n❌ Total: {} issue(s) found", total_issues);
        std::process::exit(1);
    }
}

fn lint_code(source: &str, strict: bool) -> Vec<String> {
    let mut issues = Vec::new();
    
    for (line_num, line) in source.lines().enumerate() {
        let line_num = line_num + 1;
        let trimmed = line.trim();
        
        // Rule: Line too long
        if line.len() > 100 {
            issues.push(format!("⚠️  Line {}: Line too long ({} > 100 chars)", line_num, line.len()));
        }
        
        // Rule: Trailing whitespace
        if line.ends_with(' ') || line.ends_with('\t') {
            issues.push(format!("⚠️  Line {}: Trailing whitespace", line_num));
        }
        
        // Rule: Missing space after comma
        if line.contains(',') && line.contains(",(") {
            issues.push(format!("⚠️  Line {}: Missing space after comma", line_num));
        }
        
        // Rule: TODO/FIXME comments
        if trimmed.contains("TODO") || trimmed.contains("FIXME") {
            issues.push(format!("💡 Line {}: TODO/FIXME comment", line_num));
        }
        
        // Rule: Inconsistent quotes (strict mode)
        if strict {
            let single_quotes = line.matches('\'').count();
            let double_quotes = line.matches('"').count();
            if single_quotes > 0 && double_quotes > 0 {
                issues.push(format!("⚠️  Line {}: Mixed quote styles", line_num));
            }
        }
        
        // Rule: Missing semicolon (for certain statements)
        if strict && (trimmed.starts_with("let ") || trimmed.starts_with("const ")) 
           && !trimmed.ends_with(';') && !trimmed.ends_with('{') {
            // m5rcode doesn't require semicolons, but warn in strict mode
        }
        
        // Rule: Undefined function calls (basic check)
        if trimmed.contains("println(") && !trimmed.contains("io.println") {
            issues.push(format!("❌ Line {}: Use 'io.println' instead of 'println'", line_num));
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
