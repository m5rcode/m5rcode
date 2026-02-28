# m5rcode Enhancement Plan

## Critical Bugs Fixed ✅
1. ✅ else if parsing (was always taking first branch)
2. ✅ String comparison operators (==, !=, <, >, etc.)

## Missing Core Features to Add

### 1. Environment Variables Support
- `env.get(name)` - Read environment variable
- `env.set(name, value)` - Set environment variable
- `env.has(name)` - Check if exists

### 2. File System Operations
- `fs.read(path)` - Read file
- `fs.write(path, content)` - Write file
- `fs.exists(path)` - Check if exists
- `fs.delete(path)` - Delete file
- `fs.list(path)` - List directory

### 3. Better String Operations
- `str.contains(haystack, needle)`
- `str.starts_with(str, prefix)`
- `str.ends_with(str, suffix)`
- `str.replace(str, from, to)`
- `str.substring(str, start, end)`

### 4. List Operations
- `list.append(list, item)`
- `list.remove(list, index)`
- `list.contains(list, item)`
- `list.sort(list)`
- `list.reverse(list)`

### 5. JSON Support
- `json.parse(str)` - Parse JSON string
- `json.stringify(obj)` - Convert to JSON

### 6. HTTP Client (Basic)
- `http.get(url)` - GET request
- `http.post(url, data)` - POST request

### 7. Better Error Handling
- Try/catch blocks
- Error types
- Stack traces

### 8. Module System
- Proper import resolution
- Module caching
- Relative imports

## Implementation Priority
1. Environment variables (CRITICAL for Discord bot)
2. Better string operations
3. File system operations
4. List operations
5. JSON support
6. Module system improvements
