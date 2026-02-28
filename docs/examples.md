# m5rcode Examples

Collection of example programs demonstrating m5rcode features.

## Hello World

```m5
import std.io

fn main() {
    io.println("Hello, m5rcode!")
}

main()
```

## Fibonacci Sequence

```m5
import std.io

fn fibonacci(n: int) -> int {
    if n <= 1 {
        return n
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}

fn main() {
    for i in range(10) {
        io.println(fibonacci(i))
    }
}

main()
```

## Factorial

```m5
import std.io

fn factorial(n: int) -> int {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}

fn main() {
    result = factorial(5)
    io.println("5! = " + result)  # 120
}

main()
```

## FizzBuzz

```m5
import std.io

fn fizzbuzz(n: int) {
    for i in range(1, n + 1) {
        if i % 15 == 0 {
            io.println("FizzBuzz")
        } else if i % 3 == 0 {
            io.println("Fizz")
        } else if i % 5 == 0 {
            io.println("Buzz")
        } else {
            io.println(i)
        }
    }
}

fn main() {
    fizzbuzz(20)
}

main()
```

## Class Example

```m5
import std.io

class Rectangle {
    width: float
    height: float
    
    fn new(width: float, height: float) -> Rectangle {
        return Rectangle { width: width, height: height }
    }
    
    fn area() -> float {
        return self.width * self.height
    }
    
    fn perimeter() -> float {
        return 2 * (self.width + self.height)
    }
}

fn main() {
    rect = Rectangle.new(5.0, 3.0)
    io.println("Area: " + rect.area())
    io.println("Perimeter: " + rect.perimeter())
}

main()
```

## File Processing

```m5
import std.io
import std.fs
import std.string

fn count_words(filename: string) -> int {
    content = fs.read_file(filename)
    match content {
        Ok(text) => {
            words = string.split(text, " ")
            return words.len()
        },
        Err(e) => {
            io.println("Error reading file: " + e.message)
            return 0
        }
    }
}

fn main() {
    count = count_words("data.txt")
    io.println("Word count: " + count)
}

main()
```

## HTTP Client

```m5
import std.io
import std.net

async fn fetch_data(url: string) {
    client = HttpClient.new()
    response = await client.get(url)
    
    match response {
        Ok(resp) => {
            io.println("Status: " + resp.status)
            io.println("Body: " + resp.body)
        },
        Err(e) => {
            io.println("Request failed")
        }
    }
}

async fn main() {
    await fetch_data("https://api.github.com")
}

main()
```

## Simple Web Server

```m5
import std.io
import std.net

fn handle_request(request) {
    io.println("Request: " + request.method + " " + request.path)
    
    if request.path == "/" {
        return {
            status: 200,
            headers: {"Content-Type": "text/html"},
            body: "<h1>Hello from m5rcode!</h1>"
        }
    } else {
        return {
            status: 404,
            body: "Not Found"
        }
    }
}

fn main() {
    server = HttpServer.new("127.0.0.1:8080")
    io.println("Server running on http://127.0.0.1:8080")
    
    server.handle(handle_request)
    server.run()
}

main()
```

## Sorting Algorithms

```m5
import std.io

fn bubble_sort(arr: List<int>) -> List<int> {
    n = arr.len()
    for i in range(n) {
        for j in range(0, n - i - 1) {
            if arr[j] > arr[j + 1] {
                # Swap
                temp = arr[j]
                arr[j] = arr[j + 1]
                arr[j + 1] = temp
            }
        }
    }
    return arr
}

fn main() {
    numbers = [64, 34, 25, 12, 22, 11, 90]
    io.println("Original: " + numbers)
    
    sorted = bubble_sort(numbers)
    io.println("Sorted: " + sorted)
}

main()
```

## Pattern Matching

```m5
import std.io

fn describe(value) {
    match value {
        0 => io.println("Zero"),
        1 => io.println("One"),
        2 => io.println("Two"),
        _ => io.println("Many")
    }
}

fn main() {
    describe(0)  # "Zero"
    describe(1)  # "One"
    describe(5)  # "Many"
}

main()
```

## Closures

```m5
import std.io

fn make_counter() {
    count = 0
    return || {
        count = count + 1
        return count
    }
}

fn main() {
    counter = make_counter()
    io.println(counter())  # 1
    io.println(counter())  # 2
    io.println(counter())  # 3
}

main()
```

## Generic Function

```m5
import std.io

fn swap<T>(a: T, b: T) -> (T, T) {
    return (b, a)
}

fn main() {
    x, y = swap(1, 2)
    io.println("x=" + x + ", y=" + y)  # x=2, y=1
    
    a, b = swap("hello", "world")
    io.println("a=" + a + ", b=" + b)  # a=world, b=hello
}

main()
```

## Error Handling

```m5
import std.io

fn divide(a: int, b: int) -> Result<int, string> {
    if b == 0 {
        return Err("Division by zero")
    }
    return Ok(a / b)
}

fn main() {
    result1 = divide(10, 2)
    match result1 {
        Ok(value) => io.println("Result: " + value),
        Err(msg) => io.println("Error: " + msg)
    }
    
    result2 = divide(10, 0)
    match result2 {
        Ok(value) => io.println("Result: " + value),
        Err(msg) => io.println("Error: " + msg)
    }
}

main()
```

## Async Parallel Tasks

```m5
import std.io
import std.async

async fn task(name: string, duration: float) {
    io.println(name + " starting...")
    await async.sleep(duration)
    io.println(name + " done!")
    return name
}

async fn main() {
    tasks = [
        task("Task 1", 2.0),
        task("Task 2", 1.0),
        task("Task 3", 1.5)
    ]
    
    results = await async.join_all(tasks)
    io.println("All tasks completed: " + results)
}

main()
```

## Testing Example

```m5
import std.test

fn add(a: int, b: int) -> int {
    return a + b
}

fn multiply(a: int, b: int) -> int {
    return a * b
}

test("addition works", || {
    test.assert_eq(add(2, 3), 5, "2 + 3 should equal 5")
    test.assert_eq(add(-1, 1), 0, "-1 + 1 should equal 0")
})

test("multiplication works", || {
    test.assert_eq(multiply(2, 3), 6, "2 * 3 should equal 6")
    test.assert_eq(multiply(0, 5), 0, "0 * 5 should equal 0")
})

# Run tests
sys.exit(test.run_tests())
```

## More Examples

See the `packages/` directory for additional examples:
- `hello_world.m5` - Basic hello world
- `server_example/server.m5` - HTTP server

## Contributing Examples

Have a cool example? Submit a pull request!

1. Create your example in `packages/`
2. Add documentation comments
3. Test it with `m5repl`
4. Submit PR

See [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines.
