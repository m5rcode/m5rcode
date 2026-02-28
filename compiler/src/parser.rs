/// Parser module - Uses new lexer, outputs to old AST for compatibility
use crate::lexer::{Token, TokenType};
use std::collections::HashMap;

// Old AST types for compatibility
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
    List(Vec<Expr>),
    Index { obj: Box<Expr>, index: Box<Expr> },
    Object { fields: Vec<(String, Expr)> },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Let { name: String, value: Expr },
    Assign { name: String, value: Expr },
    If { cond: Expr, then_block: Vec<Stmt>, else_block: Option<Vec<Stmt>> },
    While { cond: Expr, body: Vec<Stmt> },
    For { var: String, iter: Expr, body: Vec<Stmt> },
    Return(Option<Expr>),
    Function { name: String, params: Vec<String>, body: Vec<Stmt> },
    Class { name: String, fields: Vec<(String, String)>, methods: Vec<Stmt> },
    Import(String),
}

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
            TokenType::Class => self.parse_class(),
            TokenType::If => self.parse_if(),
            TokenType::While => self.parse_while(),
            TokenType::For => self.parse_for(),
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
        self.advance();
        let name = self.expect_ident()?;
        self.expect_token(TokenType::Eq)?;
        let value = self.parse_expr()?;
        self.skip_newlines();
        Ok(Stmt::Let { name, value })
    }
    
    fn parse_function(&mut self) -> Result<Stmt, String> {
        self.advance();
        let name = self.expect_ident()?;
        self.expect_token(TokenType::LParen)?;
        let mut params = Vec::new();
        while !matches!(self.peek().typ, TokenType::RParen) {
            params.push(self.expect_ident()?);
            if matches!(self.peek().typ, TokenType::Comma) {
                self.advance();
            }
        }
        self.expect_token(TokenType::RParen)?;
        self.expect_token(TokenType::LBrace)?;
        let body = self.parse_block()?;
        Ok(Stmt::Function { name, params, body })
    }
    
    fn parse_class(&mut self) -> Result<Stmt, String> {
        self.advance();
        let name = self.expect_ident()?;
        self.expect_token(TokenType::LBrace)?;
        
        let mut fields = Vec::new();
        let mut methods = Vec::new();
        
        self.skip_newlines();
        while !matches!(self.peek().typ, TokenType::RBrace | TokenType::Eof) {
            if matches!(self.peek().typ, TokenType::Fn) {
                methods.push(self.parse_function()?);
            } else {
                let field_name = self.expect_ident()?;
                self.expect_token(TokenType::Colon)?;
                let field_type = self.expect_ident()?;
                fields.push((field_name, field_type));
                self.skip_newlines();
            }
            self.skip_newlines();
        }
        
        self.expect_token(TokenType::RBrace)?;
        Ok(Stmt::Class { name, fields, methods })
    }
    
    fn parse_if(&mut self) -> Result<Stmt, String> {
        self.advance();
        let cond = self.parse_expr()?;
        self.expect_token(TokenType::LBrace)?;
        let then_block = self.parse_block()?;
        let else_block = if matches!(self.peek().typ, TokenType::Else) {
            self.advance();
            self.expect_token(TokenType::LBrace)?;
            Some(self.parse_block()?)
        } else {
            None
        };
        Ok(Stmt::If { cond, then_block, else_block })
    }
    
    fn parse_while(&mut self) -> Result<Stmt, String> {
        self.advance();
        let cond = self.parse_expr()?;
        self.expect_token(TokenType::LBrace)?;
        let body = self.parse_block()?;
        Ok(Stmt::While { cond, body })
    }
    
    fn parse_for(&mut self) -> Result<Stmt, String> {
        self.advance();
        let var = self.expect_ident()?;
        if !matches!(self.peek().typ, TokenType::In) {
            return Err(format!("Expected 'in' in for loop at line {}", self.peek().line));
        }
        self.advance();
        let iter = self.parse_expr()?;
        self.expect_token(TokenType::LBrace)?;
        let body = self.parse_block()?;
        Ok(Stmt::For { var, iter, body })
    }
    
    fn parse_return(&mut self) -> Result<Stmt, String> {
        self.advance();
        if matches!(self.peek().typ, TokenType::Newline | TokenType::RBrace) {
            Ok(Stmt::Return(None))
        } else {
            Ok(Stmt::Return(Some(self.parse_expr()?)))
        }
    }
    
    fn parse_import(&mut self) -> Result<Stmt, String> {
        self.advance();
        let module = self.expect_ident()?;
        let mut full_path = module;
        while matches!(self.peek().typ, TokenType::Dot) {
            self.advance();
            full_path.push('.');
            full_path.push_str(&self.expect_ident()?);
        }
        self.skip_newlines();
        Ok(Stmt::Import(full_path))
    }
    
    fn parse_block(&mut self) -> Result<Vec<Stmt>, String> {
        let mut stmts = Vec::new();
        self.skip_newlines();
        while !matches!(self.peek().typ, TokenType::RBrace | TokenType::Eof) {
            stmts.push(self.parse_stmt()?);
            self.skip_newlines();
        }
        self.expect_token(TokenType::RBrace)?;
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
                    self.expect_token(TokenType::RParen)?;
                    expr = Expr::Call { func: Box::new(expr), args };
                },
                TokenType::Dot => {
                    self.advance();
                    let field = self.expect_ident()?;
                    expr = Expr::Member { obj: Box::new(expr), field };
                },
                TokenType::LBracket => {
                    self.advance();
                    let index = self.parse_expr()?;
                    self.expect_token(TokenType::RBracket)?;
                    expr = Expr::Index { obj: Box::new(expr), index: Box::new(index) };
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
            TokenType::StringLit(s) => Ok(Expr::String(s.clone())),
            TokenType::Bool(b) => Ok(Expr::Bool(*b)),
            TokenType::Null => Ok(Expr::Null),
            TokenType::Ident(name) => Ok(Expr::Ident(name.clone())),
            TokenType::LParen => {
                let expr = self.parse_expr()?;
                self.expect_token(TokenType::RParen)?;
                Ok(expr)
            },
            TokenType::LBracket => {
                let mut items = Vec::new();
                while !matches!(self.peek().typ, TokenType::RBracket) {
                    items.push(self.parse_expr()?);
                    if matches!(self.peek().typ, TokenType::Comma) {
                        self.advance();
                    }
                }
                self.expect_token(TokenType::RBracket)?;
                Ok(Expr::List(items))
            },
            TokenType::LBrace => {
                let mut fields = Vec::new();
                while !matches!(self.peek().typ, TokenType::RBrace) {
                    let key = self.expect_ident()?;
                    self.expect_token(TokenType::Colon)?;
                    let value = self.parse_expr()?;
                    fields.push((key, value));
                    if matches!(self.peek().typ, TokenType::Comma) {
                        self.advance();
                    }
                }
                self.expect_token(TokenType::RBrace)?;
                Ok(Expr::Object { fields })
            },
            tok => Err(format!("Unexpected token in expression: {:?} at line {}", tok, self.tokens[self.pos - 1].line)),
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
    
    fn expect_token(&mut self, expected: TokenType) -> Result<(), String> {
        if std::mem::discriminant(&self.peek().typ) == std::mem::discriminant(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {:?}, found {:?} at line {}", expected, self.peek().typ, self.peek().line))
        }
    }
    
    fn expect_ident(&mut self) -> Result<String, String> {
        match &self.advance().typ {
            TokenType::Ident(name) => Ok(name.clone()),
            tok => Err(format!("Expected identifier, found {:?} at line {}", tok, self.tokens[self.pos - 1].line)),
        }
    }
    
    fn skip_newlines(&mut self) {
        while matches!(self.peek().typ, TokenType::Newline | TokenType::Indent | TokenType::Dedent) {
            self.advance();
        }
    }
}
