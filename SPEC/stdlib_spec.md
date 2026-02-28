# m5rcode Standard Library Specification

## Module Organization

```
std/
├── io          # Input/output operations
├── collections # Data structures
├── math        # Mathematical functions
├── string      # String manipulation
├── fs          # File system operations
├── net         # Networking
├── async       # Asynchronous programming
├── ffi         # Foreign function interface
├── sys         # System operations
└── test        # Testing framework
```

## std.io

### Functions

```m5
fn print(value: any) -> void
# Print value without newline

fn println(value: any) -> void
# Print value with newline

fn readln() -> string
# Read line from stdin

fn eprint(value: any) -> void
# Print to stderr

fn eprintln(value: any) -> void
# Print to stderr with newline

fn input(prompt: string) -> string
# Read input with prompt
```

### Example

```m5
import std.io

io.println("Hello, World!")
name = io.input("Enter name: ")
io.println("Hello, " + name)
```

## std.collections

### List<T>

```m5
class List<T> {
    fn new() -> List<T>
    fn push(item: T) -> void
    fn pop() -> T?
    fn get(index: int) -> T?
    fn set(index: int, value: T) -> void
    fn len() -> int
    fn is_empty() -> bool
    fn clear() -> void
    fn contains(item: T) -> bool
    fn remove(index: int) -> T?
    fn insert(index: int, item: T) -> void
    fn reverse() -> void
    fn sort() -> void  # Requires T: Ord
    fn map<U>(fn(T) -> U) -> List<U>
    fn filter(fn(T) -> bool) -> List<T>
    fn reduce<U>(fn(U, T) -> U, initial: U) -> U
}
```

### Dict<K, V>

```m5
class Dict<K, V> {
    fn new() -> Dict<K, V>
    fn get(key: K) -> V?
    fn set(key: K, value: V) -> void
    fn remove(key: K) -> V?
    fn contains(key: K) -> bool
    fn len() -> int
    fn is_empty() -> bool
    fn clear() -> void
    fn keys() -> List<K>
    fn values() -> List<V>
    fn items() -> List<(K, V)>
}
```

### Set<T>

```m5
class Set<T> {
    fn new() -> Set<T>
    fn add(item: T) -> void
    fn remove(item: T) -> bool
    fn contains(item: T) -> bool
    fn len() -> int
    fn is_empty() -> bool
    fn clear() -> void
    fn union(other: Set<T>) -> Set<T>
    fn intersection(other: Set<T>) -> Set<T>
    fn difference(other: Set<T>) -> Set<T>
}
```

### Example

```m5
import std.collections

list = List.new()
list.push(1)
list.push(2)
list.push(3)

doubled = list.map(|x| x * 2)  # [2, 4, 6]

map = Dict.new()
map.set("name", "Alice")
map.set("age", 30)
```

## std.math

### Constants

```m5
const PI: float = 3.141592653589793
const E: float = 2.718281828459045
const TAU: float = 6.283185307179586
```

### Functions

```m5
fn abs(x: float) -> float
fn sqrt(x: float) -> float
fn pow(base: float, exp: float) -> float
fn exp(x: float) -> float
fn log(x: float) -> float
fn log10(x: float) -> float
fn sin(x: float) -> float
fn cos(x: float) -> float
fn tan(x: float) -> float
fn asin(x: float) -> float
fn acos(x: float) -> float
fn atan(x: float) -> float
fn atan2(y: float, x: float) -> float
fn floor(x: float) -> float
fn ceil(x: float) -> float
fn round(x: float) -> float
fn min(a: float, b: float) -> float
fn max(a: float, b: float) -> float
```

### Example

```m5
import std.math

distance = math.sqrt(3*3 + 4*4)  # 5.0
angle = math.atan2(1.0, 1.0)     # π/4
```

## std.string

### Functions

```m5
fn len(s: string) -> int
fn upper(s: string) -> string
fn lower(s: string) -> string
fn trim(s: string) -> string
fn split(s: string, sep: string) -> List<string>
fn join(parts: List<string>, sep: string) -> string
fn replace(s: string, old: string, new: string) -> string
fn starts_with(s: string, prefix: string) -> bool
fn ends_with(s: string, suffix: string) -> bool
fn contains(s: string, substr: string) -> bool
fn substring(s: string, start: int, end: int) -> string
fn char_at(s: string, index: int) -> string
fn repeat(s: string, count: int) -> string
fn reverse(s: string) -> string
```

### Example

```m5
import std.string

text = "  Hello, World!  "
clean = string.trim(text)
words = string.split(clean, " ")
upper = string.upper(clean)
```

## std.fs

### Functions

```m5
fn read_file(path: string) -> Result<string, Error>
fn write_file(path: string, content: string) -> Result<void, Error>
fn append_file(path: string, content: string) -> Result<void, Error>
fn exists(path: string) -> bool
fn is_file(path: string) -> bool
fn is_dir(path: string) -> bool
fn create_dir(path: string) -> Result<void, Error>
fn remove_file(path: string) -> Result<void, Error>
fn remove_dir(path: string) -> Result<void, Error>
fn list_dir(path: string) -> Result<List<string>, Error>
fn copy_file(src: string, dst: string) -> Result<void, Error>
fn move_file(src: string, dst: string) -> Result<void, Error>
```

### Example

