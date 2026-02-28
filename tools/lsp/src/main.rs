/// m5rcode Language Server Protocol implementation
use std::io::{self, BufRead, Write, Read};
use std::collections::HashMap;

fn main() {
    eprintln!("m5rcode LSP server starting...");
    
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut documents: HashMap<String, String> = HashMap::new();
    
    loop {
        let mut header = String::new();
        if stdin.lock().read_line(&mut header).unwrap() == 0 {
            break;
        }
        
        if header.starts_with("Content-Length:") {
            let len: usize = header.split(':').nth(1)
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
            if let Some(response) = handle_message(&message, &mut documents) {
                send_response(&mut stdout, &response);
            }
        }
    }
}

fn send_response(stdout: &mut io::Stdout, response: &str) {
    let response_body = response.as_bytes();
    write!(stdout, "Content-Length: {}\r\n\r\n", response_body.len()).unwrap();
    stdout.write_all(response_body).unwrap();
    stdout.flush().unwrap();
}

fn handle_message(message: &str, documents: &mut HashMap<String, String>) -> Option<String> {
    eprintln!("Received: {}", message);
    
    // Initialize
    if message.contains("\"method\":\"initialize\"") {
        return Some(r#"{"jsonrpc":"2.0","id":1,"result":{"capabilities":{"textDocumentSync":1,"completionProvider":{"triggerCharacters":[".","("]},"hoverProvider":true,"definitionProvider":true,"documentSymbolProvider":true}}}"#.to_string());
    }
    
    // Initialized notification
    if message.contains("\"method\":\"initialized\"") {
        return None;
    }
    
    // Document opened
    if message.contains("\"method\":\"textDocument/didOpen\"") {
        if let Some(uri) = extract_field(message, "uri") {
            if let Some(text) = extract_field(message, "text") {
                documents.insert(uri.clone(), text.clone());
                let diagnostics = check_syntax(&text);
                return Some(format!(
                    r#"{{"jsonrpc":"2.0","method":"textDocument/publishDiagnostics","params":{{"uri":"{}","diagnostics":{}}}}}"#,
                    uri, diagnostics
                ));
            }
        }
        return None;
    }
    
    // Document changed
    if message.contains("\"method\":\"textDocument/didChange\"") {
        if let Some(uri) = extract_field(message, "uri") {
            if let Some(text) = extract_field(message, "text") {
                documents.insert(uri.clone(), text.clone());
                let diagnostics = check_syntax(&text);
                return Some(format!(
                    r#"{{"jsonrpc":"2.0","method":"textDocument/publishDiagnostics","params":{{"uri":"{}","diagnostics":{}}}}}"#,
                    uri, diagnostics
                ));
            }
        }
        return None;
    }
    
    // Completion
    if message.contains("\"method\":\"textDocument/completion\"") {
        let id = extract_id(message);
        let completions = get_completions();
        return Some(format!(
            r#"{{"jsonrpc":"2.0","id":{},"result":{{"items":{}}}}}"#,
            id, completions
        ));
    }
    
    // Hover
    if message.contains("\"method\":\"textDocument/hover\"") {
        let id = extract_id(message);
        return Some(format!(
            r#"{{"jsonrpc":"2.0","id":{},"result":{{"contents":{{"kind":"markdown","value":"m5rcode language"}}}}}}"#,
            id
        ));
    }
    
    // Go to definition
    if message.contains("\"method\":\"textDocument/definition\"") {
        let id = extract_id(message);
        if let Some(uri) = extract_field(message, "uri") {
            return Some(format!(
                r#"{{"jsonrpc":"2.0","id":{},"result":{{"uri":"{}","range":{{"start":{{"line":0,"character":0}},"end":{{"line":0,"character":5}}}}}}}}"#,
                id, uri
            ));
        }
    }
    
    // Document symbols
    if message.contains("\"method\":\"textDocument/documentSymbol\"") {
        let id = extract_id(message);
        if let Some(uri) = extract_field(message, "uri") {
            if let Some(text) = documents.get(&uri) {
                let symbols = extract_symbols(text);
                return Some(format!(
                    r#"{{"jsonrpc":"2.0","id":{},"result":{}}}"#,
                    id, symbols
                ));
            }
        }
    }
    
    Some(r#"{"jsonrpc":"2.0","id":0,"result":null}"#.to_string())
}

fn check_syntax(text: &str) -> String {
    let mut diagnostics = Vec::new();
    
    for (line_num, line) in text.lines().enumerate() {
        // Check for common syntax errors
        if line.contains("fn ") && !line.contains("(") {
            diagnostics.push(format!(
                r#"{{"range":{{"start":{{"line":{},"character":0}},"end":{{"line":{},"character":{}}}}},"severity":1,"message":"Function declaration missing parentheses"}}"#,
                line_num, line_num, line.len()
            ));
        }
        
        // Check for unmatched braces
        let open_braces = line.matches('{').count();
        let close_braces = line.matches('}').count();
        if open_braces != close_braces {
            diagnostics.push(format!(
                r#"{{"range":{{"start":{{"line":{},"character":0}},"end":{{"line":{},"character":{}}}}},"severity":2,"message":"Unmatched braces"}}"#,
                line_num, line_num, line.len()
            ));
        }
        
        // Check for missing semicolons (simple heuristic)
        if line.trim().starts_with("let ") && !line.trim().ends_with(';') && !line.trim().ends_with('{') {
            diagnostics.push(format!(
                r#"{{"range":{{"start":{{"line":{},"character":0}},"end":{{"line":{},"character":{}}}}},"severity":3,"message":"Consider adding semicolon"}}"#,
                line_num, line_num, line.len()
            ));
        }
    }
    
    format!("[{}]", diagnostics.join(","))
}

fn get_completions() -> String {
    r#"[
        {"label":"fn","kind":3,"detail":"Function declaration"},
        {"label":"class","kind":7,"detail":"Class declaration"},
        {"label":"if","kind":14,"detail":"If statement"},
        {"label":"else","kind":14,"detail":"Else statement"},
        {"label":"while","kind":14,"detail":"While loop"},
        {"label":"for","kind":14,"detail":"For loop"},
        {"label":"return","kind":14,"detail":"Return statement"},
        {"label":"import","kind":9,"detail":"Import module"},
        {"label":"let","kind":14,"detail":"Variable declaration"},
        {"label":"const","kind":14,"detail":"Constant declaration"},
        {"label":"io.println","kind":2,"detail":"Print with newline"},
        {"label":"io.print","kind":2,"detail":"Print without newline"},
        {"label":"io.readln","kind":2,"detail":"Read line from stdin"},
        {"label":"std.io","kind":9,"detail":"IO module"},
        {"label":"std.collections","kind":9,"detail":"Collections module"},
        {"label":"std.math","kind":9,"detail":"Math module"}
    ]"#.to_string()
}

fn extract_symbols(text: &str) -> String {
    let mut symbols = Vec::new();
    
    for (line_num, line) in text.lines().enumerate() {
        // Extract function definitions
        if line.contains("fn ") {
            if let Some(start) = line.find("fn ") {
                if let Some(end) = line[start..].find('(') {
                    let name = line[start+3..start+end].trim();
                    symbols.push(format!(
                        r#"{{"name":"{}","kind":12,"range":{{"start":{{"line":{},"character":{}}},"end":{{"line":{},"character":{}}}}},"selectionRange":{{"start":{{"line":{},"character":{}}},"end":{{"line":{},"character":{}}}}}}}"#,
                        name, line_num, start, line_num, start+end,
                        line_num, start+3, line_num, start+end
                    ));
                }
            }
        }
        
        // Extract class definitions
        if line.contains("class ") {
            if let Some(start) = line.find("class ") {
                if let Some(end) = line[start..].find(|c: char| c.is_whitespace() || c == '{') {
                    let name = line[start+6..start+end].trim();
                    symbols.push(format!(
                        r#"{{"name":"{}","kind":5,"range":{{"start":{{"line":{},"character":{}}},"end":{{"line":{},"character":{}}}}},"selectionRange":{{"start":{{"line":{},"character":{}}},"end":{{"line":{},"character":{}}}}}}}"#,
                        name, line_num, start, line_num, start+end,
                        line_num, start+6, line_num, start+end
                    ));
                }
            }
        }
    }
    
    format!("[{}]", symbols.join(","))
}

fn extract_field(message: &str, field: &str) -> Option<String> {
    let pattern = format!("\"{}\":\"", field);
    if let Some(start) = message.find(&pattern) {
        let start = start + pattern.len();
        if let Some(end) = message[start..].find('"') {
            return Some(message[start..start+end].to_string());
        }
    }
    None
}

fn extract_id(message: &str) -> i32 {
    if let Some(start) = message.find("\"id\":") {
        let start = start + 5;
        if let Some(end) = message[start..].find(|c: char| !c.is_ascii_digit()) {
            if let Ok(id) = message[start..start+end].parse() {
                return id;
            }
        }
    }
    0
}
