/// m5rcode Compiler - Production-Grade Implementation
/// Inspired by Terry A. Davis's HolyC compiler
/// 
/// Architecture:
/// 1. Lexer - Tokenization with full Unicode support
/// 2. Parser - Recursive descent with error recovery
/// 3. Semantic Analyzer - Type checking and symbol resolution
/// 4. IR Generator - Intermediate representation
/// 5. Optimizer - Multiple optimization passes
/// 6. Code Generator - LLVM IR or C backend
/// 7. Runtime - GC and standard library

use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

// ============================================================================
// LEXER - Comprehensive tokenization
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Int(i64),
    Float(f64),
    StringLit(String),
    Char(char),
    Bool(bool),
    Null,
    
    // Identifiers and keywords
    Ident(String),
    
    // Keywords - Control flow
    Fn, Class, Trait, Impl, If, Else, While, For, In, Loop, Break, Continue,
    Return, Match, Case, Async, Await,
    
    // Keywords - Declarations
    Let, Var, Const, Static, Pub, Priv, Mut, Own, Move, Ref,
    Import, Export, As, Use, Mod, Type, Enum, Struct, Union,
    
    // Keywords - Types
    IntType, FloatType, StringType, BoolType, VoidType, SelfType,
    
    // Operators - Arithmetic
    Plus, Minus, Star, Slash, Percent, StarStar, // ** for power
    
    // Operators - Comparison
    Eq, EqEq, NotEq, Lt, Gt, LtEq, GtEq, Spaceship, // <=> for three-way comparison
    
    // Operators - Logical
    And, Or, Not, AndAnd, OrOr,
    
    // Operators - Bitwise
    BitAnd, BitOr, BitXor, BitNot, Shl, Shr,
    
    // Operators - Assignment
    PlusEq, MinusEq, StarEq, SlashEq, PercentEq,
    AndEq, OrEq, XorEq, ShlEq, ShrEq,
    
    // Operators - Other
    Question, Colon, ColonColon, Arrow, FatArrow, Dot, DotDot, DotDotEq,
    At, Hash, Dollar, Pipe, Ampersand, Caret, Tilde,
    
    // Delimiters
    LParen, RParen, LBrace, RBrace, LBracket, RBracket,
    Comma, Semi, Backslash,
    
    // Special
    Newline, Indent, Dedent, Eof, Error(String),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub typ: TokenType,
    pub line: usize,
    pub column: usize,
    pub file: String,
}

impl Token {
    fn new(typ: TokenType, line: usize, column: usize, file: String) -> Self {
        Token { typ, line, column, file }
    }
}

