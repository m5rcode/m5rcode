#!/bin/bash
# m5discord - Run m5rcode Discord bots easily
# Usage: m5discord bot.m5

set -e

if [ $# -lt 1 ]; then
    echo "Usage: m5discord <bot.m5>"
    echo ""
    echo "Your bot.m5 should contain:"
    echo "  let BOT_TOKEN = \"your_token_here\""
    exit 1
fi

BOT_FILE="$1"

if [ ! -f "$BOT_FILE" ]; then
    echo "Error: File not found: $BOT_FILE"
    exit 1
fi

# Extract token from bot file
TOKEN=$(grep "BOT_TOKEN" "$BOT_FILE" | sed 's/.*"\(.*\)".*/\1/' | head -1)

if [ -z "$TOKEN" ]; then
    echo "Error: No BOT_TOKEN found in $BOT_FILE"
    echo "Add this line to your bot:"
    echo '  let BOT_TOKEN = "your_discord_token"'
    exit 1
fi

echo "🤖 Starting Discord bot from: $BOT_FILE"
echo "🔑 Token: ${TOKEN:0:20}..."
echo ""

# Find the m5rcode-discord binary
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BRIDGE_BIN="$SCRIPT_DIR/runtime/discord/target/release/m5rcode-discord"

if [ ! -f "$BRIDGE_BIN" ]; then
    # Try in PATH
    if command -v m5rcode-discord &> /dev/null; then
        BRIDGE_BIN="m5rcode-discord"
    else
        echo "Error: m5rcode-discord not found at: $BRIDGE_BIN"
        echo "Build it with: cd runtime/discord && cargo build --release"
        exit 1
    fi
fi

# Run the bridge
exec "$BRIDGE_BIN" "$TOKEN" "$BOT_FILE"
