/// m5rcode Language Server Protocol implementation
use std::io::{self, BufRead, Write, Read};

fn main() {
    eprintln!("m5rcode LSP server starting...");
    
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        
        if line.starts_with("Content-Length:") {
            // Parse LSP message
            let len: usize = line.split(':').nth(1)
                .unwrap()
                .trim()
                .parse()
                .unwrap();
            
            // Read empty line
            let mut _empty = String::new();
            stdin.lock().read_line(&mut _empty).unwrap();
            
            // Read message body
            let mut buffer = vec![0u8; len];
            io::stdin().read_exact(&mut buffer).unwrap();
            
            let message = String::from_utf8(buffer).unwrap();
            let response = handle_message(&message);
            
            // Send response
            let response_body = response.as_bytes();
            write!(stdout, "Content-Length: {}\r\n\r\n", response_body.len()).unwrap();
            stdout.write_all(response_body).unwrap();
            stdout.flush().unwrap();
        }
    }
}

fn handle_message(message: &str) -> String {
    eprintln!("Received: {}", message);
    
    // Parse JSON (simplified)
    if message.contains("initialize") {
        return r#"{"jsonrpc":"2.0","id":1,"result":{"capabilities":{"textDocumentSync":1,"completionProvider":{"triggerCharacters":["."]}}}}#.to_string();
    }
    
    if message.contains("textDocument/didOpen") {
        // Syntax check
        return r#"{"jsonrpc":"2.0","method":"textDocument/publishDiagnostics","params":{"uri":"file:///test.m5","diagnostics":[]}}"#.to_string();
    }
    
    if message.contains("textDocument/definition") {
        // Go to definition
        return r#"{"jsonrpc":"2.0","id":2,"result":{"uri":"file:///test.m5","range":{"start":{"line":0,"character":0},"end":{"line":0,"character":5}}}}"#.to_string();
    }
    
    if message.contains("textDocument/completion") {
        // Completions
        return r#"{"jsonrpc":"2.0","id":3,"result":{"items":[{"label":"println","kind":3}]}}"#.to_string();
    }
    
    r#"{"jsonrpc":"2.0","id":0,"result":null}"#.to_string()
}
