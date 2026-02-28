# m5rcode v0.3.2 - Enhancement Summary

## Critical Bugs Fixed ✅

### 1. else if Statement Bug
**Problem**: Parser didn't handle `else if` - always executed first branch
**Fix**: Modified `parse_if()` to recursively parse `else if` as nested if statement
**Impact**: All conditional logic now works correctly

### 2. String Comparison Operators
**Problem**: Only `+` operator worked for strings
**Fix**: Added `==`, `!=`, `<`, `>`, `<=`, `>=` operators for strings in `eval_binary()`
**Impact**: String comparisons in conditionals now work

## New Features Added ✅

### 1. Environment Variables (std.env)
```m5
env.get(name)      # Get environment variable
env.set(name, val) # Set environment variable  
env.has(name)      # Check if variable exists
```

### 2. File System Operations (std.fs)
```m5
fs.read(path)         # Read file content
fs.write(path, content) # Write to file
fs.exists(path)       # Check if file exists
fs.delete(path)       # Delete file
```

### 3. Enhanced String Operations (std.str)
```m5
str.contains(haystack, needle)  # Check if contains substring
str.starts_with(str, prefix)    # Check if starts with
str.ends_with(str, suffix)      # Check if ends with
str.replace(str, from, to)      # Replace all occurrences
```

### 4. Enhanced List Operations (std.list)
```m5
list.append(list, item)    # Add item to list
list.contains(list, item)  # Check if list contains item
```

## Discord Bot Improvements ✅

### 1. Working API Integration
- User mention parsing (`<@123>` → user ID)
- Role mention parsing (`<@&123>` → role ID)
- Actual Discord API calls (kick, ban, mute, etc.)
- Embed support with colors
- Reaction support

### 2. Enhanced discord.m5 Library
- 20+ API functions
- Clean, simple function names
- Helper functions (parse_duration, color constants)
- Proper error messages

### 3. ModBot Pro
- 12 working commands
- Beautiful embeds
- Reaction confirmations
- Professional responses
- Environment variable support

## Testing

All features tested and working:

```bash
# Test environment variables
m5repl /tmp/test_env.m5

# Test file system
m5repl /tmp/test_fs.m5

# Test Discord bot
cd /home/m5rcel/m5rcode/runtime/discord
./target/release/m5rcode-discord TOKEN modbot_pro.m5
```

## Performance

- Compilation time: ~10s
- Runtime: Fast (native code via interpreter)
- Memory: Efficient (no leaks detected)

## Next Steps

### High Priority
1. JSON support (json.parse, json.stringify)
2. HTTP client (http.get, http.post)
3. Better error messages with stack traces
4. Module system improvements

### Medium Priority
1. Regex support
2. Date/time functions
3. More list operations (sort, reverse, slice)
4. Dictionary/map type

### Low Priority
1. Async/await
2. Generators
3. Decorators
4. Type annotations

## Breaking Changes

None - all changes are backwards compatible

## Migration Guide

No migration needed. All existing code continues to work.

New features are opt-in via new modules (env, fs) and enhanced functions.

## Version

**Current**: v0.3.2
**Previous**: v0.3.1
**Release Date**: 2026-02-28
