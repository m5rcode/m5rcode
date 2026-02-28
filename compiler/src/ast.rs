/// AST - Abstract Syntax Tree
/// Complete representation of m5rcode programs

use std::fmt;

// ============================================================================
// TYPE SYSTEM
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    // Primitive types
    Int,
    Float,
    String,
    Bool,
    Char,
    Void,
    Null,
    
    // Compound types
    Array(Box<Type>, Option<usize>),  // Type and optional size
    Tuple(Vec<Type>),
    Function(Vec<Type>, Box<Type>),   // params, return
    
    // User-defined types
    Named(String),                     // Class, struct, enum name
    Generic(String, Vec<Type>),        // Generic type with parameters
    
    // Special types
    Optional(Box<Type>),               // T?
    Reference(Box<Type>, bool),        // &T or &mut T
    Owned(Box<Type>),                  // own T
    
    // Type variables for inference
    Var(usize),                        // Type variable for HM inference
    Unknown,                           // Not yet inferred
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::String => write!(f, "string"),
            Type::Bool => write!(f, "bool"),
            Type::Char => write!(f, "char"),
            Type::Void => write!(f, "void"),
            Type::Null => write!(f, "null"),
            Type::Array(t, Some(n)) => write!(f, "[{}; {}]", t, n),
            Type::Array(t, None) => write!(f, "[{}]", t),
            Type::Tuple(types) => {
                write!(f, "(")?;
                for (i, t) in types.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", t)?;
                }
                write!(f, ")")
            },
            Type::Function(params, ret) => {
                write!(f, "fn(")?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", p)?;
                }
                write!(f, ") -> {}", ret)
            },
            Type::Named(name) => write!(f, "{}", name),
            Type::Generic(name, params) => {
                write!(f, "{}<", name)?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", p)?;
                }
                write!(f, ">")
            },
            Type::Optional(t) => write!(f, "{}?", t),
            Type::Reference(t, mutable) => {
                if *mutable {
                    write!(f, "&mut {}", t)
                } else {
                    write!(f, "&{}", t)
                }
            },
            Type::Owned(t) => write!(f, "own {}", t),
            Type::Var(id) => write!(f, "T{}", id),
            Type::Unknown => write!(f, "?"),
        }
    }
}

// ============================================================================
// EXPRESSIONS
// ============================================================================

