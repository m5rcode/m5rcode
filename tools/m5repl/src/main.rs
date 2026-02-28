/// m5repl - Enhanced Interactive REPL for m5rcode v0.3.0
use std::io::{self, Write};
use std::fs;
use std::env;

extern crate m5rcode;
use m5rcode::{Lexer, Parser, Interpreter};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        match args[1].as_str() {
            "--help" | "-h" => print_help(),
            "--version" | "-v" => println!("m5repl v0.3.0"),
            _ => run_file(&args[1]),
        }
    } else {
        repl();
    }
}

fn print_help() {
    println!("m5repl - m5rcode Interactive REPL v0.3.0");
    println!("\nUsage:");
    println!("  m5repl              Start interactive REPL");
    println!("  m5repl <file.m5>    Execute a file");
    println!("  m5repl --help       Show this help");
    println!("  m5repl --version    Show version");
    println!("\nREPL Commands:");
    println!("  exit, quit, :q      Exit REPL");
    println!("  :clear, :c          Clear screen");
    println!("  :help, :h           Show help");
    println!("  :vars               Show all variables");
    println!("  :reset              Reset interpreter state");
}

fn run_file(filename: &str) {
    let source = fs::read_to_string(filename)
        .unwrap_or_else(|e| {
            eprintln!("❌ Error reading file '{}': {}", filename, e);
            std::process::exit(1);
        });
    
    execute(&source);
}

fn repl() {
    println!("╔═══════════════════════════════════════════════════════╗");
    println!("║          m5rcode Interactive REPL v0.3.0             ║");
    println!("╚═══════════════════════════════════════════════════════╝");
    println!("\n💡 Type ':help' for commands, 'exit' to quit\n");
    
    let mut buffer = String::new();
    let mut interpreter = Interpreter::new();
    let mut history: Vec<String> = Vec::new();
    
    loop {
        let prompt = if buffer.is_empty() { "m5> " } else { "... " };
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        
        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() {
            break;
        }
        
        let line = line.trim();
        
        // Handle REPL commands
        match line {
            "exit" | "quit" | ":q" => {
                println!("👋 Goodbye!");
                break;
            },
            ":clear" | ":c" => {
                print!("\x1B[2J\x1B[1;1H");
                continue;
            },
            ":help" | ":h" => {
                print_repl_help();
                continue;
            },
            ":vars" => {
                println!("📊 Variables: (feature coming soon)");
                continue;
            },
            ":reset" => {
                interpreter = Interpreter::new();
                buffer.clear();
                println!("🔄 Interpreter reset");
                continue;
            },
            ":history" => {
                println!("📜 History:");
                for (i, cmd) in history.iter().enumerate() {
                    println!("  {}: {}", i + 1, cmd);
                }
                continue;
            },
            "" => continue,
            _ => {}
        }
        
        buffer.push_str(line);
        buffer.push('\n');
        
        if is_complete(&buffer) {
            history.push(buffer.trim().to_string());
            execute_with_interpreter(&buffer, &mut interpreter);
            buffer.clear();
        }
    }
}

fn print_repl_help() {
    println!("\n📖 REPL Commands:");
    println!("  exit, quit, :q    Exit REPL");
    println!("  :clear, :c        Clear screen");
    println!("  :help, :h         Show this help");
    println!("  :vars             Show variables");
    println!("  :reset            Reset interpreter");
    println!("  :history          Show command history");
    println!("\n📚 Quick Examples:");
    println!("  let x = 42");
    println!("  io.println(\"Hello!\")");
    println!("  math.sqrt(16)");
    println!("  typeof(x)");
    println!();
}

fn is_complete(code: &str) -> bool {
    let open_braces = code.matches('{').count();
    let close_braces = code.matches('}').count();
    let open_parens = code.matches('(').count();
    let close_parens = code.matches(')').count();
    let open_brackets = code.matches('[').count();
    let close_brackets = code.matches(']').count();
    
    open_braces == close_braces && 
    open_parens == close_parens && 
    open_brackets == close_brackets
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
            eprintln!("❌ Lexer error: {}", e);
            return;
        }
    };
    
    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("❌ Parser error: {}", e);
            return;
        }
    };
    
    if let Err(e) = interpreter.execute(&ast) {
        eprintln!("❌ Runtime error: {}", e);
    }
}
