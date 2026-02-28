/// m5rcode Language Server Protocol implementation - v0.3.0
/// Enhanced with semantic analysis, better diagnostics, and improved completions
use std::io::{self, BufRead, Write, Read};
use std::collections::HashMap;

fn main() {
    eprintln!("m5rcode LSP server v0.3.0 starting...");
    
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut documents: HashMap<String, String> = HashMap::new();
    let mut symbols_cache: HashMap<String, Vec<Symbol>> = HashMap::new();
    
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
            if let Some(response) = handle_message(&message, &mut documents, &mut symbols_cache) {
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

#[derive(Debug, Clone)]
struct Symbol {
    name: String,
    kind: SymbolKind,
    line: usize,
    column: usize,
}

#[derive(Debug, Clone, PartialEq)]
enum SymbolKind {
    Function,
    Variable,
    Class,
    Method,
    Parameter,
}

fn handle_message(
    message: &str,
    documents: &mut HashMap<String, String>,
    symbols_cache: &mut HashMap<String, Vec<Symbol>>
) -> Option<String> {
    eprintln!("Received: {}", &message[..message.len().min(200)]);
    
    // Initialize
    if message.contains("\"method\":\"initialize\"") {
        return Some(r#"{"jsonrpc":"2.0","id":1,"result":{"capabilities":{"textDocumentSync":1,"completionProvider":{"triggerCharacters":[".","(",":",","]},"hoverProvider":true,"definitionProvider":true,"documentSymbolProvider":true,"diagnosticProvider":true,"semanticTokensProvider":{"legend":{"tokenTypes":["function","variable","class","keyword","operator","string","number","comment"],"tokenModifiers":["declaration","definition","readonly"]},"full":true}}}}"#.to_string());
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
                let symbols = extract_symbols(&text);
                symbols_cache.insert(uri.clone(), symbols);
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
                let symbols = extract_symbols(&text);
                symbols_cache.insert(uri.clone(), symbols);
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
        if let Some(uri) = extract_field(message, "uri") {
            let completions = if let Some(symbols) = symbols_cache.get(&uri) {
                get_completions_with_context(symbols, documents.get(&uri))
            } else {
                get_completions()
            };
            return Some(format!(
                r#"{{"jsonrpc":"2.0","id":{},"result":{{"items":{}}}}}"#,
                id, completions
            ));
        }
    }
    
    // Hover
    if message.contains("\"method\":\"textDocument/hover\"") {
        let id = extract_id(message);
        if let Some(uri) = extract_field(message, "uri") {
            if let Some(line) = extract_number(message, "line") {
                if let Some(symbols) = symbols_cache.get(&uri) {
                    if let Some(hover_text) = get_hover_info(symbols, line) {
                        return Some(format!(
                            r#"{{"jsonrpc":"2.0","id":{},"result":{{"contents":{{"kind":"markdown","value":"{}"}}}}}}"#,
                            id, hover_text
                        ));
                    }
                }
            }
        }
        return Some(format!(
            r#"{{"jsonrpc":"2.0","id":{},"result":{{"contents":{{"kind":"markdown","value":"m5rcode language"}}}}}}"#,
            extract_id(message)
        ));
    }
    
    // Go to definition
    if message.contains("\"method\":\"textDocument/definition\"") {
        let id = extract_id(message);
        if let Some(uri) = extract_field(message, "uri") {
            if let Some(line) = extract_number(message, "line") {
                if let Some(symbols) = symbols_cache.get(&uri) {
                    if let Some(symbol) = find_symbol_at_line(symbols, line) {
                        return Some(format!(
                            r#"{{"jsonrpc":"2.0","id":{},"result":{{"uri":"{}","range":{{"start":{{"line":{},"character":{}}},"end":{{"line":{},"character":{}}}}}}}}}"#,
                            id, uri, symbol.line, symbol.column, symbol.line, symbol.column + symbol.name.len()
                        ));
                    }
                }
            }
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
            if let Some(symbols) = symbols_cache.get(&uri) {
                let symbols_json = format_symbols(symbols);
                return Some(format!(
                    r#"{{"jsonrpc":"2.0","id":{},"result":{}}}"#,
                    id, symbols_json
                ));
            }
        }
    }
    
    Some(r#"{"jsonrpc":"2.0","id":0,"result":null}"#.to_string())
}

fn check_syntax(text: &str) -> String {
    let mut diagnostics = Vec::new();
    let mut brace_count: i32 = 0;
    let mut paren_count: i32 = 0;
    let mut bracket_count: i32 = 0;
    
    for (line_num, line) in text.lines().enumerate() {
        let trimmed = line.trim();
        
        // Track braces, parens, brackets
        for ch in line.chars() {
            match ch {
                '{' => brace_count += 1,
                '}' => brace_count -= 1,
                '(' => paren_count += 1,
                ')' => paren_count -= 1,
                '[' => bracket_count += 1,
                ']' => bracket_count -= 1,
                _ => {}
            }
        }
        
        // Check for function declaration without parentheses
        if trimmed.starts_with("fn ") && !line.contains("(") {
            diagnostics.push(format!(
                r#"{{"range":{{"start":{{"line":{},"character":0}},"end":{{"line":{},"character":{}}}}},"severity":1,"message":"Function declaration missing parentheses"}}"#,
                line_num, line_num, line.len()
            ));
        }
        
        // Check for let without assignment
        if trimmed.starts_with("let ") && !line.contains("=") && !line.ends_with(":") {
            diagnostics.push(format!(
                r#"{{"range":{{"start":{{"line":{},"character":0}},"end":{{"line":{},"character":{}}}}},"severity":2,"message":"Variable declaration without initialization"}}"#,
                line_num, line_num, line.len()
            ));
        }
        
        // Check for common typos
        if trimmed.contains("fucntion") || trimmed.contains("funciton") {
            diagnostics.push(format!(
                r#"{{"range":{{"start":{{"line":{},"character":0}},"end":{{"line":{},"character":{}}}}},"severity":2,"message":"Did you mean 'function'?"}}"#,
                line_num, line_num, line.len()
            ));
        }
        
        // Check for undefined variables (basic heuristic)
        if trimmed.contains("println") && !trimmed.contains("io.println") {
            diagnostics.push(format!(
                r#"{{"range":{{"start":{{"line":{},"character":0}},"end":{{"line":{},"character":{}}}}},"severity":3,"message":"Use 'io.println' instead of 'println'"}}"#,
                line_num, line_num, line.len()
            ));
        }
    }
    
    // Check for unmatched delimiters
    if brace_count != 0 {
        diagnostics.push(format!(
            r#"{{"range":{{"start":{{"line":0,"character":0}},"end":{{"line":0,"character":1}}}},"severity":1,"message":"Unmatched braces: {} unclosed"}}"#,
            brace_count.abs()
        ));
    }
    if paren_count != 0 {
        diagnostics.push(format!(
            r#"{{"range":{{"start":{{"line":0,"character":0}},"end":{{"line":0,"character":1}}}},"severity":1,"message":"Unmatched parentheses: {} unclosed"}}"#,
            paren_count.abs()
        ));
    }
    if bracket_count != 0 {
        diagnostics.push(format!(
            r#"{{"range":{{"start":{{"line":0,"character":0}},"end":{{"line":0,"character":1}}}},"severity":1,"message":"Unmatched brackets: {} unclosed"}}"#,
            bracket_count.abs()
        ));
    }
    
    format!("[{}]", diagnostics.join(","))
}

fn extract_symbols(text: &str) -> Vec<Symbol> {
    let mut symbols = Vec::new();
    
    for (line_num, line) in text.lines().enumerate() {
        let trimmed = line.trim();
        
        // Extract function definitions
        if trimmed.starts_with("fn ") {
            if let Some(name_start) = trimmed.find("fn ") {
                let rest = &trimmed[name_start + 3..];
                if let Some(paren_pos) = rest.find('(') {
                    let name = rest[..paren_pos].trim().to_string();
                    symbols.push(Symbol {
                        name,
                        kind: SymbolKind::Function,
                        line: line_num,
                        column: name_start + 3,
                    });
                }
            }
        }
        
        // Extract class definitions
        if trimmed.starts_with("class ") {
            if let Some(name_start) = trimmed.find("class ") {
                let rest = &trimmed[name_start + 6..];
                let name = rest.split_whitespace().next().unwrap_or("").trim_matches('{').to_string();
                if !name.is_empty() {
                    symbols.push(Symbol {
                        name,
                        kind: SymbolKind::Class,
                        line: line_num,
                        column: name_start + 6,
                    });
                }
            }
        }
        
        // Extract variable declarations
        if trimmed.starts_with("let ") || trimmed.starts_with("const ") {
            let keyword_len = if trimmed.starts_with("let ") { 4 } else { 6 };
            let rest = &trimmed[keyword_len..];
            if let Some(eq_pos) = rest.find('=') {
                let name = rest[..eq_pos].trim().to_string();
                symbols.push(Symbol {
                    name,
                    kind: SymbolKind::Variable,
                    line: line_num,
                    column: keyword_len,
                });
            }
        }
    }
    
    symbols
}

fn format_symbols(symbols: &[Symbol]) -> String {
    let symbol_items: Vec<String> = symbols.iter().map(|sym| {
        let kind = match sym.kind {
            SymbolKind::Function => 12,
            SymbolKind::Variable => 13,
            SymbolKind::Class => 5,
            SymbolKind::Method => 6,
            SymbolKind::Parameter => 17,
        };
        format!(
            r#"{{"name":"{}","kind":{},"range":{{"start":{{"line":{},"character":{}}},"end":{{"line":{},"character":{}}}}},"selectionRange":{{"start":{{"line":{},"character":{}}},"end":{{"line":{},"character":{}}}}}}}"#,
            sym.name, kind, sym.line, sym.column, sym.line, sym.column + sym.name.len(),
            sym.line, sym.column, sym.line, sym.column + sym.name.len()
        )
    }).collect();
    
    format!("[{}]", symbol_items.join(","))
}

fn get_completions() -> String {
    let completions = vec![
        r#"{"label":"fn","kind":3,"detail":"Function declaration","insertText":"fn ${1:name}($2) {\n\t$0\n}","insertTextFormat":2}"#,
        r#"{"label":"let","kind":14,"detail":"Variable declaration","insertText":"let ${1:name} = $0","insertTextFormat":2}"#,
        r#"{"label":"class","kind":7,"detail":"Class declaration","insertText":"class ${1:Name} {\n\t$0\n}","insertTextFormat":2}"#,
        r#"{"label":"if","kind":14,"detail":"If statement","insertText":"if $1 {\n\t$0\n}","insertTextFormat":2}"#,
        r#"{"label":"while","kind":14,"detail":"While loop","insertText":"while $1 {\n\t$0\n}","insertTextFormat":2}"#,
        r#"{"label":"for","kind":14,"detail":"For loop","insertText":"for ${1:item} in ${2:iterable} {\n\t$0\n}","insertTextFormat":2}"#,
        r#"{"label":"return","kind":14,"detail":"Return statement"}"#,
        r#"{"label":"import","kind":9,"detail":"Import module","insertText":"import ${1:std.io}","insertTextFormat":2}"#,
        r#"{"label":"io.println","kind":2,"detail":"Print line to stdout"}"#,
        r#"{"label":"io.print","kind":2,"detail":"Print to stdout"}"#,
        r#"{"label":"io.input","kind":2,"detail":"Read input from stdin"}"#,
        r#"{"label":"math.abs","kind":2,"detail":"Absolute value"}"#,
        r#"{"label":"math.sqrt","kind":2,"detail":"Square root"}"#,
        r#"{"label":"math.pow","kind":2,"detail":"Power function"}"#,
        r#"{"label":"math.floor","kind":2,"detail":"Floor function"}"#,
        r#"{"label":"math.ceil","kind":2,"detail":"Ceiling function"}"#,
        r#"{"label":"math.round","kind":2,"detail":"Round function"}"#,
        r#"{"label":"string.len","kind":2,"detail":"String length"}"#,
        r#"{"label":"string.upper","kind":2,"detail":"Convert to uppercase"}"#,
        r#"{"label":"string.lower","kind":2,"detail":"Convert to lowercase"}"#,
        r#"{"label":"string.trim","kind":2,"detail":"Trim whitespace"}"#,
        r#"{"label":"string.split","kind":2,"detail":"Split string"}"#,
        r#"{"label":"list.len","kind":2,"detail":"List length"}"#,
        r#"{"label":"type","kind":2,"detail":"Get type of value"}"#,
        r#"{"label":"str","kind":2,"detail":"Convert to string"}"#,
        r#"{"label":"int","kind":2,"detail":"Convert to integer"}"#,
        r#"{"label":"float","kind":2,"detail":"Convert to float"}"#,
        r#"{"label":"bool","kind":2,"detail":"Convert to boolean"}"#,
    ];
    
    format!("[{}]", completions.join(","))
}

fn get_completions_with_context(symbols: &[Symbol], _text: Option<&String>) -> String {
    let mut completions = vec![
        r#"{"label":"fn","kind":3,"detail":"Function declaration","insertText":"fn ${1:name}($2) {\n\t$0\n}","insertTextFormat":2}"#.to_string(),
        r#"{"label":"let","kind":14,"detail":"Variable declaration","insertText":"let ${1:name} = $0","insertTextFormat":2}"#.to_string(),
        r#"{"label":"class","kind":7,"detail":"Class declaration","insertText":"class ${1:Name} {\n\t$0\n}","insertTextFormat":2}"#.to_string(),
        r#"{"label":"if","kind":14,"detail":"If statement","insertText":"if $1 {\n\t$0\n}","insertTextFormat":2}"#.to_string(),
        r#"{"label":"while","kind":14,"detail":"While loop","insertText":"while $1 {\n\t$0\n}","insertTextFormat":2}"#.to_string(),
        r#"{"label":"for","kind":14,"detail":"For loop","insertText":"for ${1:item} in ${2:iterable} {\n\t$0\n}","insertTextFormat":2}"#.to_string(),
        r#"{"label":"return","kind":14,"detail":"Return statement"}"#.to_string(),
        r#"{"label":"import","kind":9,"detail":"Import module","insertText":"import ${1:std.io}","insertTextFormat":2}"#.to_string(),
        r#"{"label":"io.println","kind":2,"detail":"Print line to stdout"}"#.to_string(),
        r#"{"label":"io.print","kind":2,"detail":"Print to stdout"}"#.to_string(),
        r#"{"label":"io.input","kind":2,"detail":"Read input from stdin"}"#.to_string(),
        r#"{"label":"math.abs","kind":2,"detail":"Absolute value"}"#.to_string(),
        r#"{"label":"math.sqrt","kind":2,"detail":"Square root"}"#.to_string(),
        r#"{"label":"math.pow","kind":2,"detail":"Power function"}"#.to_string(),
        r#"{"label":"string.len","kind":2,"detail":"String length"}"#.to_string(),
        r#"{"label":"string.upper","kind":2,"detail":"Convert to uppercase"}"#.to_string(),
        r#"{"label":"string.lower","kind":2,"detail":"Convert to lowercase"}"#.to_string(),
        r#"{"label":"list.len","kind":2,"detail":"List length"}"#.to_string(),
        r#"{"label":"type","kind":2,"detail":"Get type of value"}"#.to_string(),
        r#"{"label":"str","kind":2,"detail":"Convert to string"}"#.to_string(),
        r#"{"label":"int","kind":2,"detail":"Convert to integer"}"#.to_string(),
        r#"{"label":"float","kind":2,"detail":"Convert to float"}"#.to_string(),
    ];
    
    // Add symbols from current file
    for symbol in symbols {
        let kind = match symbol.kind {
            SymbolKind::Function => 3,
            SymbolKind::Variable => 6,
            SymbolKind::Class => 7,
            _ => 6,
        };
        completions.push(format!(
            r#"{{"label":"{}","kind":{},"detail":"Defined in file"}}"#,
            symbol.name, kind
        ));
    }
    
    format!("[{}]", completions.join(","))
}

fn get_hover_info(symbols: &[Symbol], line: usize) -> Option<String> {
    for symbol in symbols {
        if symbol.line == line {
            let kind_str = match symbol.kind {
                SymbolKind::Function => "function",
                SymbolKind::Variable => "variable",
                SymbolKind::Class => "class",
                SymbolKind::Method => "method",
                SymbolKind::Parameter => "parameter",
            };
            return Some(format!("**{}** `{}`", kind_str, symbol.name));
        }
    }
    None
}

fn find_symbol_at_line(symbols: &[Symbol], line: usize) -> Option<&Symbol> {
    symbols.iter().find(|s| s.line == line)
}

fn extract_id(message: &str) -> i32 {
    if let Some(start) = message.find("\"id\":") {
        let rest = &message[start + 5..];
        if let Some(end) = rest.find(|c: char| !c.is_numeric()) {
            return rest[..end].parse().unwrap_or(0);
        }
    }
    0
}

fn extract_number(message: &str, field: &str) -> Option<usize> {
    let search = format!("\"{}\":", field);
    if let Some(start) = message.find(&search) {
        let rest = &message[start + search.len()..];
        if let Some(end) = rest.find(|c: char| !c.is_numeric()) {
            return rest[..end].parse().ok();
        }
    }
    None
}

fn extract_field(message: &str, field: &str) -> Option<String> {
    let search = format!("\"{}\":\"", field);
    if let Some(start) = message.find(&search) {
        let rest = &message[start + search.len()..];
        if let Some(end) = rest.find('"') {
            return Some(rest[..end].to_string());
        }
    }
    
    // Try without quotes for text field
    if field == "text" {
        let search = format!("\"{}\":\"", field);
        if let Some(start) = message.find(&search) {
            let rest = &message[start + search.len()..];
            let depth = 0;
            let mut escaped = false;
            for (i, ch) in rest.chars().enumerate() {
                if escaped {
                    escaped = false;
                    continue;
                }
                match ch {
                    '\\' => escaped = true,
                    '"' if depth == 0 => return Some(rest[..i].replace("\\n", "\n").replace("\\t", "\t").replace("\\\"", "\"")),
                    _ => {}
                }
            }
        }
    }
    
    None
}
