/// m5rcode compiler library
use std::collections::HashMap;
use std::fmt;

// Token types
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Null,
    
    // Identifiers and keywords
    Ident(String),
    Fn, Class, If, Else, While, For, In, Return, Import, Let, Const,
    
    // Operators
    Plus, Minus, Star, Slash, Percent,
    Eq, EqEq, NotEq, Lt, Gt, LtEq, GtEq,
    And, Or, Not,
    
    // Delimiters
    LParen, RParen, LBrace, RBrace, LBracket, RBracket,
    Comma, Dot, Colon, Semi, Arrow,
    
    // Special
    Newline, Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub typ: TokenType,
    pub line: usize,
}

/// Lexer - tokenizes source code
pub struct Lexer {
    source: Vec<char>,
    pos: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            source: source.chars().collect(),
            pos: 0,
            line: 1,
        }
    }
    
    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        
        while !self.is_eof() {
            self.skip_whitespace();
            if self.is_eof() { break; }
            
            // Comments
            if self.peek() == '#' {
                self.skip_line();
                continue;
            }
            
            let token = self.next_token()?;
            tokens.push(token);
        }
        
        tokens.push(Token { typ: TokenType::Eof, line: self.line });
        Ok(tokens)
    }
    
    fn next_token(&mut self) -> Result<Token, String> {
        let ch = self.advance();
        let line = self.line;
        
        let typ = match ch {
            '\n' => { self.line += 1; TokenType::Newline },
            '+' => TokenType::Plus,
            '-' => if self.peek() == '>' { self.advance(); TokenType::Arrow } else { TokenType::Minus },
            '*' => TokenType::Star,
            '/' => TokenType::Slash,
            '%' => TokenType::Percent,
            '(' => TokenType::LParen,
            ')' => TokenType::RParen,
            '{' => TokenType::LBrace,
            '}' => TokenType::RBrace,
            '[' => TokenType::LBracket,
            ']' => TokenType::RBracket,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            ':' => TokenType::Colon,
            ';' => TokenType::Semi,
            '=' => if self.peek() == '=' { self.advance(); TokenType::EqEq } else { TokenType::Eq },
            '!' => if self.peek() == '=' { self.advance(); TokenType::NotEq } else { TokenType::Not },
            '<' => if self.peek() == '=' { self.advance(); TokenType::LtEq } else { TokenType::Lt },
            '>' => if self.peek() == '=' { self.advance(); TokenType::GtEq } else { TokenType::Gt },
            '&' => if self.peek() == '&' { self.advance(); TokenType::And } else { return Err(format!("Unexpected character: {}", ch)); },
            '|' => if self.peek() == '|' { self.advance(); TokenType::Or } else { return Err(format!("Unexpected character: {}", ch)); },
            '"' => self.read_string()?,
            _ if ch.is_ascii_digit() => self.read_number(ch)?,
            _ if ch.is_alphabetic() || ch == '_' => self.read_ident(ch),
            _ => return Err(format!("Unexpected character: {}", ch)),
        };
        
        Ok(Token { typ, line })
    }
    
    fn read_string(&mut self) -> Result<TokenType, String> {
        let mut s = String::new();
        while !self.is_eof() && self.peek() != '"' {
            s.push(self.advance());
        }
        if self.is_eof() {
            return Err("Unterminated string".to_string());
        }
        self.advance(); // closing "
        Ok(TokenType::String(s))
    }
    
    fn read_number(&mut self, first: char) -> Result<TokenType, String> {
        let mut num = String::from(first);
        while !self.is_eof() && self.peek().is_ascii_digit() {
            num.push(self.advance());
        }
        if !self.is_eof() && self.peek() == '.' {
            num.push(self.advance());
            while !self.is_eof() && self.peek().is_ascii_digit() {
                num.push(self.advance());
            }
            Ok(TokenType::Float(num.parse().unwrap()))
        } else {
            Ok(TokenType::Int(num.parse().unwrap()))
        }
    }
    
    fn read_ident(&mut self, first: char) -> TokenType {
        let mut ident = String::from(first);
        while !self.is_eof() && (self.peek().is_alphanumeric() || self.peek() == '_') {
            ident.push(self.advance());
        }
        
        match ident.as_str() {
            "fn" => TokenType::Fn,
            "class" => TokenType::Class,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "while" => TokenType::While,
            "for" => TokenType::For,
            "in" => TokenType::In,
            "return" => TokenType::Return,
            "import" => TokenType::Import,
            "let" => TokenType::Let,
            "const" => TokenType::Const,
            "true" => TokenType::Bool(true),
            "false" => TokenType::Bool(false),
            "null" => TokenType::Null,
            _ => TokenType::Ident(ident),
        }
    }
    
    fn advance(&mut self) -> char {
        let ch = self.source[self.pos];
        self.pos += 1;
        ch
    }
    
    fn peek(&self) -> char {
        if self.is_eof() { '\0' } else { self.source[self.pos] }
    }
    
    fn is_eof(&self) -> bool {
        self.pos >= self.source.len()
    }
    
    fn skip_whitespace(&mut self) {
        while !self.is_eof() && self.peek().is_whitespace() && self.peek() != '\n' {
            self.advance();
        }
    }
    
    fn skip_line(&mut self) {
        while !self.is_eof() && self.peek() != '\n' {
            self.advance();
        }
    }
}

