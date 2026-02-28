/// m5rcode runtime library
pub mod gc;

use std::collections::HashMap;

/// Runtime value representation
#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Null,
    Object(gc::GcPtr),
}

/// Object types for GC
#[derive(Debug, Clone)]
pub enum Object {
    List(Vec<Value>),
    Dict(HashMap<String, Value>),
    Function {
        params: Vec<String>,
        body: Vec<u8>, // Bytecode
    },
}
