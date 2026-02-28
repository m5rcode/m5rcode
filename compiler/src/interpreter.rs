/// Interpreter module - Executes parsed AST with enhanced error handling
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
    NativeFunction { name: String, arity: usize },
}

#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub message: String,
    pub line: Option<usize>,
    pub column: Option<usize>,
}

impl RuntimeError {
    pub fn new(message: String) -> Self {
        RuntimeError { message, line: None, column: None }
    }
    
    pub fn with_location(message: String, line: usize, column: usize) -> Self {
        RuntimeError { message, line: Some(line), column: Some(column) }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let (Some(line), Some(col)) = (self.line, self.column) {
            write!(f, "Runtime error at line {}, column {}: {}", line, col, self.message)
        } else {
            write!(f, "Runtime error: {}", self.message)
        }
    }
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
            Value::NativeFunction { name, .. } => write!(f, "<native function {}>", name),
        }
    }
}

impl Value {
    /// Type name for better error messages
    pub fn type_name(&self) -> &str {
        match self {
            Value::Int(_) => "int",
            Value::Float(_) => "float",
            Value::String(_) => "string",
            Value::Bool(_) => "bool",
            Value::Null => "null",
            Value::List(_) => "list",
            Value::Object(_) => "object",
            Value::Function { .. } => "function",
            Value::Class { .. } => "class",
            Value::Instance { .. } => "instance",
            Value::NativeFunction { .. } => "native_function",
        }
    }
    
    /// Convert to boolean for truthiness checks
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Null => false,
            Value::Int(0) => false,
            Value::Float(f) if *f == 0.0 => false,
            Value::String(s) if s.is_empty() => false,
            Value::List(l) if l.is_empty() => false,
            _ => true,
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
        
        // Enhanced std.io module
        let mut io_methods = HashMap::new();
        io_methods.insert("println".to_string(), Value::NativeFunction { name: "println".to_string(), arity: 1 });
        io_methods.insert("print".to_string(), Value::NativeFunction { name: "print".to_string(), arity: 1 });
        io_methods.insert("input".to_string(), Value::NativeFunction { name: "input".to_string(), arity: 1 });
        globals.insert("io".to_string(), Value::Object(io_methods));
        
        // std.env module - Environment variables
        let mut env_methods = HashMap::new();
        env_methods.insert("get".to_string(), Value::NativeFunction { name: "env_get".to_string(), arity: 1 });
        env_methods.insert("set".to_string(), Value::NativeFunction { name: "env_set".to_string(), arity: 2 });
        env_methods.insert("has".to_string(), Value::NativeFunction { name: "env_has".to_string(), arity: 1 });
        globals.insert("env".to_string(), Value::Object(env_methods));
        
        // std.fs module - File system
        let mut fs_methods = HashMap::new();
        fs_methods.insert("read".to_string(), Value::NativeFunction { name: "fs_read".to_string(), arity: 1 });
        fs_methods.insert("write".to_string(), Value::NativeFunction { name: "fs_write".to_string(), arity: 2 });
        fs_methods.insert("exists".to_string(), Value::NativeFunction { name: "fs_exists".to_string(), arity: 1 });
        fs_methods.insert("delete".to_string(), Value::NativeFunction { name: "fs_delete".to_string(), arity: 1 });
        globals.insert("fs".to_string(), Value::Object(fs_methods));
        
        // std.math module
        let mut math_methods = HashMap::new();
        math_methods.insert("abs".to_string(), Value::NativeFunction { name: "abs".to_string(), arity: 1 });
        math_methods.insert("sqrt".to_string(), Value::NativeFunction { name: "sqrt".to_string(), arity: 1 });
        math_methods.insert("pow".to_string(), Value::NativeFunction { name: "pow".to_string(), arity: 2 });
        math_methods.insert("floor".to_string(), Value::NativeFunction { name: "floor".to_string(), arity: 1 });
        math_methods.insert("ceil".to_string(), Value::NativeFunction { name: "ceil".to_string(), arity: 1 });
        math_methods.insert("round".to_string(), Value::NativeFunction { name: "round".to_string(), arity: 1 });
        math_methods.insert("min".to_string(), Value::NativeFunction { name: "min".to_string(), arity: 2 });
        math_methods.insert("max".to_string(), Value::NativeFunction { name: "max".to_string(), arity: 2 });
        globals.insert("math".to_string(), Value::Object(math_methods));
        