// AST nodes
#[derive(Debug, Clone)]
pub enum Expr {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Null,
    Ident(String),
    Binary { op: String, left: Box<Expr>, right: Box<Expr> },
    Call { func: Box<Expr>, args: Vec<Expr> },
    Member { obj: Box<Expr>, field: String },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Let { name: String, value: Expr },
    Assign { name: String, value: Expr },
    If { cond: Expr, then_block: Vec<Stmt>, else_block: Option<Vec<Stmt>> },
    While { cond: Expr, body: Vec<Stmt> },
    Return(Option<Expr>),
    Function { name: String, params: Vec<String>, body: Vec<Stmt> },
    Import(String),
}

/// Parser - builds AST from tokens
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }
    
    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut stmts = Vec::new();
        while !self.is_eof() {
            self.skip_newlines();
            if self.is_eof() { break; }
            stmts.push(self.parse_stmt()?);
        }
        Ok(stmts)
    }
    
    fn parse_stmt(&mut self) -> Result<Stmt, String> {
        match &self.peek().typ {
            TokenType::Let | TokenType::Const => self.parse_let(),
            TokenType::Fn => self.parse_function(),
            TokenType::If => self.parse_if(),
            TokenType::While => self.parse_while(),
            TokenType::Return => self.parse_return(),
            TokenType::Import => self.parse_import(),
            TokenType::Ident(_) => {
                let name = if let TokenType::Ident(n) = &self.peek().typ { n.clone() } else { unreachable!() };
                self.advance();
                if matches!(self.peek().typ, TokenType::Eq) {
                    self.advance();
                    let value = self.parse_expr()?;
                    self.skip_newlines();
                    Ok(Stmt::Assign { name, value })
                } else {
                    self.pos -= 1;
                    let expr = self.parse_expr()?;
                    self.skip_newlines();
                    Ok(Stmt::Expr(expr))
                }
            },
            _ => {
                let expr = self.parse_expr()?;
                self.skip_newlines();
                Ok(Stmt::Expr(expr))
            }
        }
    }
    
    fn parse_let(&mut self) -> Result<Stmt, String> {
        self.advance(); // let/const
        let name = self.expect_ident()?;
        self.expect(TokenType::Eq)?;
        let value = self.parse_expr()?;
        self.skip_newlines();
        Ok(Stmt::Let { name, value })
    }
    
    fn parse_function(&mut self) -> Result<Stmt, String> {
        self.advance(); // fn
        let name = self.expect_ident()?;
        self.expect(TokenType::LParen)?;
        let mut params = Vec::new();
        while !matches!(self.peek().typ, TokenType::RParen) {
            params.push(self.expect_ident()?);
            if matches!(self.peek().typ, TokenType::Comma) {
                self.advance();
            }
        }
        self.expect(TokenType::RParen)?;
        self.expect(TokenType::LBrace)?;
        let body = self.parse_block()?;
        Ok(Stmt::Function { name, params, body })
    }
    
    fn parse_if(&mut self) -> Result<Stmt, String> {
        self.advance(); // if
        let cond = self.parse_expr()?;
        self.expect(TokenType::LBrace)?;
        let then_block = self.parse_block()?;
        let else_block = if matches!(self.peek().typ, TokenType::Else) {
            self.advance();
            self.expect(TokenType::LBrace)?;
            Some(self.parse_block()?)
        } else {
            None
        };
        Ok(Stmt::If { cond, then_block, else_block })
    }
    
    fn parse_while(&mut self) -> Result<Stmt, String> {
        self.advance(); // while
        let cond = self.parse_expr()?;
        self.expect(TokenType::LBrace)?;
        let body = self.parse_block()?;
        Ok(Stmt::While { cond, body })
    }
    
    fn parse_return(&mut self) -> Result<Stmt, String> {
        self.advance(); // return
        if matches!(self.peek().typ, TokenType::Newline | TokenType::RBrace) {
            Ok(Stmt::Return(None))
        } else {
            Ok(Stmt::Return(Some(self.parse_expr()?)))
        }
    }
    
    fn parse_import(&mut self) -> Result<Stmt, String> {
        self.advance(); // import
        let module = self.expect_ident()?;
        self.skip_newlines();
        Ok(Stmt::Import(module))
    }
    
    fn parse_block(&mut self) -> Result<Vec<Stmt>, String> {
        let mut stmts = Vec::new();
        self.skip_newlines();
        while !matches!(self.peek().typ, TokenType::RBrace | TokenType::Eof) {
            stmts.push(self.parse_stmt()?);
            self.skip_newlines();
        }
        self.expect(TokenType::RBrace)?;
        Ok(stmts)
    }
    
    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_comparison()
    }
    
    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_additive()?;
        while matches!(self.peek().typ, TokenType::EqEq | TokenType::NotEq | TokenType::Lt | TokenType::Gt | TokenType::LtEq | TokenType::GtEq) {
            let op = match &self.advance().typ {
                TokenType::EqEq => "==",
                TokenType::NotEq => "!=",
                TokenType::Lt => "<",
                TokenType::Gt => ">",
                TokenType::LtEq => "<=",
                TokenType::GtEq => ">=",
                _ => unreachable!(),
            }.to_string();
            let right = self.parse_additive()?;
            left = Expr::Binary { op, left: Box::new(left), right: Box::new(right) };
        }
        Ok(left)
    }
    
    fn parse_additive(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_multiplicative()?;
        while matches!(self.peek().typ, TokenType::Plus | TokenType::Minus) {
            let op = match &self.advance().typ {
                TokenType::Plus => "+",
                TokenType::Minus => "-",
                _ => unreachable!(),
            }.to_string();
            let right = self.parse_multiplicative()?;
            left = Expr::Binary { op, left: Box::new(left), right: Box::new(right) };
        }
        Ok(left)
    }
    
    fn parse_multiplicative(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_postfix()?;
        while matches!(self.peek().typ, TokenType::Star | TokenType::Slash | TokenType::Percent) {
            let op = match &self.advance().typ {
                TokenType::Star => "*",
                TokenType::Slash => "/",
                TokenType::Percent => "%",
                _ => unreachable!(),
            }.to_string();
            let right = self.parse_postfix()?;
            left = Expr::Binary { op, left: Box::new(left), right: Box::new(right) };
        }
        Ok(left)
    }
    
    fn parse_postfix(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;
        loop {
            match &self.peek().typ {
                TokenType::LParen => {
                    self.advance();
                    let mut args = Vec::new();
                    while !matches!(self.peek().typ, TokenType::RParen) {
                        args.push(self.parse_expr()?);
                        if matches!(self.peek().typ, TokenType::Comma) {
                            self.advance();
                        }
                    }
                    self.expect(TokenType::RParen)?;
                    expr = Expr::Call { func: Box::new(expr), args };
                },
                TokenType::Dot => {
                    self.advance();
                    let field = self.expect_ident()?;
                    expr = Expr::Member { obj: Box::new(expr), field };
                },
                _ => break,
            }
        }
        Ok(expr)
    }
    
    fn parse_primary(&mut self) -> Result<Expr, String> {
        match &self.advance().typ {
            TokenType::Int(n) => Ok(Expr::Int(*n)),
            TokenType::Float(f) => Ok(Expr::Float(*f)),
            TokenType::String(s) => Ok(Expr::String(s.clone())),
            TokenType::Bool(b) => Ok(Expr::Bool(*b)),
            TokenType::Null => Ok(Expr::Null),
            TokenType::Ident(name) => Ok(Expr::Ident(name.clone())),
            TokenType::LParen => {
                let expr = self.parse_expr()?;
                self.expect(TokenType::RParen)?;
                Ok(expr)
            },
            _ => Err("Unexpected token in expression".to_string()),
        }
    }
    
    fn advance(&mut self) -> Token {
        let token = self.tokens[self.pos].clone();
        self.pos += 1;
        token
    }
    
    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }
    
    fn is_eof(&self) -> bool {
        matches!(self.peek().typ, TokenType::Eof)
    }
    
    fn expect(&mut self, typ: TokenType) -> Result<(), String> {
        if std::mem::discriminant(&self.peek().typ) == std::mem::discriminant(&typ) {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {:?}, found {:?}", typ, self.peek().typ))
        }
    }
    
    fn expect_ident(&mut self) -> Result<String, String> {
        match &self.advance().typ {
            TokenType::Ident(name) => Ok(name.clone()),
            _ => Err("Expected identifier".to_string()),
        }
    }
    
    fn skip_newlines(&mut self) {
        while matches!(self.peek().typ, TokenType::Newline) {
            self.advance();
        }
    }
}