```m5
import std.fs

result = fs.read_file("data.txt")
match result {
    Ok(content) => io.println(content),
    Err(e) => io.eprintln("Error: " + e.message)
}

fs.write_file("output.txt", "Hello, World!")
```

## std.net

### HTTP Client

```m5
class HttpClient {
    fn new() -> HttpClient
    fn get(url: string) -> Result<Response, Error>
    fn post(url: string, body: string) -> Result<Response, Error>
    fn put(url: string, body: string) -> Result<Response, Error>
    fn delete(url: string) -> Result<Response, Error>
}

class Response {
    status: int
    headers: Dict<string, string>
    body: string
}
```

### TCP Sockets

```m5
class TcpListener {
    fn bind(addr: string) -> Result<TcpListener, Error>
    fn accept() -> Result<TcpStream, Error>
}

class TcpStream {
    fn connect(addr: string) -> Result<TcpStream, Error>
    fn read(buffer: Buffer) -> Result<int, Error>
    fn write(data: Buffer) -> Result<int, Error>
    fn close() -> void
}
```

### Example

```m5
import std.net

client = HttpClient.new()
response = client.get("https://api.example.com/data")
match response {
    Ok(resp) => io.println(resp.body),
    Err(e) => io.eprintln("Request failed")
}
```

## std.async

### Functions

```m5
fn spawn<T>(fn() -> T) -> Task<T>
# Spawn async task

fn sleep(duration: float) -> void
# Sleep for duration seconds

fn timeout<T>(duration: float, task: Task<T>) -> Result<T, TimeoutError>
# Wait for task with timeout

fn join_all<T>(tasks: List<Task<T>>) -> List<T>
# Wait for all tasks to complete

fn race<T>(tasks: List<Task<T>>) -> T
# Return first completed task result
```

### Example

```m5
import std.async

async fn fetch_data(url: string) -> string {
    await async.sleep(1.0)
    return "data from " + url
}

async fn main() {
    tasks = [
        fetch_data("url1"),
        fetch_data("url2"),
        fetch_data("url3")
    ]
    results = await async.join_all(tasks)
    for result in results {
        io.println(result)
    }
}
```

## std.ffi

### Functions

```m5
fn load_library(path: string) -> Result<Library, Error>
# Load dynamic library

class Library {
    fn get_symbol(name: string) -> Result<Function, Error>
    fn close() -> void
}

fn c_string(s: string) -> *char
# Convert m5rcode string to C string

fn from_c_string(ptr: *char) -> string
# Convert C string to m5rcode string
```

### Example

```m5
import std.ffi

lib = ffi.load_library("libm.so.6")
match lib {
    Ok(library) => {
        sqrt_fn = library.get_symbol("sqrt")
        # Call C function
    },
    Err(e) => io.eprintln("Failed to load library")
}
```

## std.sys

### Functions

```m5
fn args() -> List<string>
# Command-line arguments

fn env(key: string) -> string?
# Get environment variable

fn set_env(key: string, value: string) -> void
# Set environment variable

fn exit(code: int) -> void
# Exit program

fn exec(command: string, args: List<string>) -> Result<Output, Error>
# Execute command

class Output {
    status: int
    stdout: string
    stderr: string
}
```

### Example

```m5
import std.sys

args = sys.args()
io.println("Arguments: " + args)

home = sys.env("HOME")
match home {
    Some(path) => io.println("Home: " + path),
    None => io.println("HOME not set")
}
```

## std.test

### Functions

```m5
fn assert(condition: bool, message: string) -> void
# Assert condition is true

fn assert_eq<T>(actual: T, expected: T, message: string) -> void
# Assert equality

fn assert_ne<T>(actual: T, expected: T, message: string) -> void
# Assert inequality

fn test(name: string, fn() -> void) -> void
# Define test case

fn run_tests() -> int
# Run all tests, return exit code
```

### Example

```m5
import std.test

test("addition works", || {
    result = 1 + 1
    test.assert_eq(result, 2, "1 + 1 should equal 2")
})

test("list operations", || {
    list = [1, 2, 3]
    test.assert_eq(list.len(), 3, "list should have 3 elements")
    list.push(4)
    test.assert_eq(list.len(), 4, "list should have 4 elements")
})

# Run tests
sys.exit(test.run_tests())
```

## Built-in Traits

### Ord

```m5
trait Ord {
    fn compare(other: Self) -> int
    # Returns: -1 if less, 0 if equal, 1 if greater
}
```

### Add, Sub, Mul, Div

```m5
trait Add {
    fn add(other: Self) -> Self
}

trait Sub {
    fn sub(other: Self) -> Self
}

trait Mul {
    fn mul(other: Self) -> Self
}

trait Div {
    fn div(other: Self) -> Self
}
```

### Display

```m5
trait Display {
    fn to_string() -> string
}
```

### Iterator

```m5
trait Iterator<T> {
    fn next() -> T?
    fn has_next() -> bool
}
```

## Type Utilities

```m5
fn type_of(value: any) -> string
# Get type name

fn is_type<T>(value: any) -> bool
# Check if value is of type T

fn cast<T>(value: any) -> T?
# Safe cast to type T
```

## Error Types

```m5
class Error {
    message: string
    cause: Error?
}

class ValueError : Error { }
class TypeError : Error { }
class IOError : Error { }
class NetworkError : Error { }
class TimeoutError : Error { }
```