        // std.str module (renamed from string to avoid keyword conflict)
        let mut str_methods = HashMap::new();
        str_methods.insert("len".to_string(), Value::NativeFunction { name: "len".to_string(), arity: 1 });
        str_methods.insert("upper".to_string(), Value::NativeFunction { name: "upper".to_string(), arity: 1 });
        str_methods.insert("lower".to_string(), Value::NativeFunction { name: "lower".to_string(), arity: 1 });
        str_methods.insert("trim".to_string(), Value::NativeFunction { name: "trim".to_string(), arity: 1 });
        str_methods.insert("split".to_string(), Value::NativeFunction { name: "split".to_string(), arity: 2 });
        str_methods.insert("join".to_string(), Value::NativeFunction { name: "join".to_string(), arity: 2 });
        str_methods.insert("contains".to_string(), Value::NativeFunction { name: "str_contains".to_string(), arity: 2 });
        str_methods.insert("starts_with".to_string(), Value::NativeFunction { name: "str_starts_with".to_string(), arity: 2 });
        str_methods.insert("ends_with".to_string(), Value::NativeFunction { name: "str_ends_with".to_string(), arity: 2 });
        str_methods.insert("replace".to_string(), Value::NativeFunction { name: "str_replace".to_string(), arity: 3 });
        globals.insert("str".to_string(), Value::Object(str_methods));
        
        // std.list module
        let mut list_methods = HashMap::new();
        list_methods.insert("len".to_string(), Value::NativeFunction { name: "list_len".to_string(), arity: 1 });
        list_methods.insert("push".to_string(), Value::NativeFunction { name: "push".to_string(), arity: 2 });
        list_methods.insert("pop".to_string(), Value::NativeFunction { name: "pop".to_string(), arity: 1 });
        list_methods.insert("append".to_string(), Value::NativeFunction { name: "list_append".to_string(), arity: 2 });
        list_methods.insert("contains".to_string(), Value::NativeFunction { name: "list_contains".to_string(), arity: 2 });
        list_methods.insert("map".to_string(), Value::NativeFunction { name: "map".to_string(), arity: 2 });
        list_methods.insert("filter".to_string(), Value::NativeFunction { name: "filter".to_string(), arity: 2 });
        list_methods.insert("reduce".to_string(), Value::NativeFunction { name: "reduce".to_string(), arity: 3 });
        globals.insert("list".to_string(), Value::Object(list_methods));
        
