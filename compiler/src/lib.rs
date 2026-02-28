/// m5rcode compiler library - Modular architecture
pub mod lexer;
pub mod ast;
pub mod parser;
pub mod interpreter;

// Re-export main types for convenience
pub use lexer::{Lexer, Token, TokenType};
pub use parser::{Parser, Expr, Stmt};
pub use interpreter::{Interpreter, Value};
