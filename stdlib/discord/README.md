# discord.m5 - Discord Bot Library for m5rcode

Build Discord bots using m5rcode! This library provides a simple API similar to discord.py.

## Features

- ✅ Simple command handling
- ✅ Event-based architecture
- ✅ Easy message responses
- ✅ Math operations
- ✅ String manipulation
- 🚧 Full Discord API (coming soon)

## Quick Start

### 1. Create a Bot

```m5
import std.io

# Define commands
fn handle_ping() {
    io.println("🏓 Pong!")
}

fn handle_hello() {
    io.println("👋 Hello from m5rcode!")
}

# Test locally
handle_ping()
handle_hello()
```

### 2. Get Discord Bot Token

1. Go to [Discord Developer Portal](https://discord.com/developers/applications)
2. Create a New Application
3. Go to "Bot" section
4. Click "Reset Token" and copy it
5. Enable "MESSAGE CONTENT INTENT" under Privileged Gateway Intents
6. Go to OAuth2 → URL Generator
7. Select scopes: `bot`
8. Select permissions: `Send Messages`, `Read Messages/View Channels`
9. Copy the generated URL and invite the bot to your server

### 3. Run Your Bot

```bash
# Test locally first
m5repl examples/discord/simple_bot.m5

# Run with Discord (coming soon)
m5rcode-discord <YOUR_TOKEN> examples/discord/simple_bot.m5
```

## Example Bot

See `examples/discord/simple_bot.m5` for a complete example with:
- !ping - Responds with "Pong!"
- !hello - Greets the user
- !calc - Performs calculations
- !info - Shows bot information
- !math - Math operations

## Commands

### Basic Commands

```m5
fn handle_ping() {
    io.println("Pong!")
}

fn handle_hello() {
    io.println("Hello, World!")
}
```

### Math Commands

```m5
fn handle_calc() {
    let result = 2 + 2
    io.println("Result: " + toStr(result))
}

fn handle_math(a, b) {
    io.println(toStr(a) + " + " + toStr(b) + " = " + toStr(a + b))
}
```

### String Commands

```m5
fn handle_echo(message) {
    io.println("Echo: " + message)
}

fn handle_upper(text) {
    io.println(str.upper(text))
}
```

## Roadmap

- [x] Basic command structure
- [x] Local testing
- [ ] Discord API integration (Rust bridge)
- [ ] Message events
- [ ] Command parsing
- [ ] Embed support
- [ ] Reaction handling
- [ ] Voice channel support

## Architecture

```
┌─────────────────┐
│   Discord API   │
└────────┬────────┘
         │
┌────────▼────────┐
│  Rust Bridge    │  (m5rcode-discord)
│   (Serenity)    │
└────────┬────────┘
         │
┌────────▼────────┐
│  m5rcode Bot    │  (your_bot.m5)
│   (m5repl)      │
└─────────────────┘
```

## Contributing

Want to help build discord.m5? Check out:
- `stdlib/discord/` - Core library
- `runtime/discord/` - Rust bridge
- `examples/discord/` - Example bots

## License

MIT OR Apache-2.0 (same as m5rcode)