pub struct Lexer {
    source: Vec<char>,
    pos: usize,
    line: usize,
    column: usize,
    file: String,
    indent_stack: Vec<usize>,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self::new_with_file(source, "<stdin>".to_string())
    }
    
    pub fn new_with_file(source: &str, file: String) -> Self {
        Lexer {
            source: source.chars().collect(),
            pos: 0,
            line: 1,
            column: 1,
            file,
            indent_stack: vec![0],
        }
    }
    
    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        
        while !self.is_eof() {
            // Handle indentation at start of line
            if self.column == 1 && !self.is_eof() {
                let indent_level = self.count_indent();
                let current_indent = *self.indent_stack.last().unwrap();
                
                if indent_level > current_indent {
                    self.indent_stack.push(indent_level);
                    tokens.push(self.make_token(TokenType::Indent));
                } else if indent_level < current_indent {
                    while let Some(&stack_indent) = self.indent_stack.last() {
                        if stack_indent <= indent_level {
                            break;
                        }
                        self.indent_stack.pop();
                        tokens.push(self.make_token(TokenType::Dedent));
                    }
                }
            }
            
            self.skip_whitespace_inline();
            if self.is_eof() { break; }
            
            // Comments
            if self.peek() == '#' {
                self.skip_line();
                continue;
            }
            
            if self.peek() == '/' && self.peek_ahead(1) == '*' {
                self.skip_block_comment()?;
                continue;
            }
            
            let token = self.next_token()?;
            tokens.push(token);
        }
        
        // Emit remaining dedents
        while self.indent_stack.len() > 1 {
            self.indent_stack.pop();
            tokens.push(self.make_token(TokenType::Dedent));
        }
        
        tokens.push(self.make_token(TokenType::Eof));
        Ok(tokens)
    }
    
    fn next_token(&mut self) -> Result<Token, String> {
        let ch = self.advance();
        
        let typ = match ch {
            '\n' => {
                self.line += 1;
                self.column = 1;
                TokenType::Newline
            },
            '+' => {
                if self.peek() == '=' {
                    self.advance();
                    TokenType::PlusEq
                } else {
                    TokenType::Plus
                }
            },
            '-' => {
                if self.peek() == '>' {
                    self.advance();
                    TokenType::Arrow
                } else if self.peek() == '=' {
                    self.advance();
                    TokenType::MinusEq
                } else {
                    TokenType::Minus
                }
            },
            '*' => {
                if self.peek() == '*' {
                    self.advance();
                    TokenType::StarStar
                } else if self.peek() == '=' {
                    self.advance();
                    TokenType::StarEq
                } else {
                    TokenType::Star
                }
            },
            '/' => {
                if self.peek() == '=' {
                    self.advance();
                    TokenType::SlashEq
                } else {
                    TokenType::Slash
                }
            },
            '%' => {
                if self.peek() == '=' {
                    self.advance();
                    TokenType::PercentEq
                } else {
                    TokenType::Percent
                }
            },
            '=' => {
                if self.peek() == '=' {
                    self.advance();
                    TokenType::EqEq
                } else if self.peek() == '>' {
                    self.advance();
                    TokenType::FatArrow
                } else {
                    TokenType::Eq
                }
            },
            '!' => {
                if self.peek() == '=' {
                    self.advance();
                    TokenType::NotEq
                } else {
                    TokenType::Not
                }
            },
            '<' => {
                if self.peek() == '=' {
                    self.advance();
                    if self.peek() == '>' {
                        self.advance();
                        TokenType::Spaceship
                    } else {
                        TokenType::LtEq
                    }
                } else if self.peek() == '<' {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        TokenType::ShlEq
                    } else {
                        TokenType::Shl
                    }
                } else {
                    TokenType::Lt
                }
            },
            '>' => {
                if self.peek() == '=' {
                    self.advance();
                    TokenType::GtEq
                } else if self.peek() == '>' {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        TokenType::ShrEq
                    } else {
                        TokenType::Shr
                    }
                } else {
                    TokenType::Gt
                }
            },
            '&' => {
                if self.peek() == '&' {
                    self.advance();
                    TokenType::AndAnd
                } else if self.peek() == '=' {
                    self.advance();
                    TokenType::AndEq
                } else {
                    TokenType::BitAnd
                }
            },
            '|' => {
                if self.peek() == '|' {
                    self.advance();
                    TokenType::OrOr
                } else if self.peek() == '=' {
                    self.advance();
                    TokenType::OrEq
                } else {
                    TokenType::BitOr
                }
            },
            '^' => {
                if self.peek() == '=' {
                    self.advance();
                    TokenType::XorEq
                } else {
                    TokenType::BitXor
                }
            },
            '~' => TokenType::BitNot,
            '(' => TokenType::LParen,
            ')' => TokenType::RParen,
            '{' => TokenType::LBrace,
            '}' => TokenType::RBrace,
            '[' => TokenType::LBracket,
            ']' => TokenType::RBracket,
            ',' => TokenType::Comma,
            ';' => TokenType::Semi,
            ':' => {
                if self.peek() == ':' {
                    self.advance();
                    TokenType::ColonColon
                } else {
                    TokenType::Colon
                }
            },
            '.' => {
                if self.peek() == '.' {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        TokenType::DotDotEq
                    } else {
                        TokenType::DotDot
                    }
                } else {
                    TokenType::Dot
                }
            },
            '?' => TokenType::Question,
            '@' => TokenType::At,
            '#' => TokenType::Hash,
            '$' => TokenType::Dollar,
            '\\' => TokenType::Backslash,
            '"' => return self.read_string(),
            '\'' => return self.read_char(),
            _ if ch.is_ascii_digit() => return self.read_number(ch),
            _ if ch.is_alphabetic() || ch == '_' => return self.read_ident(ch),
            _ => return Err(format!("Unexpected character: '{}' at {}:{}", ch, self.line, self.column)),
        };
        
        Ok(self.make_token(typ))
    }
    
    fn read_string(&mut self) -> Result<Token, std::string::String> {
        let mut s = std::string::String::new();
        let mut escaped = false;
        
        while !self.is_eof() {
            let ch = self.peek();
            
            if escaped {
                self.advance();
                let escape_char = match ch {
                    'n' => '\n',
                    't' => '\t',
                    'r' => '\r',
                    '\\' => '\\',
                    '"' => '"',
                    '0' => '\0',
                    _ => return Err(format!("Invalid escape sequence: \\{}", ch)),
                };
                s.push(escape_char);
                escaped = false;
            } else if ch == '\\' {
                self.advance();
                escaped = true;
            } else if ch == '"' {
                self.advance();
                break;
            } else {
                s.push(self.advance());
            }
        }
        
        Ok(self.make_token(TokenType::StringLit(s)))
    }
    
    fn read_char(&mut self) -> Result<Token, String> {
        if self.is_eof() {
            return Err("Unterminated character literal".to_string());
        }
        
        let ch = if self.peek() == '\\' {
            self.advance();
            match self.advance() {
                'n' => '\n',
                't' => '\t',
                'r' => '\r',
                '\\' => '\\',
                '\'' => '\'',
                '0' => '\0',
                c => return Err(format!("Invalid escape sequence: \\{}", c)),
            }
        } else {
            self.advance()
        };
        
        if self.peek() != '\'' {
            return Err("Unterminated character literal".to_string());
        }
        self.advance();
        
        Ok(self.make_token(TokenType::Char(ch)))
    }
    
    fn read_number(&mut self, first: char) -> Result<Token, String> {
        let mut num = String::from(first);
        let mut is_float = false;
        
        // Handle hex, binary, octal
        if first == '0' && !self.is_eof() {
            match self.peek() {
                'x' | 'X' => {
                    num.push(self.advance());
                    while !self.is_eof() && self.peek().is_ascii_hexdigit() {
                        num.push(self.advance());
                    }
                    let value = i64::from_str_radix(&num[2..], 16)
                        .map_err(|e| format!("Invalid hex number: {}", e))?;
                    return Ok(self.make_token(TokenType::Int(value)));
                },
                'b' | 'B' => {
                    num.push(self.advance());
                    while !self.is_eof() && matches!(self.peek(), '0' | '1') {
                        num.push(self.advance());
                    }
                    let value = i64::from_str_radix(&num[2..], 2)
                        .map_err(|e| format!("Invalid binary number: {}", e))?;
                    return Ok(self.make_token(TokenType::Int(value)));
                },
                'o' | 'O' => {
                    num.push(self.advance());
                    while !self.is_eof() && self.peek().is_digit(8) {
                        num.push(self.advance());
                    }
                    let value = i64::from_str_radix(&num[2..], 8)
                        .map_err(|e| format!("Invalid octal number: {}", e))?;
                    return Ok(self.make_token(TokenType::Int(value)));
                },
                _ => {}
            }
        }
        
        // Regular decimal number
        while !self.is_eof() && self.peek().is_ascii_digit() {
            num.push(self.advance());
        }
        
        // Check for decimal point
        if !self.is_eof() && self.peek() == '.' && self.peek_ahead(1).is_ascii_digit() {
            is_float = true;
            num.push(self.advance());
            while !self.is_eof() && self.peek().is_ascii_digit() {
                num.push(self.advance());
            }
        }
        
        // Check for scientific notation
        if !self.is_eof() && matches!(self.peek(), 'e' | 'E') {
            is_float = true;
            num.push(self.advance());
            if !self.is_eof() && matches!(self.peek(), '+' | '-') {
                num.push(self.advance());
            }
            while !self.is_eof() && self.peek().is_ascii_digit() {
                num.push(self.advance());
            }
        }
        
        if is_float {
            let value = num.parse::<f64>()
                .map_err(|e| format!("Invalid float: {}", e))?;
            Ok(self.make_token(TokenType::Float(value)))
        } else {
            let value = num.parse::<i64>()
                .map_err(|e| format!("Invalid integer: {}", e))?;
            Ok(self.make_token(TokenType::Int(value)))
        }
    }
    
    fn read_ident(&mut self, first: char) -> Result<Token, String> {
        let mut ident = String::from(first);
        while !self.is_eof() && (self.peek().is_alphanumeric() || self.peek() == '_') {
            ident.push(self.advance());
        }
        
        let typ = match ident.as_str() {
            // Control flow
            "fn" => TokenType::Fn,
            "class" => TokenType::Class,
            "trait" => TokenType::Trait,
            "impl" => TokenType::Impl,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "while" => TokenType::While,
            "for" => TokenType::For,
            "in" => TokenType::In,
            "loop" => TokenType::Loop,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            "return" => TokenType::Return,
            "match" => TokenType::Match,
            "case" => TokenType::Case,
            "async" => TokenType::Async,
            "await" => TokenType::Await,
            
            // Declarations
            "let" => TokenType::Let,
            "var" => TokenType::Var,
            "const" => TokenType::Const,
            "static" => TokenType::Static,
            "pub" => TokenType::Pub,
            "priv" => TokenType::Priv,
            "mut" => TokenType::Mut,
            "own" => TokenType::Own,
            "move" => TokenType::Move,
            "ref" => TokenType::Ref,
            "import" => TokenType::Import,
            "export" => TokenType::Export,
            "as" => TokenType::As,
            "use" => TokenType::Use,
            "mod" => TokenType::Mod,
            "type" => TokenType::Type,
            "enum" => TokenType::Enum,
            "struct" => TokenType::Struct,
            "union" => TokenType::Union,
            
            // Types
            "int" => TokenType::IntType,
            "float" => TokenType::FloatType,
            "string" => TokenType::StringType,
            "bool" => TokenType::BoolType,
            "void" => TokenType::VoidType,
            "self" | "Self" => TokenType::SelfType,
            
            // Literals
            "true" => TokenType::Bool(true),
            "false" => TokenType::Bool(false),
            "null" => TokenType::Null,
            
            _ => TokenType::Ident(ident),
        };
        
        Ok(self.make_token(typ))
    }
    
    fn count_indent(&mut self) -> usize {
        let mut count = 0;
        while !self.is_eof() && matches!(self.peek(), ' ' | '\t') {
            if self.peek() == '\t' {
                count += 4; // Tab = 4 spaces
            } else {
                count += 1;
            }
            self.advance();
        }
        count
    }
    
    fn skip_whitespace_inline(&mut self) {
        while !self.is_eof() && matches!(self.peek(), ' ' | '\t') {
            self.advance();
        }
    }
    
    fn skip_line(&mut self) {
        while !self.is_eof() && self.peek() != '\n' {
            self.advance();
        }
    }
    
    fn skip_block_comment(&mut self) -> Result<(), String> {
        self.advance(); // /
        self.advance(); // *
        
        let mut depth = 1;
        while !self.is_eof() && depth > 0 {
            if self.peek() == '/' && self.peek_ahead(1) == '*' {
                self.advance();
                self.advance();
                depth += 1;
            } else if self.peek() == '*' && self.peek_ahead(1) == '/' {
                self.advance();
                self.advance();
                depth -= 1;
            } else {
                if self.peek() == '\n' {
                    self.line += 1;
                    self.column = 0;
                }
                self.advance();
            }
        }
        
        if depth > 0 {
            return Err("Unterminated block comment".to_string());
        }
        
        Ok(())
    }
    
    fn advance(&mut self) -> char {
        let ch = self.source[self.pos];
        self.pos += 1;
        self.column += 1;
        ch
    }
    
    fn peek(&self) -> char {
        if self.is_eof() { '\0' } else { self.source[self.pos] }
    }
    
    fn peek_ahead(&self, n: usize) -> char {
        let pos = self.pos + n;
        if pos >= self.source.len() { '\0' } else { self.source[pos] }
    }
    
    fn is_eof(&self) -> bool {
        self.pos >= self.source.len()
    }
    
    fn make_token(&self, typ: TokenType) -> Token {
        Token::new(typ, self.line, self.column, self.file.clone())
    }
}

// Export for use in other modules
pub use TokenType::*;
