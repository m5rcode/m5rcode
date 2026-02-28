/// m5idle - GTK-based IDE for m5rcode
use std::process::{Command, Stdio};
use std::io::Write;

fn main() {
    println!("m5idle - m5rcode IDE v0.1.0");
    println!("GTK GUI not yet implemented");
    println!("Launching embedded REPL...\n");
    
    // Launch m5repl as subprocess
    let mut child = Command::new("m5repl")
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to launch m5repl");
    
    // Simple text-based interface for now
    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        
        if input.trim() == "exit" {
            break;
        }
        
        stdin.write_all(input.as_bytes()).unwrap();
        stdin.flush().unwrap();
    }
    
    child.wait().unwrap();
}

// TODO: Implement GTK GUI with:
// - Text editor pane (syntax highlighting)
// - REPL output pane
// - File browser
// - Menu bar (File, Edit, Run, Help)
// - Toolbar (New, Open, Save, Run)