#[derive(Debug, Clone)]
pub struct Expr {
    pub kind: ExprKind,
    pub typ: Type,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum ExprKind {
    // Literals
    IntLit(i64),
    FloatLit(f64),
    StringLit(String),
    CharLit(char),
    BoolLit(bool),
    NullLit,
    
    // Variables and paths
    Ident(String),
    Path(Vec<String>),  // module::path::to::item
    
    // Operators
    Unary(UnOp, Box<Expr>),
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Assign(Box<Expr>, Box<Expr>),
    AssignOp(BinOp, Box<Expr>, Box<Expr>),
    
    // Function calls and method calls
    Call(Box<Expr>, Vec<Expr>),
    MethodCall(Box<Expr>, String, Vec<Expr>),
    
    // Field access and indexing
    Field(Box<Expr>, String),
    Index(Box<Expr>, Box<Expr>),
    
    // Collections
    Array(Vec<Expr>),
    Tuple(Vec<Expr>),
    Struct(String, Vec<(String, Expr)>),
    
    // Control flow expressions
    If(Box<Expr>, Box<Expr>, Option<Box<Expr>>),
    Match(Box<Expr>, Vec<MatchArm>),
    Block(Vec<Stmt>, Option<Box<Expr>>),
    
    // Closures
    Closure(Vec<Pattern>, Option<Type>, Box<Expr>),
    
    // Type operations
    Cast(Box<Expr>, Type),
    TypeCheck(Box<Expr>, Type),  // x is Type
    
    // Range
    Range(Option<Box<Expr>>, Option<Box<Expr>>, bool),  // start..end or start..=end
    
    // Async
    Async(Box<Expr>),
    Await(Box<Expr>),
    
    // Special
    Return(Option<Box<Expr>>),
    Break(Option<Box<Expr>>),
    Continue,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnOp {
    Neg,      // -
    Not,      // !
    BitNot,   // ~
    Deref,    // *
    Ref,      // &
    RefMut,   // &mut
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOp {
    // Arithmetic
    Add, Sub, Mul, Div, Mod, Pow,
    
    // Comparison
    Eq, Ne, Lt, Gt, Le, Ge,
    
    // Logical
    And, Or,
    
    // Bitwise
    BitAnd, BitOr, BitXor, Shl, Shr,
}

#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<Expr>,
    pub body: Expr,
}

// ============================================================================
// PATTERNS
// ============================================================================

#[derive(Debug, Clone)]
pub struct Pattern {
    pub kind: PatternKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum PatternKind {
    Wildcard,                              // _
    Ident(String, bool),                   // name, is_mut
    Literal(Expr),                         // 42, "hello", etc
    Tuple(Vec<Pattern>),                   // (a, b, c)
    Array(Vec<Pattern>),                   // [a, b, c]
    Struct(String, Vec<(String, Pattern)>), // Point { x, y }
    Enum(String, Vec<Pattern>),            // Some(x)
    Or(Vec<Pattern>),                      // a | b | c
    Range(Expr, Expr, bool),               // 1..10 or 1..=10
}

// ============================================================================
// STATEMENTS
// ============================================================================

#[derive(Debug, Clone)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum StmtKind {
    // Declarations
    Let(Pattern, Option<Type>, Option<Expr>, bool), // pattern, type, init, is_mut
    Const(String, Type, Expr),
    Static(String, Type, Expr, bool),  // name, type, init, is_mut
    
    // Functions
    Function(FunctionDecl),
    
    // Types
    Struct(StructDecl),
    Enum(EnumDecl),
    Trait(TraitDecl),
    Impl(ImplDecl),
    TypeAlias(String, Vec<String>, Type),  // name, generics, type
    
    // Modules
    Import(Vec<String>, Option<String>),  // path, alias
    Export(Vec<String>),
    Module(String, Vec<Stmt>),
    
    // Control flow
    Expr(Expr),
    While(Expr, Box<Stmt>),
    For(Pattern, Expr, Box<Stmt>),
    Loop(Box<Stmt>),
    
    // Empty
    Empty,
}

// ============================================================================
// DECLARATIONS
// ============================================================================

#[derive(Debug, Clone)]
pub struct FunctionDecl {
    pub name: String,
    pub generics: Vec<GenericParam>,
    pub params: Vec<(Pattern, Type)>,
    pub return_type: Type,
    pub body: Option<Expr>,  // None for extern functions
    pub is_async: bool,
    pub is_pub: bool,
    pub is_extern: bool,
}

#[derive(Debug, Clone)]
pub struct StructDecl {
    pub name: String,
    pub generics: Vec<GenericParam>,
    pub fields: Vec<(String, Type, bool)>,  // name, type, is_pub
    pub is_pub: bool,
}

#[derive(Debug, Clone)]
pub struct EnumDecl {
    pub name: String,
    pub generics: Vec<GenericParam>,
    pub variants: Vec<EnumVariant>,
    pub is_pub: bool,
}

#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub name: String,
    pub data: EnumVariantData,
}

#[derive(Debug, Clone)]
pub enum EnumVariantData {
    Unit,                           // Variant
    Tuple(Vec<Type>),              // Variant(T1, T2)
    Struct(Vec<(String, Type)>),   // Variant { field: Type }
}

#[derive(Debug, Clone)]
pub struct TraitDecl {
    pub name: String,
    pub generics: Vec<GenericParam>,
    pub supertraits: Vec<Type>,
    pub items: Vec<TraitItem>,
    pub is_pub: bool,
}

#[derive(Debug, Clone)]
pub enum TraitItem {
    Function(FunctionDecl),
    Type(String, Vec<Type>),  // Associated type with bounds
    Const(String, Type),
}

#[derive(Debug, Clone)]
pub struct ImplDecl {
    pub generics: Vec<GenericParam>,
    pub trait_ref: Option<Type>,  // None for inherent impl
    pub self_type: Type,
    pub items: Vec<ImplItem>,
}

#[derive(Debug, Clone)]
pub enum ImplItem {
    Function(FunctionDecl),
    Type(String, Type),  // Associated type implementation
    Const(String, Type, Expr),
}

#[derive(Debug, Clone)]
pub struct GenericParam {
    pub name: String,
    pub bounds: Vec<Type>,  // Trait bounds
    pub default: Option<Type>,
}

// ============================================================================
// SOURCE LOCATION
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

impl Span {
    pub fn new(start: usize, end: usize, line: usize, column: usize) -> Self {
        Span { start, end, line, column }
    }
    
    pub fn dummy() -> Self {
        Span { start: 0, end: 0, line: 0, column: 0 }
    }
}

// ============================================================================
// PROGRAM
// ============================================================================

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Stmt>,
    pub file: String,
}

impl Program {
    pub fn new(statements: Vec<Stmt>, file: String) -> Self {
        Program { statements, file }
    }
}
