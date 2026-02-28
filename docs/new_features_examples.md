# m5rcode Examples - New Features

## Environment Variables

```m5
import std.io

# Set environment variable
env.set("API_KEY", "secret123")

# Get environment variable
let key = env.get("API_KEY")
io.println("API Key: " + key)

# Check if exists
if env.has("API_KEY") {
    io.println("API key is configured")
}

# Get with fallback
let port = env.get("PORT")
if port == "" {
    let port = "8080"
}
io.println("Port: " + port)
```

## File System Operations

```m5
import std.io

# Write to file
fs.write("config.txt", "server=localhost\nport=8080")

# Read from file
let config = fs.read("config.txt")
io.println("Config:\n" + config)

# Check if file exists
if fs.exists("config.txt") {
    io.println("Config file found")
}

# Delete file
fs.delete("config.txt")
```

## String Operations

```m5
import std.io

let text = "Hello, World!"

# Check contains
if str.contains(text, "World") {
    io.println("Found 'World'")
}

# Check starts/ends with
if str.starts_with(text, "Hello") {
    io.println("Starts with Hello")
}

if str.ends_with(text, "!") {
    io.println("Ends with !")
}

# Replace
let new_text = str.replace(text, "World", "m5rcode")
io.println(new_text)  # Hello, m5rcode!

# Combine operations
let cleaned = str.trim("  hello  ")
let upper = str.upper(cleaned)
io.println(upper)  # HELLO
```

## List Operations

```m5
import std.io

let numbers = [1, 2, 3]

# Append item
let numbers = list.append(numbers, 4)
io.println(toStr(list.len(numbers)))  # 4

# Check contains
if list.contains(numbers, 3) {
    io.println("List contains 3")
}

# Iterate
for num in numbers {
    io.println("Number: " + toStr(num))
}
```

## Discord Bot with Environment Variables

```m5
import std.io

# Get command data from environment
let cmd = env.get("DISCORD_COMMAND")
let args = env.get("DISCORD_ARGS")
let author = env.get("DISCORD_AUTHOR")

# Fallback for testing
if cmd == "" {
    let cmd = "help"
}

fn cmd_kick() {
    if args == "" {
        io.println("Usage: !kick @user reason")
    } else {
        io.println("[API:KICK]" + args + "|Kicked by " + author)
    }
}

fn cmd_help() {
    io.println("Commands: !kick, !ban, !mute")
}

# Route command
if cmd == "kick" {
    cmd_kick()
} else if cmd == "help" {
    cmd_help()
}
```

## Configuration File Parser

```m5
import std.io

fn parse_config(path) {
    if fs.exists(path) {
        let content = fs.read(path)
        let lines = str.split(content, "\n")
        
        for line in lines {
            if str.contains(line, "=") {
                let parts = str.split(line, "=")
                # Process config...
            }
        }
    }
}

parse_config("app.conf")
```

## Simple Web Server Config

```m5
import std.io

# Read port from environment or use default
let port = env.get("PORT")
if port == "" {
    let port = "8080"
}

# Read host from environment
let host = env.get("HOST")
if host == "" {
    let host = "0.0.0.0"
}

io.println("Server starting on " + host + ":" + port)

# Write PID file
fs.write("server.pid", "12345")

# Check if config exists
if fs.exists("server.conf") {
    let config = fs.read("server.conf")
    io.println("Loaded config")
} else {
    io.println("No config found, using defaults")
}
```

## Log File Manager

```m5
import std.io

fn log(message) {
    let log_file = "app.log"
    let existing = ""
    
    if fs.exists(log_file) {
        let existing = fs.read(log_file)
    }
    
    let new_content = existing + message + "\n"
    fs.write(log_file, new_content)
}

log("Application started")
log("Processing request")
log("Application stopped")

# Read logs
let logs = fs.read("app.log")
io.println("Logs:\n" + logs)
```

## String Validation

```m5
import std.io

fn validate_email(email) {
    if str.contains(email, "@") {
        if str.contains(email, ".") {
            return true
        }
    }
    return false
}

fn validate_url(url) {
    if str.starts_with(url, "http://") {
        return true
    }
    if str.starts_with(url, "https://") {
        return true
    }
    return false
}

let email = "user@example.com"
if validate_email(email) {
    io.println("Valid email")
}

let url = "https://example.com"
if validate_url(url) {
    io.println("Valid URL")
}
```
