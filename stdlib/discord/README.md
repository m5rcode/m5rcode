# discord.m5 - Production-Grade Discord Bot Library

A professional Discord bot framework for m5rcode with modular command handling, event system, and clean architecture.

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Discord API                          │
└────────────────────┬────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────┐
│          Rust Bridge (m5rcode-discord)                  │
│  • Event handling                                       │
│  • Command routing                                      │
│  • JSON serialization                                   │
│  • Environment variable passing                         │
└────────────────────┬────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────┐
│         m5rcode Command Handlers (.m5 files)            │
│  • Modular command scripts                             │
│  • Access to stdlib functions                          │
│  • Clean separation of concerns                        │
└─────────────────────────────────────────────────────────┘
```

## Features

✅ **Modular Command System** - Each command is a separate .m5 file
✅ **JSON Configuration** - Easy bot setup with bot_config.json
✅ **Event-Driven** - Proper event handling with Discord API
✅ **Environment Variables** - Commands receive context via env vars
✅ **Production-Ready** - Proper error handling and logging
✅ **Extensible** - Easy to add new commands
✅ **Type-Safe** - Rust bridge ensures reliability

## Quick Start

### 1. Project Structure

```
your-bot/
├── bot_config.json       # Bot configuration
└── handlers/             # Command handlers
    ├── ping.m5
    ├── help.m5
    ├── info.m5
    └── ...
```

### 2. Configuration (bot_config.json)

```json
{
  "bot": {
    "name": "MyBot",
    "version": "1.0.0",
    "prefix": "!"
  },
  "commands": {
    "ping": "handlers/ping.m5",
    "help": "handlers/help.m5"
  }
}
```

### 3. Create Command Handler

**handlers/ping.m5:**
```m5
import std.io

io.println("🏓 Pong!")
```

### 4. Run Your Bot

```bash
cd /path/to/m5rcode/runtime/discord
./target/release/m5rcode-discord <TOKEN> /path/to/bot_config.json
```

## Command Handler API

Each command handler is a standalone m5rcode script that:
- Receives context via environment variables
- Outputs response via `io.println()`
- Has access to full m5rcode stdlib

### Available Environment Variables

- `DISCORD_COMMAND` - Command name (e.g., "ping")
- `DISCORD_ARGS` - Command arguments
- `DISCORD_AUTHOR` - Message author name
- `DISCORD_CHANNEL` - Channel ID
- `DISCORD_EVENT` - Path to full event JSON

### Example: Advanced Command

**handlers/greet.m5:**
```m5
import std.io

# Get author name from environment
# Note: In production, you'd read from DISCORD_AUTHOR env var
let author = "User"

io.println("👋 Hello, " + author + "!")
io.println("Welcome to the server!")
```

## Built-in Commands

The example bot includes:

| Command | Description |
|---------|-------------|
| `!ping` | Test bot responsiveness |
| `!hello` | Greeting message |
| `!help` | Command list and help |
| `!info` | Bot information |
| `!calc` | Calculator examples |
| `!math` | Math operations demo |

## Creating Custom Commands

### 1. Create Handler File

```m5
# handlers/mycommand.m5
import std.io

io.println("This is my custom command!")
```

### 2. Register in Config

```json
{
  "commands": {
    "mycommand": "handlers/mycommand.m5"
  }
}
```

### 3. Restart Bot

The command is now available as `!mycommand`

## Advanced Features

### Math Operations

```m5
import std.io

let result = math.sqrt(16)
io.println("Result: " + toStr(result))
```

### String Manipulation

```m5
import std.io

let text = "  hello world  "
let cleaned = str.trim(text)
let upper = str.upper(cleaned)
io.println(upper)
```

### Lists

```m5
import std.io

let numbers = [1, 2, 3, 4, 5]
let length = list.len(numbers)
io.println("List has " + toStr(length) + " items")
```

## Error Handling

The Rust bridge handles:
- ✅ Bot message filtering (prevents infinite loops)
- ✅ Command prefix validation
- ✅ Missing handler files
- ✅ Script execution errors
- ✅ Discord API errors

## Logging

The bridge logs:
- Command registrations on startup
- Successful command executions
- Errors and warnings
- Bot connection status

## Performance

- **Fast startup** - Rust-based bridge
- **Low latency** - Direct command execution
- **Scalable** - Handles multiple commands efficiently
- **Memory efficient** - Each command runs in isolation

## Security

- ✅ Bot messages ignored (prevents loops)
- ✅ Command validation
- ✅ Sandboxed script execution
- ✅ No direct Discord token exposure to scripts

## Deployment

### Development

```bash
./target/release/m5rcode-discord <TOKEN> bot_config.json
```

### Production (with systemd)

```ini
[Unit]
Description=m5rcode Discord Bot
After=network.target

[Service]
Type=simple
User=botuser
WorkingDirectory=/path/to/bot
ExecStart=/path/to/m5rcode-discord TOKEN /path/to/bot_config.json
Restart=always

[Install]
WantedBy=multi-user.target
```

## Troubleshooting

### Bot not responding

1. Check token is valid
2. Verify MESSAGE_CONTENT intent is enabled
3. Check bot has permissions in channel
4. Review logs for errors

### Command not found

1. Verify handler file exists
2. Check path in bot_config.json
3. Ensure file has .m5 extension
4. Restart bot after config changes

### Script errors

1. Test handler locally: `m5repl handlers/command.m5`
2. Check syntax errors
3. Verify imports are correct
4. Review error output in logs

## Examples

See `examples/discord/` for:
- Complete bot setup
- All command handlers
- Configuration examples
- Best practices

## Contributing

Want to improve discord.m5?
- Add new stdlib functions
- Improve error handling
- Add more event types
- Enhance documentation

## License

MIT OR Apache-2.0 (same as m5rcode)

---

**Built with ❤️ using m5rcode v0.3.1**
