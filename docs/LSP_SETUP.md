# m5rcode Language Server Setup

The enhanced LSP now supports:
- ✅ Real syntax checking (missing parentheses, unmatched braces, missing semicolons)
- ✅ Auto-completion (keywords, functions, modules)
- ✅ Hover information
- ✅ Go-to-definition
- ✅ Document symbols (functions, classes)
- ✅ Diagnostics with severity levels

## VS Code

### 1. Create extension (optional)

Create `.vscode/extensions/m5rcode/package.json`:
```json
{
  "name": "m5rcode",
  "displayName": "m5rcode Language Support",
  "version": "0.1.0",
  "engines": { "vscode": "^1.60.0" },
  "categories": ["Programming Languages"],
  "contributes": {
    "languages": [{
      "id": "m5rcode",
      "aliases": ["m5rcode", "m5"],
      "extensions": [".m5"],
      "configuration": "./language-configuration.json"
    }],
    "grammars": [{
      "language": "m5rcode",
      "scopeName": "source.m5",
      "path": "./syntaxes/m5rcode.tmLanguage.json"
    }],
    "configuration": {
      "title": "m5rcode",
      "properties": {
        "m5rcode.lsp.path": {
          "type": "string",
          "default": "m5lsp",
          "description": "Path to m5lsp executable"
        }
      }
    }
  },
  "activationEvents": ["onLanguage:m5rcode"],
  "main": "./extension.js"
}
```

### 2. Or use settings.json directly

Create `.vscode/settings.json`:
```json
{
  "files.associations": {
    "*.m5": "m5rcode"
  }
}
```

Then install a generic LSP extension and configure it to use `m5lsp`.

## Neovim

Add to your config (`~/.config/nvim/init.lua`):

```lua
-- Register m5rcode filetype
vim.filetype.add({
  extension = {
    m5 = 'm5rcode',
  },
})

-- Setup LSP
vim.api.nvim_create_autocmd('FileType', {
  pattern = 'm5rcode',
  callback = function()
    vim.lsp.start({
      name = 'm5rcode-lsp',
      cmd = {'m5lsp'},
      root_dir = vim.fs.dirname(vim.fs.find({'.git', 'Cargo.toml'}, { upward = true })[1]),
    })
  end,
})

-- Optional: Add syntax highlighting
vim.api.nvim_create_autocmd({'BufRead', 'BufNewFile'}, {
  pattern = '*.m5',
  callback = function()
    vim.bo.filetype = 'm5rcode'
    vim.bo.commentstring = '# %s'
  end,
})
```

## Helix

Add to `~/.config/helix/languages.toml`:

```toml
[[language]]
name = "m5rcode"
scope = "source.m5"
injection-regex = "m5"
file-types = ["m5"]
comment-token = "#"
indent = { tab-width = 4, unit = "    " }
language-server = { command = "m5lsp" }

[[grammar]]
name = "m5rcode"
source = { git = "https://github.com/m5rcode/tree-sitter-m5rcode", rev = "main" }
```

## Emacs (lsp-mode)

Add to your config:

```elisp
(add-to-list 'auto-mode-alist '("\\.m5\\'" . m5rcode-mode))

(define-derived-mode m5rcode-mode prog-mode "m5rcode"
  "Major mode for m5rcode files."
  (setq-local comment-start "#")
  (setq-local comment-end ""))

(with-eval-after-load 'lsp-mode
  (add-to-list 'lsp-language-id-configuration '(m5rcode-mode . "m5rcode"))
  (lsp-register-client
   (make-lsp-client
    :new-connection (lsp-stdio-connection "m5lsp")
    :major-modes '(m5rcode-mode)
    :server-id 'm5rcode-lsp)))
```

## Sublime Text

Create `Packages/User/m5rcode.sublime-settings`:

```json
{
  "extensions": ["m5"],
  "lsp_settings": {
    "m5rcode": {
      "command": ["m5lsp"],
      "enabled": true,
      "languageId": "m5rcode"
    }
  }
}
```

## Kate/KWrite

Create `~/.local/share/katepart5/syntax/m5rcode.xml`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE language SYSTEM "language.dtd">
<language name="m5rcode" section="Sources" extensions="*.m5" mimetype="text/x-m5rcode" version="1">
  <highlighting>
    <list name="keywords">
      <item>fn</item>
      <item>class</item>
      <item>if</item>
      <item>else</item>
      <item>while</item>
      <item>for</item>
      <item>return</item>
      <item>import</item>
      <item>let</item>
      <item>const</item>
    </list>
  </highlighting>
</language>
```

## Testing the LSP

Create a test file `test.m5`:

```m5
import std.io

fn main() {
    io.println("Hello, World!")
}

fn broken_function
    # Missing parentheses - LSP will warn
}

class MyClass {
    x: int
}
```

Open in your editor and you should see:
- ✅ Syntax errors highlighted
- ✅ Auto-completion when typing
- ✅ Hover information on keywords
- ✅ Document outline showing functions/classes

## LSP Features

### Diagnostics
- Error (severity 1): Missing parentheses
- Warning (severity 2): Unmatched braces
- Info (severity 3): Missing semicolons

### Completions
- Keywords: `fn`, `class`, `if`, `else`, `while`, `for`, `return`, `import`, `let`, `const`
- Functions: `io.println`, `io.print`, `io.readln`
- Modules: `std.io`, `std.collections`, `std.math`

### Symbols
- Functions (kind 12)
- Classes (kind 5)

## Troubleshooting

If LSP doesn't work:

1. **Check m5lsp is in PATH:**
   ```bash
   which m5lsp
   m5lsp --version  # Should start the server
   ```

2. **Check LSP logs:**
   - VS Code: Output → m5rcode Language Server
   - Neovim: `:LspLog`
   - Helix: Check `~/.cache/helix/helix.log`

3. **Test manually:**
   ```bash
   echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | m5lsp
   ```

## Next Steps

To further enhance the LSP:
1. Add semantic tokens for syntax highlighting
2. Implement code actions (quick fixes)
3. Add formatting support
4. Implement rename refactoring
5. Add workspace symbols
6. Integrate with the actual compiler for real type checking
