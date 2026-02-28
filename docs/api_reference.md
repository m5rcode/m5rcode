# m5rcode API Reference

Complete reference for the m5rcode standard library.

## std.io - Input/Output

### Functions

#### `println(value: any) -> void`
Print value to stdout with newline.

```m5
io.println("Hello, World!")
io.println(42)
```

#### `print(value: any) -> void`
Print value to stdout without newline.

```m5
io.print("Loading")
io.print(".")
io.print(".")
```

#### `readln() -> string`
Read line from stdin.

```m5
name = io.readln()
```

#### `input(prompt: string) -> string`
Read input with prompt.

```m5
name = io.input("Enter your name: ")
```

## std.collections - Data Structures

### List<T>

Dynamic array.

```m5
list = List.new()
list.push(1)
list.push(2)
item = list.pop()
length = list.len()
```

### Dict<K, V>

Hash map.

```m5
map = Dict.new()
map.set("key", "value")
value = map.get("key")
```

### Set<T>

Unique elements collection.

```m5
set = Set.new()
set.add(1)
set.add(2)
has = set.contains(1)
```

## std.math - Mathematics

### Constants

- `PI: float` - π (3.14159...)
- `E: float` - e (2.71828...)
- `TAU: float` - τ (6.28318...)

### Functions

```m5
import std.math

# Basic
abs_val = math.abs(-5)        # 5
square_root = math.sqrt(16)   # 4.0
power = math.pow(2, 3)        # 8.0

# Trigonometry
sine = math.sin(math.PI / 2)  # 1.0
cosine = math.cos(0)          # 1.0
tangent = math.tan(math.PI / 4)  # 1.0

# Rounding
floor_val = math.floor(3.7)   # 3.0
ceil_val = math.ceil(3.2)     # 4.0
round_val = math.round(3.5)   # 4.0
```

## std.string - String Operations

```m5
import std.string

# Length
len = string.len("hello")  # 5

# Case conversion
upper = string.upper("hello")  # "HELLO"
lower = string.lower("HELLO")  # "hello"

# Trimming
trimmed = string.trim("  hello  ")  # "hello"

# Splitting
words = string.split("a,b,c", ",")  # ["a", "b", "c"]

# Joining
joined = string.join(["a", "b"], ",")  # "a,b"

# Searching
has = string.contains("hello", "ell")  # true
starts = string.starts_with("hello", "he")  # true
ends = string.ends_with("hello", "lo")  # true
```

## std.fs - File System

```m5
import std.fs

# Read file
content = fs.read_file("data.txt")
match content {
    Ok(text) => io.println(text),
    Err(e) => io.println("Error: " + e.message)
}

# Write file
fs.write_file("output.txt", "content")

# Check existence
if fs.exists("file.txt") {
    io.println("File exists")
}

# Directory operations
fs.create_dir("mydir")
files = fs.list_dir(".")
```

## std.net - Networking

### HTTP Client

```m5
import std.net

client = HttpClient.new()
response = client.get("https://api.example.com")

match response {
    Ok(resp) => {
        io.println("Status: " + resp.status)
        io.println("Body: " + resp.body)
    },
    Err(e) => io.println("Request failed")
}
```

### TCP Sockets

```m5
# Server
listener = TcpListener.bind("127.0.0.1:8080")
match listener {
    Ok(server) => {
        stream = server.accept()
        # Handle connection
    },
    Err(e) => io.println("Bind failed")
}

# Client
stream = TcpStream.connect("127.0.0.1:8080")
```

## std.async - Asynchronous Programming

```m5
import std.async

async fn task1() {
    await async.sleep(1.0)
    return "Task 1 done"
}

async fn task2() {
    await async.sleep(0.5)
    return "Task 2 done"
}

async fn main() {
    # Run in parallel
    results = await async.join_all([task1(), task2()])
    
    # Race
    first = await async.race([task1(), task2()])
}
```

## std.test - Testing

```m5
import std.test

test("addition works", || {
    result = 1 + 1
    test.assert_eq(result, 2, "1 + 1 should equal 2")
})

test("list operations", || {
    list = [1, 2, 3]
    test.assert_eq(list.len(), 3, "list should have 3 elements")
})

# Run all tests
sys.exit(test.run_tests())
```

## Built-in Functions

### `type_of(value: any) -> string`
Get type name.

```m5
type_of(42)        # "int"
type_of("hello")   # "string"
type_of([1, 2])    # "List"
```

### `print(value: any) -> void`
Print without newline (alias for io.print).

### `range(n: int) -> Iterator<int>`
Create range iterator.

```m5
for i in range(10) {
    io.println(i)  # 0 to 9
}
```

## Error Types

### `Error`
Base error type.

```m5
class Error {
    message: string
    cause: Error?
}
```

### `ValueError`
Value-related errors.

### `TypeError`
Type-related errors.

### `IOError`
I/O operation errors.

### `NetworkError`
Network operation errors.

## Result Type

```m5
# Result<T, E> represents success or failure
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

## Optional Type

```m5
# T? represents optional value
fn find(list: List<int>, target: int) -> int? {
    for i in range(list.len()) {
        if list[i] == target {
            return i
        }
    }
    return null
}

index = find([1, 2, 3], 2)
if index != null {
    io.println("Found at index: " + index)
}
```

## See Also

- [Tutorial](tutorial.md) - Getting started guide
- [Examples](examples.md) - Code examples
- [Language Spec](../SPEC/language_overview.md) - Language specification
