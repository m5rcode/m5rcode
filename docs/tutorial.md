# m5rcode Tutorial

Welcome to m5rcode! This 20-minute tutorial will get you started with the language.

## Installation

### Arch Linux

```bash
git clone https://github.com/m5rcode/m5rcode.git
cd m5rcode
./scripts/bootstrap.sh
./scripts/build_arch.sh --install
```

### Other Linux

```bash
git clone https://github.com/m5rcode/m5rcode.git
cd m5rcode
./scripts/bootstrap.sh
cargo build --release
sudo cp target/release/m5r* /usr/local/bin/
```

## Hello World

Create a file `hello.m5`:

```m5
import std.io

fn main() {
    io.println("Hello, m5rcode!")
}

main()
```

Run it:

```bash
m5repl hello.m5
```

## Basic Types

```m5
# Integers
x = 42
y: int = 100

# Floats
pi = 3.14159
e: float = 2.71828

# Strings
name = "Alice"
greeting = "Hello, " + name

# Booleans
is_valid = true
is_ready = false

# Null
value = null
```

## Variables

```m5
# Mutable by default
x = 10
x = 20  # OK

# Explicit immutable
const PI = 3.14159
# PI = 3.14  # ERROR

# Type annotations
age: int = 30
price: float = 19.99
```

## Functions

```m5
# Basic function
fn greet(name) {
    return "Hello, " + name
}

# With types
fn add(a: int, b: int) -> int {
    return a + b
}

# Default arguments
fn power(base, exp = 2) {
    result = 1
    i = 0
    while i < exp {
        result = result * base
        i = i + 1
    }
    return result
}

# Call functions
message = greet("Alice")
sum = add(5, 3)
squared = power(4)      # 16
cubed = power(4, 3)     # 64
```

## Control Flow

### If/Else

```m5
age = 18

if age >= 18 {
    io.println("Adult")
} else {
    io.println("Minor")
}

# Expression form
status = if age >= 18 { "adult" } else { "minor" }
```

### While Loop

```m5
i = 0
while i < 5 {
    io.println(i)
    i = i + 1
}
```

### For Loop

```m5
# Iterate over list
for num in [1, 2, 3, 4, 5] {
    io.println(num)
}

# Range iteration
for i in range(10) {
    io.println(i)
}
```

## Data Structures

### Lists

```m5
import std.collections

# Create list
numbers = [1, 2, 3, 4, 5]

# Access elements
first = numbers[0]
last = numbers[4]

# List methods
list = List.new()
list.push(10)
list.push(20)
length = list.len()
```

### Dictionaries

```m5
# Create dict
person = {
    "name": "Alice",
    "age": 30,
    "city": "NYC"
}

# Access values
name = person["name"]

# Dict methods
map = Dict.new()
map.set("key", "value")
value = map.get("key")
```

## Classes

```m5
class Person {
    name: string
    age: int
    
    fn new(name: string, age: int) -> Person {
        return Person { name: name, age: age }
    }
    
    fn greet() {
        io.println("Hello, I'm " + self.name)
    }
    
    fn birthday() {
        self.age = self.age + 1
    }
}

# Create instance
alice = Person.new("Alice", 30)
alice.greet()
alice.birthday()
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

## Modules

### Creating a Module

File: `math_utils.m5`

```m5
export fn square(x) {
    return x * x
}

export fn cube(x) {
    return x * x * x
}

export const PI = 3.14159
```

### Using a Module

```m5
import math_utils

result = math_utils.square(5)  # 25
volume = math_utils.cube(3)    # 27
```

## Error Handling

```m5
fn divide(a, b) {
    if b == 0 {
        throw ValueError("Division by zero")
    }
    return a / b
}

try {
    result = divide(10, 0)
} catch ValueError as e {
    io.println("Error: " + e.message)
}
```

## File I/O

```m5
import std.fs

# Read file
content = fs.read_file("data.txt")
match content {
    Ok(text) => io.println(text),
    Err(e) => io.println("Error: " + e.message)
}

# Write file
fs.write_file("output.txt", "Hello, World!")
```

## Async/Await

```m5
import std.async
import std.net

async fn fetch_data(url) {
    client = HttpClient.new()
    response = await client.get(url)
    return response.body
}

async fn main() {
    data = await fetch_data("https://api.example.com")
    io.println(data)
}
```

## Interactive REPL

Start the REPL:

```bash
m5repl
```

Try some expressions:

```m5
>>> 1 + 2
3
>>> x = 42
>>> x * 2
84
>>> fn double(n) { return n * 2 }
>>> double(21)
42
```

## Next Steps

- Read the [API Reference](api_reference.md) for standard library details
- Check out [Examples](examples.md) for more code samples
- Read the [Language Specification](../SPEC/language_overview.md) for in-depth details
- Join the community and contribute!

## Getting Help

- Documentation: `/usr/share/doc/m5rcode/`
- Examples: `/usr/share/doc/m5rcode/examples/`
- GitHub: https://github.com/m5rcode/m5rcode
- Issues: https://github.com/m5rcode/m5rcode/issues
