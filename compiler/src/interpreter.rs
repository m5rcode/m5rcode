/// Interpreter module - Executes parsed AST
use crate::parser::{Expr, Stmt};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Null,
    List(Vec<Value>),
    Object(HashMap<String, Value>),
    Function { params: Vec<String>, body: Vec<Stmt>, closure: HashMap<String, Value> },
    Class { name: String, fields: Vec<(String, String)>, methods: HashMap<String, Value> },
    Instance { class_name: String, fields: HashMap<String, Value> },
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{}", n),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::String(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Null => write!(f, "null"),
            Value::List(items) => {
                write!(f, "[")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            },
            Value::Object(fields) => {
                write!(f, "{{")?;
                for (i, (k, v)) in fields.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}: {}", k, v)?;
                }
                write!(f, "}}")
            },
            Value::Function { .. } => write!(f, "<function>"),
            Value::Class { name, .. } => write!(f, "<class {}>", name),
            Value::Instance { class_name, .. } => write!(f, "<{} instance>", class_name),
        }
    }
}

pub struct Interpreter {
    globals: HashMap<String, Value>,
    locals: Vec<HashMap<String, Value>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut globals = HashMap::new();
        let mut io_methods = HashMap::new();
        io_methods.insert("println".to_string(), Value::Function {
            params: vec!["msg".to_string()],
            body: vec![],
            closure: HashMap::new(),
        });
        globals.insert("io".to_string(), Value::Object(io_methods));
        
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
            Stmt::For { var, iter, body } => {
                let iter_val = self.eval_expr(iter)?;
                match iter_val {
                    Value::List(items) => {
                        for item in items {
                            self.set_var(var, item);
                            for s in body {
                                if let Some(v) = self.exec_stmt(s)? {
                                    return Ok(Some(v));
                                }
                            }
                        }
                    },
                    _ => return Err("For loop requires iterable".to_string()),
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
            Stmt::Class { name, fields, methods } => {
                let mut method_map = HashMap::new();
                for method in methods {
                    if let Stmt::Function { name: method_name, params, body } = method {
                        method_map.insert(method_name.clone(), Value::Function {
                            params: params.clone(),
                            body: body.clone(),
                            closure: HashMap::new(),
                        });
                    }
                }
                let class = Value::Class {
                    name: name.clone(),
                    fields: fields.clone(),
                    methods: method_map,
                };
                self.set_var(name, class);
                Ok(None)
            },
            Stmt::Import(module) => {
                if module == "std.io" {
                    let mut io_methods = HashMap::new();
                    io_methods.insert("println".to_string(), Value::Function {
                        params: vec!["msg".to_string()],
                        body: vec![],
                        closure: HashMap::new(),
                    });
                    self.globals.insert("io".to_string(), Value::Object(io_methods));
                }
                Ok(None)
            },
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
            Expr::List(items) => {
                let vals: Result<Vec<_>, _> = items.iter().map(|i| self.eval_expr(i)).collect();
                Ok(Value::List(vals?))
            },
            Expr::Index { obj, index } => {
                let o = self.eval_expr(obj)?;
                let i = self.eval_expr(index)?;
                self.eval_index(o, i)
            },
            Expr::Object { fields } => {
                let mut map = HashMap::new();
                for (k, v) in fields {
                    map.insert(k.clone(), self.eval_expr(v)?);
                }
                Ok(Value::Object(map))
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
            (Value::String(l), Value::Int(r)) if op == "+" => Ok(Value::String(l + &r.to_string())),
            (Value::Int(l), Value::String(r)) if op == "+" => Ok(Value::String(l.to_string() + &r)),
            (Value::String(l), other) if op == "+" => Ok(Value::String(l + &format!("{}", other))),
            (other, Value::String(r)) if op == "+" => Ok(Value::String(format!("{}", other) + &r)),
            _ => Err("Type error in binary operation".to_string()),
        }
    }
    
    fn eval_index(&self, obj: Value, index: Value) -> Result<Value, String> {
        match (obj, index) {
            (Value::List(items), Value::Int(i)) => {
                let idx = if i < 0 {
                    (items.len() as i64 + i) as usize
                } else {
                    i as usize
                };
                items.get(idx).cloned().ok_or_else(|| "Index out of bounds".to_string())
            },
            (Value::String(s), Value::Int(i)) => {
                let chars: Vec<char> = s.chars().collect();
                let idx = if i < 0 {
                    (chars.len() as i64 + i) as usize
                } else {
                    i as usize
                };
                chars.get(idx)
                    .map(|&c| Value::String(c.to_string()))
                    .ok_or_else(|| "Index out of bounds".to_string())
            },
            _ => Err("Invalid index operation".to_string()),
        }
    }
    
    fn get_member(&mut self, obj: Value, field: &str) -> Result<Value, String> {
        match obj {
            Value::Object(ref map) => {
                map.get(field).cloned().ok_or_else(|| format!("Field '{}' not found", field))
            },
            Value::Class { ref methods, .. } => {
                methods.get(field).cloned().ok_or_else(|| format!("Class has no method '{}'", field))
            },
            Value::Instance { ref fields, .. } => {
                fields.get(field).cloned().ok_or_else(|| format!("Field '{}' not found", field))
            },
            Value::String(ref s) => {
                match field {
                    "len" => Ok(Value::Function {
                        params: vec![],
                        body: vec![],
                        closure: {
                            let mut map = HashMap::new();
                            map.insert("__self__".to_string(), Value::Int(s.len() as i64));
                            map
                        },
                    }),
                    "upper" => Ok(Value::Function {
                        params: vec![],
                        body: vec![],
                        closure: {
                            let mut map = HashMap::new();
                            map.insert("__self__".to_string(), Value::String(s.to_uppercase()));
                            map
                        },
                    }),
                    _ => Err(format!("String has no method '{}'", field)),
                }
            },
            Value::List(ref items) => {
                match field {
                    "len" => Ok(Value::Function {
                        params: vec![],
                        body: vec![],
                        closure: {
                            let mut map = HashMap::new();
                            map.insert("__self__".to_string(), Value::Int(items.len() as i64));
                            map
                        },
                    }),
                    _ => Err(format!("List has no method '{}'", field)),
                }
            },
            _ => Err(format!("Cannot access member '{}' on this type", field))
        }
    }
    
    fn call_function(&mut self, func: Value, args: Vec<Value>) -> Result<Value, String> {
        match func {
            Value::Function { params, body, closure } => {
                if body.is_empty() {
                    if params.len() == 1 && params[0] == "msg" {
                        if let Some(arg) = args.first() {
                            println!("{}", arg);
                        }
                        return Ok(Value::Null);
                    }
                    if let Some(self_val) = closure.get("__self__") {
                        return Ok(self_val.clone());
                    }
                }
                
                if params.len() != args.len() {
                    return Err(format!("Expected {} arguments, got {}", params.len(), args.len()));
                }
                
                let mut new_scope = closure;
                for (param, arg) in params.iter().zip(args.iter()) {
                    new_scope.insert(param.clone(), arg.clone());
                }
                self.locals.push(new_scope);
                
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
            Value::Class { name, fields: _, methods } => {
                if let Some(new_method) = methods.get("new") {
                    if let Value::Function { body, .. } = new_method {
                        self.locals.push(HashMap::new());
                        let mut result_fields = HashMap::new();
                        
                        for stmt in body {
                            if let Stmt::Return(Some(Expr::Object { fields: obj_fields })) = stmt {
                                for (k, v) in obj_fields {
                                    result_fields.insert(k.clone(), self.eval_expr(v)?);
                                }
                                break;
                            }
                        }
                        
                        self.locals.pop();
                        return Ok(Value::Instance {
                            class_name: name,
                            fields: result_fields,
                        });
                    }
                }
                Err(format!("Class '{}' has no constructor", name))
            },
            _ => Err("Not a function".to_string()),
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
        for scope in self.locals.iter().rev() {
            if let Some(val) = scope.get(name) {
                return Ok(val.clone());
            }
        }
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