// Runtime values
#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Null,
    Function { params: Vec<String>, body: Vec<Stmt>, closure: HashMap<String, Value> },
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{}", n),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::String(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Null => write!(f, "null"),
            Value::Function { .. } => write!(f, "<function>"),
        }
    }
}

/// Interpreter - executes AST
pub struct Interpreter {
    globals: HashMap<String, Value>,
    locals: Vec<HashMap<String, Value>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut globals = HashMap::new();
        // Built-in std.io module
        globals.insert("io".to_string(), Value::Null); // Placeholder
        
        Interpreter {
            globals,
            locals: vec![],
        }
    }
    
    pub fn execute(&mut self, stmts: &[Stmt]) -> Result<(), String> {
        for stmt in stmts {
            self.exec_stmt(stmt)?;
        }
        Ok(())
    }
    
    fn exec_stmt(&mut self, stmt: &Stmt) -> Result<Option<Value>, String> {
        match stmt {
            Stmt::Expr(expr) => {
                self.eval_expr(expr)?;
                Ok(None)
            },
            Stmt::Let { name, value } => {
                let val = self.eval_expr(value)?;
                self.set_var(name, val);
                Ok(None)
            },
            Stmt::Assign { name, value } => {
                let val = self.eval_expr(value)?;
                self.set_var(name, val);
                Ok(None)
            },
            Stmt::If { cond, then_block, else_block } => {
                let cond_val = self.eval_expr(cond)?;
                if self.is_truthy(&cond_val) {
                    for s in then_block {
                        if let Some(v) = self.exec_stmt(s)? {
                            return Ok(Some(v));
                        }
                    }
                } else if let Some(else_b) = else_block {
                    for s in else_b {
                        if let Some(v) = self.exec_stmt(s)? {
                            return Ok(Some(v));
                        }
                    }
                }
                Ok(None)
            },
            Stmt::While { cond, body } => {
                loop {
                    let cond_val = self.eval_expr(cond)?;
                    if !self.is_truthy(&cond_val) {
                        break;
                    }
                    for s in body {
                        if let Some(v) = self.exec_stmt(s)? {
                            return Ok(Some(v));
                        }
                    }
                }
                Ok(None)
            },
            Stmt::Return(expr) => {
                let val = if let Some(e) = expr {
                    self.eval_expr(e)?
                } else {
                    Value::Null
                };
                Ok(Some(val))
            },
            Stmt::Function { name, params, body } => {
                let func = Value::Function {
                    params: params.clone(),
                    body: body.clone(),
                    closure: self.capture_env(),
                };
                self.set_var(name, func);
                Ok(None)
            },
            Stmt::Import(_) => Ok(None), // Stub
        }
    }
    
    fn eval_expr(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Int(n) => Ok(Value::Int(*n)),
            Expr::Float(f) => Ok(Value::Float(*f)),
            Expr::String(s) => Ok(Value::String(s.clone())),
            Expr::Bool(b) => Ok(Value::Bool(*b)),
            Expr::Null => Ok(Value::Null),
            Expr::Ident(name) => self.get_var(name),
            Expr::Binary { op, left, right } => {
                let l = self.eval_expr(left)?;
                let r = self.eval_expr(right)?;
                self.eval_binary(op, l, r)
            },
            Expr::Call { func, args } => {
                let f = self.eval_expr(func)?;
                let arg_vals: Result<Vec<_>, _> = args.iter().map(|a| self.eval_expr(a)).collect();
                self.call_function(f, arg_vals?)
            },
            Expr::Member { obj, field } => {
                let o = self.eval_expr(obj)?;
                self.get_member(o, field)
            },
        }
    }
    
    fn eval_binary(&self, op: &str, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Int(l), Value::Int(r)) => match op {
                "+" => Ok(Value::Int(l + r)),
                "-" => Ok(Value::Int(l - r)),
                "*" => Ok(Value::Int(l * r)),
                "/" => Ok(Value::Int(l / r)),
                "%" => Ok(Value::Int(l % r)),
                "==" => Ok(Value::Bool(l == r)),
                "!=" => Ok(Value::Bool(l != r)),
                "<" => Ok(Value::Bool(l < r)),
                ">" => Ok(Value::Bool(l > r)),
                "<=" => Ok(Value::Bool(l <= r)),
                ">=" => Ok(Value::Bool(l >= r)),
                _ => Err(format!("Unknown operator: {}", op)),
            },
            (Value::String(l), Value::String(r)) if op == "+" => Ok(Value::String(l + &r)),
            _ => Err("Type error in binary operation".to_string()),
        }
    }
    
    fn call_function(&mut self, func: Value, args: Vec<Value>) -> Result<Value, String> {
        match func {
            Value::Function { params, body, closure } => {
                if params.len() != args.len() {
                    return Err("Argument count mismatch".to_string());
                }
                
                // Create new scope
                let mut new_scope = closure;
                for (param, arg) in params.iter().zip(args.iter()) {
                    new_scope.insert(param.clone(), arg.clone());
                }
                self.locals.push(new_scope);
                
                // Execute function body
                let mut result = Value::Null;
                for stmt in &body {
                    if let Some(v) = self.exec_stmt(stmt)? {
                        result = v;
                        break;
                    }
                }
                
                self.locals.pop();
                Ok(result)
            },
            _ => Err("Not a function".to_string()),
        }
    }
    
    fn get_member(&self, obj: Value, field: &str) -> Result<Value, String> {
        // Handle io.println specially
        if matches!(obj, Value::Null) && field == "println" {
            // Return a built-in function marker
            Ok(Value::Function {
                params: vec!["msg".to_string()],
                body: vec![],
                closure: HashMap::new(),
            })
        } else {
            Err("Member access not implemented".to_string())
        }
    }
    
    fn is_truthy(&self, val: &Value) -> bool {
        match val {
            Value::Bool(b) => *b,
            Value::Null => false,
            Value::Int(0) => false,
            _ => true,
        }
    }
    
    fn get_var(&self, name: &str) -> Result<Value, String> {
        // Check locals first (reverse order)
        for scope in self.locals.iter().rev() {
            if let Some(val) = scope.get(name) {
                return Ok(val.clone());
            }
        }
        // Check globals
        self.globals.get(name)
            .cloned()
            .ok_or_else(|| format!("Undefined variable: {}", name))
    }
    
    fn set_var(&mut self, name: &str, value: Value) {
        if let Some(scope) = self.locals.last_mut() {
            scope.insert(name.to_string(), value);
        } else {
            self.globals.insert(name.to_string(), value);
        }
    }
    
    fn capture_env(&self) -> HashMap<String, Value> {
        let mut env = self.globals.clone();
        for scope in &self.locals {
            env.extend(scope.clone());
        }
        env
    }
}
