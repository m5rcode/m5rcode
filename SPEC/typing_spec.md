# m5rcode Type System Specification

## Overview

m5rcode implements a **gradual type system** combining:
- Dynamic typing (runtime type checking)
- Static typing (compile-time type checking)
- Hindley-Milner type inference
- Structural subtyping for objects
- Nominal typing for classes

## Type Categories

### Primitive Types

```m5
int      # 64-bit signed integer
float    # 64-bit floating point
bool     # Boolean (true/false)
string   # UTF-8 string
null     # Null type
```

### Compound Types

```m5
List<T>           # Homogeneous list
Dict<K, V>        # Hash map
Tuple<T1, T2...>  # Fixed-size tuple
Array<T, N>       # Fixed-size array
```

### Function Types

```m5
fn(int, int) -> int           # Function taking two ints, returning int
fn(string) -> void            # Function taking string, no return
fn<T>(T) -> T                 # Generic function
```

### User-Defined Types

```m5
class MyClass { ... }         # Nominal class type
trait MyTrait { ... }         # Trait (interface)
```

## Type Inference

### Hindley-Milner Algorithm

m5rcode uses HM type inference with extensions:

```m5
# Inferred as int
x = 42

# Inferred as fn(int, int) -> int
fn add(a, b) {
    return a + b
}

# Inferred as List<int>
numbers = [1, 2, 3]

# Inferred as Dict<string, int>
ages = {"Alice": 30, "Bob": 25}
```

### Type Constraints

```m5
# T must implement Add trait
fn sum<T: Add>(a: T, b: T) -> T {
    return a + b
}

# Multiple constraints
fn compare<T: Ord + Display>(a: T, b: T) {
    if a > b {
        io.println(a)
    }
}
```

## Gradual Typing

### Dynamic Mode

```m5
# No type annotations - fully dynamic
x = 42
x = "hello"  # OK - dynamic typing
x = [1, 2, 3]  # OK
```

### Static Mode

```m5
# Explicit type annotations
x: int = 42
x = "hello"  # ERROR: type mismatch

# Function with types
fn greet(name: string) -> string {
    return "Hello, " + name
}
```

### Mixed Mode

```m5
# Inferred types with gradual checking
fn process(data) {  # data is dynamic
    x: int = data.count()  # x is static
    return x * 2
}
```

## Type Checking Rules

### Assignment

```m5
# Static variable
x: int = 42
x = 100      # OK
x = "hello"  # ERROR

# Dynamic variable
y = 42
y = "hello"  # OK
```

### Function Calls

```m5
fn add(a: int, b: int) -> int {
    return a + b
}

add(1, 2)      # OK
add(1, "2")    # ERROR: type mismatch
add(1.5, 2.5)  # ERROR: float != int
```

### Subtyping

```m5
class Animal { }
class Dog : Animal { }

fn feed(animal: Animal) { }

dog = Dog()
feed(dog)  # OK - Dog is subtype of Animal
```

## Generics

### Generic Functions

```m5
fn identity<T>(x: T) -> T {
    return x
}

identity(42)       # T = int
identity("hello")  # T = string
```

### Generic Classes

```m5
class Box<T> {
    value: T
    
    fn new(value: T) -> Box<T> {
        return Box { value: value }
    }
    
    fn get() -> T {
        return self.value
    }
}

box = Box.new(42)  # Box<int>
```

### Trait Bounds

```m5
trait Comparable {
    fn compare(other: Self) -> int
}

fn max<T: Comparable>(a: T, b: T) -> T {
    return a.compare(b) > 0 ? a : b
}
```

## Traits (Interfaces)

### Definition

```m5
trait Drawable {
    fn draw() -> void
}

trait Resizable {
    fn resize(width: int, height: int) -> void
}
```

### Implementation

```m5
class Rectangle {
    width: int
    height: int
}

impl Drawable for Rectangle {
    fn draw() {
        io.println("Drawing rectangle")
    }
}

impl Resizable for Rectangle {
    fn resize(w: int, h: int) {
        self.width = w
        self.height = h
    }
}
```

### Multiple Traits

```m5
fn render<T: Drawable + Resizable>(obj: T) {
    obj.resize(100, 100)
    obj.draw()
}
```

## Optional Types

```m5
# Optional type (can be null)
x: int? = null
x = 42

# Unwrapping
if x != null {
    io.println(x)
}

# Safe navigation
result = obj?.method()?.field
```

## Ownership Types

```m5
# Owned type (move semantics)
own buffer: Buffer = Buffer.new(1024)

# Move ownership
other = move buffer
# buffer is now invalid

# Borrowed reference
fn process(ref data: Buffer) {
    # data is borrowed, not owned
}
```

## Type Aliases

```m5
type UserId = int
type Callback = fn(string) -> void
type Result<T> = T | Error
```

## Union Types

```m5
# Union type
x: int | string = 42
x = "hello"  # OK

# Type narrowing
if x is int {
    io.println(x + 1)
} else {
    io.println(x.upper())
}
```

## Type Casting

```m5
# Explicit cast
x = 42
y = x as float  # 42.0

# Safe cast (returns optional)
s = "123"
n = s as? int  # Some(123)

s = "abc"
n = s as? int  # null
```

## Type Compatibility

### Coercion Rules

```m5
int -> float     # OK (widening)
float -> int     # ERROR (narrowing, requires cast)
T -> T?          # OK (wrapping)
T? -> T          # ERROR (requires unwrap)
```

### Structural Compatibility

```m5
# Structural typing for objects
obj1 = { x: 10, y: 20 }
obj2 = { x: 5, y: 15, z: 25 }

fn distance(p: { x: int, y: int }) -> float {
    return sqrt(p.x * p.x + p.y * p.y)
}

distance(obj1)  # OK
distance(obj2)  # OK - extra field ignored
```

## Type Errors

### Common Errors

```m5
# Type mismatch
x: int = "hello"  # ERROR: expected int, found string

# Missing trait
fn sort<T: Ord>(list: List<T>) { }
sort([obj1, obj2])  # ERROR: obj type doesn't implement Ord

# Arity mismatch
fn add(a: int, b: int) -> int { }
add(1)  # ERROR: expected 2 arguments, found 1
```

## Type Inference Limitations

```m5
# Ambiguous inference
x = []  # ERROR: cannot infer element type
x: List<int> = []  # OK

# Recursive types require annotation
fn factorial(n) {  # ERROR: cannot infer return type
    return n <= 1 ? 1 : n * factorial(n - 1)
}

fn factorial(n: int) -> int {  # OK
    return n <= 1 ? 1 : n * factorial(n - 1)
}
```

## Best Practices

1. Use type inference for local variables
2. Annotate function signatures
3. Use traits for polymorphism
4. Prefer static typing for public APIs
5. Use dynamic typing for prototyping
6. Add type annotations when inference fails
7. Use optional types instead of null checks
8. Leverage ownership types for performance