        // Global utility functions
        globals.insert("typeof".to_string(), Value::NativeFunction { name: "type".to_string(), arity: 1 });
        globals.insert("toStr".to_string(), Value::NativeFunction { name: "str".to_string(), arity: 1 });
        globals.insert("toInt".to_string(), Value::NativeFunction { name: "int".to_string(), arity: 1 });
        globals.insert("toFloat".to_string(), Value::NativeFunction { name: "float".to_string(), arity: 1 });
        globals.insert("toBool".to_string(), Value::NativeFunction { name: "bool".to_string(), arity: 1 });
        
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
            (Value::String(l), Value::String(r)) => match op {
                "+" => Ok(Value::String(l + &r)),
                "==" => Ok(Value::Bool(l == r)),
                "!=" => Ok(Value::Bool(l != r)),
                "<" => Ok(Value::Bool(l < r)),
                ">" => Ok(Value::Bool(l > r)),
                "<=" => Ok(Value::Bool(l <= r)),
                ">=" => Ok(Value::Bool(l >= r)),
                _ => Err(format!("Unknown operator for strings: {}", op)),
            },
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
            Value::NativeFunction { name, arity } => {
                if args.len() != arity {
                    return Err(format!("Function '{}' expects {} arguments, got {}", name, arity, args.len()));
                }
                self.call_native(&name, args)
            },
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
            _ => Err(format!("Cannot call value of type '{}'", func.type_name())),
        }
    }
    
    fn call_native(&mut self, name: &str, args: Vec<Value>) -> Result<Value, String> {
        match name {
            // IO functions
            "println" => {
                if let Some(arg) = args.first() {
                    println!("{}", arg);
                }
                Ok(Value::Null)
            },
            "print" => {
                if let Some(arg) = args.first() {
                    print!("{}", arg);
                }
                Ok(Value::Null)
            },
            "input" => {
                if let Some(Value::String(prompt)) = args.first() {
                    print!("{}", prompt);
                    use std::io::{self, Write};
                    io::stdout().flush().unwrap();
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    Ok(Value::String(input.trim().to_string()))
                } else {
                    Err("input() requires a string prompt".to_string())
                }
            },
            
            // Math functions
            "abs" => {
                match args.first() {
                    Some(Value::Int(n)) => Ok(Value::Int(n.abs())),
                    Some(Value::Float(f)) => Ok(Value::Float(f.abs())),
                    _ => Err("abs() requires a number".to_string()),
                }
            },
            "sqrt" => {
                match args.first() {
                    Some(Value::Int(n)) => Ok(Value::Float((*n as f64).sqrt())),
                    Some(Value::Float(f)) => Ok(Value::Float(f.sqrt())),
                    _ => Err("sqrt() requires a number".to_string()),
                }
            },
            "pow" => {
                match (&args[0], &args[1]) {
                    (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a.pow(*b as u32))),
                    (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.powf(*b))),
                    (Value::Int(a), Value::Float(b)) => Ok(Value::Float((*a as f64).powf(*b))),
                    (Value::Float(a), Value::Int(b)) => Ok(Value::Float(a.powi(*b as i32))),
                    _ => Err("pow() requires two numbers".to_string()),
                }
            },
            "floor" => {
                match args.first() {
                    Some(Value::Float(f)) => Ok(Value::Int(f.floor() as i64)),
                    Some(Value::Int(n)) => Ok(Value::Int(*n)),
                    _ => Err("floor() requires a number".to_string()),
                }
            },
            "ceil" => {
                match args.first() {
                    Some(Value::Float(f)) => Ok(Value::Int(f.ceil() as i64)),
                    Some(Value::Int(n)) => Ok(Value::Int(*n)),
                    _ => Err("ceil() requires a number".to_string()),
                }
            },
            "round" => {
                match args.first() {
                    Some(Value::Float(f)) => Ok(Value::Int(f.round() as i64)),
                    Some(Value::Int(n)) => Ok(Value::Int(*n)),
                    _ => Err("round() requires a number".to_string()),
                }
            },
            "min" => {
                match (&args[0], &args[1]) {
                    (Value::Int(a), Value::Int(b)) => Ok(Value::Int(*a.min(b))),
                    (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.min(*b))),
                    _ => Err("min() requires two numbers of the same type".to_string()),
                }
            },
            "max" => {
                match (&args[0], &args[1]) {
                    (Value::Int(a), Value::Int(b)) => Ok(Value::Int(*a.max(b))),
                    (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.max(*b))),
                    _ => Err("max() requires two numbers of the same type".to_string()),
                }
            },
            
            // String functions
            "len" => {
                match args.first() {
                    Some(Value::String(s)) => Ok(Value::Int(s.len() as i64)),
                    _ => Err("len() requires a string".to_string()),
                }
            },
            "upper" => {
                match args.first() {
                    Some(Value::String(s)) => Ok(Value::String(s.to_uppercase())),
                    _ => Err("upper() requires a string".to_string()),
                }
            },
            "lower" => {
                match args.first() {
                    Some(Value::String(s)) => Ok(Value::String(s.to_lowercase())),
                    _ => Err("lower() requires a string".to_string()),
                }
            },
            "trim" => {
                match args.first() {
                    Some(Value::String(s)) => Ok(Value::String(s.trim().to_string())),
                    _ => Err("trim() requires a string".to_string()),
                }
            },
            "split" => {
                match (&args[0], &args[1]) {
                    (Value::String(s), Value::String(delim)) => {
                        let parts: Vec<Value> = s.split(delim.as_str())
                            .map(|p| Value::String(p.to_string()))
                            .collect();
                        Ok(Value::List(parts))
                    },
                    _ => Err("split() requires two strings".to_string()),
                }
            },
            "join" => {
                match (&args[0], &args[1]) {
                    (Value::List(items), Value::String(sep)) => {
                        let strings: Result<Vec<String>, String> = items.iter()
                            .map(|v| match v {
                                Value::String(s) => Ok(s.clone()),
                                _ => Err("join() requires a list of strings".to_string()),
                            })
                            .collect();
                        Ok(Value::String(strings?.join(sep)))
                    },
                    _ => Err("join() requires a list and a string".to_string()),
                }
            },
            
            // List functions
            "list_len" => {
                match args.first() {
                    Some(Value::List(l)) => Ok(Value::Int(l.len() as i64)),
                    _ => Err("list.len() requires a list".to_string()),
                }
            },
            
            // Type conversion functions
            "type" => {
                if let Some(val) = args.first() {
                    Ok(Value::String(val.type_name().to_string()))
                } else {
                    Ok(Value::String("null".to_string()))
                }
            },
            "str" => {
                if let Some(val) = args.first() {
                    Ok(Value::String(format!("{}", val)))
                } else {
                    Ok(Value::String("null".to_string()))
                }
            },
            "int" => {
                match args.first() {
                    Some(Value::Int(n)) => Ok(Value::Int(*n)),
                    Some(Value::Float(f)) => Ok(Value::Int(*f as i64)),
                    Some(Value::String(s)) => {
                        s.parse::<i64>()
                            .map(Value::Int)
                            .map_err(|_| format!("Cannot convert '{}' to int", s))
                    },
                    Some(Value::Bool(b)) => Ok(Value::Int(if *b { 1 } else { 0 })),
                    _ => Err("Cannot convert to int".to_string()),
                }
            },
            "float" => {
                match args.first() {
                    Some(Value::Float(f)) => Ok(Value::Float(*f)),
                    Some(Value::Int(n)) => Ok(Value::Float(*n as f64)),
                    Some(Value::String(s)) => {
                        s.parse::<f64>()
                            .map(Value::Float)
                            .map_err(|_| format!("Cannot convert '{}' to float", s))
                    },
                    _ => Err("Cannot convert to float".to_string()),
                }
            },
            "bool" => {
                if let Some(val) = args.first() {
                    Ok(Value::Bool(val.is_truthy()))
                } else {
                    Ok(Value::Bool(false))
                }
            },
            
            // Environment variable functions
            "env_get" => {
                match args.first() {
                    Some(Value::String(name)) => {
                        match std::env::var(name) {
                            Ok(val) => Ok(Value::String(val)),
                            Err(_) => Ok(Value::String(String::new())),
                        }
                    },
                    _ => Err("env.get() requires a string".to_string()),
                }
            },
            "env_set" => {
                match (&args[0], &args[1]) {
                    (Value::String(name), Value::String(value)) => {
                        std::env::set_var(name, value);
                        Ok(Value::Null)
                    },
                    _ => Err("env.set() requires two strings".to_string()),
                }
            },
            "env_has" => {
                match args.first() {
                    Some(Value::String(name)) => {
                        Ok(Value::Bool(std::env::var(name).is_ok()))
                    },
                    _ => Err("env.has() requires a string".to_string()),
                }
            },
            
            // File system functions
            "fs_read" => {
                match args.first() {
                    Some(Value::String(path)) => {
                        match std::fs::read_to_string(path) {
                            Ok(content) => Ok(Value::String(content)),
                            Err(e) => Err(format!("Failed to read file: {}", e)),
                        }
                    },
                    _ => Err("fs.read() requires a string path".to_string()),
                }
            },
            "fs_write" => {
                match (&args[0], &args[1]) {
                    (Value::String(path), Value::String(content)) => {
                        match std::fs::write(path, content) {
                            Ok(_) => Ok(Value::Null),
                            Err(e) => Err(format!("Failed to write file: {}", e)),
                        }
                    },
                    _ => Err("fs.write() requires path and content strings".to_string()),
                }
            },
            "fs_exists" => {
                match args.first() {
                    Some(Value::String(path)) => {
                        Ok(Value::Bool(std::path::Path::new(path).exists()))
                    },
                    _ => Err("fs.exists() requires a string path".to_string()),
                }
            },
            "fs_delete" => {
                match args.first() {
                    Some(Value::String(path)) => {
                        match std::fs::remove_file(path) {
                            Ok(_) => Ok(Value::Null),
                            Err(e) => Err(format!("Failed to delete file: {}", e)),
                        }
                    },
                    _ => Err("fs.delete() requires a string path".to_string()),
                }
            },
            
            // Enhanced string functions
            "str_contains" => {
                match (&args[0], &args[1]) {
                    (Value::String(haystack), Value::String(needle)) => {
                        Ok(Value::Bool(haystack.contains(needle.as_str())))
                    },
                    _ => Err("str.contains() requires two strings".to_string()),
                }
            },
            "str_starts_with" => {
                match (&args[0], &args[1]) {
                    (Value::String(s), Value::String(prefix)) => {
                        Ok(Value::Bool(s.starts_with(prefix.as_str())))
                    },
                    _ => Err("str.starts_with() requires two strings".to_string()),
                }
            },
            "str_ends_with" => {
                match (&args[0], &args[1]) {
                    (Value::String(s), Value::String(suffix)) => {
                        Ok(Value::Bool(s.ends_with(suffix.as_str())))
                    },
                    _ => Err("str.ends_with() requires two strings".to_string()),
                }
            },
            "str_replace" => {
                match (&args[0], &args[1], &args[2]) {
                    (Value::String(s), Value::String(from), Value::String(to)) => {
                        Ok(Value::String(s.replace(from.as_str(), to.as_str())))
                    },
                    _ => Err("str.replace() requires three strings".to_string()),
                }
            },
            
            // Enhanced list functions
            "list_append" => {
                match (&args[0], &args[1]) {
                    (Value::List(list), item) => {
                        let mut new_list = list.clone();
                        new_list.push(item.clone());
                        Ok(Value::List(new_list))
                    },
                    _ => Err("list.append() requires a list and an item".to_string()),
                }
            },
            "list_contains" => {
                match (&args[0], &args[1]) {
                    (Value::List(list), item) => {
                        for list_item in list {
                            if format!("{}", list_item) == format!("{}", item) {
                                return Ok(Value::Bool(true));
                            }
                        }
                        Ok(Value::Bool(false))
                    },
                    _ => Err("list.contains() requires a list and an item".to_string()),
                }
            },
            
            _ => Err(format!("Unknown native function: {}", name)),
        }
    }
    
    fn is_truthy(&self, val: &Value) -> bool {
        val.is_truthy()
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
