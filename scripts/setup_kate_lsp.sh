#!/bin/bash
# Setup m5rcode LSP for Kate editor

set -e

echo "==> Setting up m5rcode LSP for Kate"

# Create syntax highlighting file
echo "==> Creating syntax highlighting..."
mkdir -p ~/.local/share/org.kde.syntax-highlighting/syntax/
cat > ~/.local/share/org.kde.syntax-highlighting/syntax/m5rcode.xml << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE language>
<language name="m5rcode" section="Sources" extensions="*.m5" mimetype="text/x-m5rcode" version="1" kateversion="5.0">
  <highlighting>
    <list name="keywords">
      <item>fn</item>
      <item>class</item>
      <item>trait</item>
      <item>impl</item>
      <item>if</item>
      <item>else</item>
      <item>while</item>
      <item>for</item>
      <item>in</item>
      <item>return</item>
      <item>break</item>
      <item>continue</item>
      <item>import</item>
      <item>export</item>
      <item>let</item>
      <item>const</item>
      <item>var</item>
      <item>own</item>
      <item>move</item>
      <item>async</item>
      <item>await</item>
      <item>match</item>
      <item>case</item>
    </list>
    <list name="types">
      <item>int</item>
      <item>float</item>
      <item>string</item>
      <item>bool</item>
      <item>null</item>
    </list>
    <contexts>
      <context name="Normal" attribute="Normal Text" lineEndContext="#stay">
        <keyword attribute="Keyword" context="#stay" String="keywords"/>
        <keyword attribute="Data Type" context="#stay" String="types"/>
        <DetectChar attribute="Comment" context="Comment" char="#"/>
        <DetectChar attribute="String" context="String" char="&quot;"/>
        <Int attribute="Number" context="#stay"/>
        <Float attribute="Float" context="#stay"/>
      </context>
      <context name="Comment" attribute="Comment" lineEndContext="#pop">
        <IncludeRules context="##Comments"/>
      </context>
      <context name="String" attribute="String" lineEndContext="#pop">
        <DetectChar attribute="String" context="#pop" char="&quot;"/>
      </context>
    </contexts>
    <itemDatas>
      <itemData name="Normal Text" defStyleNum="dsNormal"/>
      <itemData name="Keyword" defStyleNum="dsKeyword"/>
      <itemData name="Data Type" defStyleNum="dsDataType"/>
      <itemData name="Comment" defStyleNum="dsComment"/>
      <itemData name="String" defStyleNum="dsString"/>
      <itemData name="Number" defStyleNum="dsDecVal"/>
      <itemData name="Float" defStyleNum="dsFloat"/>
    </itemDatas>
  </highlighting>
  <general>
    <comments>
      <comment name="singleLine" start="#"/>
    </comments>
    <keywords casesensitive="1"/>
  </general>
</language>
EOF

echo "✓ Syntax highlighting installed"

# Add LSP configuration
echo "==> Configuring LSP..."

# Find Kate config directory
if [ -d ~/.config/kate ]; then
    KATE_CONFIG=~/.config/kate
elif [ -d ~/.local/share/kate ]; then
    KATE_CONFIG=~/.local/share/kate
else
    KATE_CONFIG=~/.config/kate
    mkdir -p "$KATE_CONFIG"
fi

LSP_CONFIG="$KATE_CONFIG/lspclient.json"

# Check if config exists
if [ -f "$LSP_CONFIG" ]; then
    echo "Found existing LSP config: $LSP_CONFIG"
    
    # Check if m5rcode already configured
    if grep -q '"m5rcode"' "$LSP_CONFIG"; then
        echo "✓ m5rcode LSP already configured"
    else
        echo "Adding m5rcode to existing config..."
        
        # Backup original
        cp "$LSP_CONFIG" "$LSP_CONFIG.backup"
        
        # Add m5rcode entry before the last closing brace
        sed -i '/"servers".*{/,/^[[:space:]]*}[[:space:]]*$/ {
            /^[[:space:]]*}[[:space:]]*$/ i\
        ,\
        "m5rcode": {\
            "command": ["m5lsp"],\
            "url": "https://github.com/m5rcode/m5rcode",\
            "highlightingModeRegex": "^m5rcode$"\
        }
        }' "$LSP_CONFIG"
        
        echo "✓ m5rcode LSP added to config"
        echo "  Backup saved: $LSP_CONFIG.backup"
    fi
else
    echo "Creating new LSP config..."
    cat > "$LSP_CONFIG" << 'EOF'
{
    "servers": {
        "m5rcode": {
            "command": ["m5lsp"],
            "url": "https://github.com/m5rcode/m5rcode",
            "highlightingModeRegex": "^m5rcode$"
        }
    }
}
EOF
    echo "✓ LSP config created: $LSP_CONFIG"
fi

echo ""
echo "==> Setup complete!"
echo ""
echo "Next steps:"
echo "1. Restart Kate"
echo "2. Open a .m5 file"
echo "3. LSP should activate automatically"
echo ""
echo "To verify:"
echo "  - Settings → Configure Kate → LSP Client"
echo "  - Check 'm5rcode' appears in the server list"
echo ""
