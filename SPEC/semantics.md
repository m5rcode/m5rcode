# m5rcode Semantics Specification

## Execution Model

### Program Execution

1. Parse source files into AST
2. Resolve imports and build module graph
3. Type check (if static types present)
4. Generate IR (intermediate representation)
5. Execute via interpreter or compile to native code

### Evaluation Order

- Expressions evaluated left-to-right
- Function arguments evaluated before call
- Short-circuit evaluation for `&&` and `||`

## Variables and Scoping

### Variable Declaration

```m5
# Mutable by default
x = 42
x = 100  # OK

# Explicit immutable
const y = 42
y = 100  # ERROR

# Explicit mutable
var z = 42
z = 100  # OK
```

### Scope Rules

```m5
x = 10  # Global scope

fn foo() {
    y = 20  # Local scope
    if true {
        z = 30  # Block scope
    }
    # z not accessible here
}
# y not accessible here
```

### Shadowing

```m5
x = 10
fn foo() {
    x = 20  # Shadows global x
    io.println(x)  # Prints 20
}
io.println(x)  # Prints 10
```

## Functions

### Function Calls

```m5
fn add(a, b) {
    return a + b
}

result = add(1, 2)  # 3
```

### Default Arguments

```m5
fn greet(name, greeting = "Hello") {
    return greeting + ", " + name
}

greet("Alice")           # "Hello, Alice"
greet("Bob", "Hi")       # "Hi, Bob"
```

### Closures

```m5
fn make_adder(x) {
    return |y| x + y
}

add5 = make_adder(5)
result = add5(10)  # 15
```

### Recursion

```m5
fn factorial(n) {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}
```

## Control Flow

### If/Else

```m5
if x > 0 {
    io.println("positive")
} else if x < 0 {
    io.println("negative")
} else {
    io.println("zero")
}

# Expression form
result = if x > 0 { "pos" } else { "neg" }
```

### While Loop

```m5
i = 0
while i < 10 {
    io.println(i)
    i = i + 1
}
```

### For Loop

```m5
# Iterate over collection
for item in [1, 2, 3] {
    io.println(item)
}

# Range iteration
for i in range(10) {
    io.println(i)
}
```

### Match Expression

```m5
result = match x {
    0 => "zero",
    1 => "one",
    2 => "two",
    _ => "many"
}
```

## Classes and Objects

### Class Definition

```m5
class Point {
    x: int
    y: int
    
    fn new(x: int, y: int) -> Point {
        return Point { x: x, y: y }
    }
    
    fn distance() -> float {
        return sqrt(self.x * self.x + self.y * self.y)
    }
}
```

### Object Creation

```m5
p = Point.new(3, 4)
d = p.distance()  # 5.0
```

### Inheritance

```m5
class Animal {
    name: string
    
    fn speak() {
        io.println("...")
    }
}

class Dog : Animal {
    fn speak() {
        io.println("Woof!")
    }
}

dog = Dog { name: "Rex" }
dog.speak()  # "Woof!"
```

## Memory Management

### Garbage Collection

Default memory management:

```m5
# Automatic allocation
obj = MyClass()
list = [1, 2, 3]

# Automatic deallocation when no references
obj = null  # Original object eligible for GC
```

### Reference Counting

- Objects tracked via reference count
- Cycle detection for circular references
- Finalizers called on deallocation

### Ownership Mode

```m5
# Explicit ownership
own buffer = Buffer.new(1024)

# Move semantics
other = move buffer
# buffer no longer valid

# Borrow checking
fn process(ref data: Buffer) {
    # data borrowed, not moved
}
```

## Modules and Imports

### Module Definition

```m5
# File: math.m5
export fn add(a, b) {
    return a + b
}

export const PI = 3.14159
```

### Importing

```m5
# Import entire module
import math
result = math.add(1, 2)

# Import specific items
import math.add, math.PI

# Import with alias
import math as m
result = m.add(1, 2)
```

## Error Handling

### Exceptions

```m5
# Throw exception
if x < 0 {
    throw ValueError("x must be positive")
}

# Catch exception
try {
    result = risky_operation()
} catch ValueError as e {
    io.println("Error: " + e.message)
} finally {
    cleanup()
}
```

### Result Type

```m5
fn divide(a: int, b: int) -> Result<int, string> {
    if b == 0 {
        return Err("division by zero")
    }
    return Ok(a / b)
}

result = divide(10, 2)
match result {
    Ok(value) => io.println(value),
    Err(msg) => io.println("Error: " + msg)
}
```

## Operators

### Arithmetic

```m5
a + b   # Addition
a - b   # Subtraction
a * b   # Multiplication
a / b   # Division
a % b   # Modulo
a ** b  # Exponentiation
```

### Comparison

```m5
a == b  # Equality
a != b  # Inequality
a < b   # Less than
a > b   # Greater than
a <= b  # Less or equal
a >= b  # Greater or equal
```

### Logical

```m5
a && b  # Logical AND (short-circuit)
a || b  # Logical OR (short-circuit)
!a      # Logical NOT
```

### Bitwise

```m5
a & b   # Bitwise AND
a | b   # Bitwise OR
a ^ b   # Bitwise XOR
~a      # Bitwise NOT
a << b  # Left shift
a >> b  # Right shift
```

## Operator Overloading

```m5
class Vector {
    x: float
    y: float
}

impl Add for Vector {
    fn add(other: Vector) -> Vector {
        return Vector { 
            x: self.x + other.x, 
            y: self.y + other.y 
        }
    }
}

v1 = Vector { x: 1.0, y: 2.0 }
v2 = Vector { x: 3.0, y: 4.0 }
v3 = v1 + v2  # Calls v1.add(v2)
```

## Async/Await

### Async Functions

```m5
async fn fetch_data(url: string) -> string {
    response = await http.get(url)
    return response.body
}
```

### Await Expression

```m5
async fn main() {
    data = await fetch_data("https://example.com")
    io.println(data)
}
```

### Concurrent Execution

```m5
async fn parallel_fetch() {
    results = await [
        fetch_data("url1"),
        fetch_data("url2"),
        fetch_data("url3")
    ]
    return results
}
```

## FFI (Foreign Function Interface)

### Calling C Functions

```m5
# Declare external function
extern "C" fn strlen(s: *char) -> int

# Call from m5rcode
length = strlen("hello")
```

### Exporting Functions

```m5
# Export for C
export "C" fn add(a: int, b: int) -> int {
    return a + b
}
```

## Metaprogramming

### Macros

```m5
macro debug(expr) {
    io.println(stringify(expr) + " = " + expr)
}

x = 42
debug(x)  # Prints "x = 42"
```

### Compile-Time Evaluation

```m5
const SIZE = compute_size()  # Evaluated at compile time

fn compute_size() -> int {
    return 1024 * 1024
}
```

## Standard Library Integration

### IO Operations

```m5
import std.io

io.println("Hello")
line = io.readln()
```

### Collections

```m5
import std.collections

list = List.new()
list.push(1)
list.push(2)

map = Dict.new()
map.set("key", "value")
```

### File System

```m5
import std.fs

content = fs.read_file("data.txt")
fs.write_file("output.txt", content)
```

## Undefined Behavior

The following are undefined and may cause crashes:

- Dereferencing null pointers in ownership mode
- Buffer overflows in unsafe code
- Data races in concurrent code
- Use-after-free in ownership mode
- Integer overflow (wraps by default, can enable checks)

## Implementation Notes

- Tail call optimization supported
- String interning for efficiency
- Copy-on-write for collections
- Lazy evaluation for generators
- JIT compilation for hot paths
