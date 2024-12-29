# Auth

![](.github/auth.png)

<div align="center">

A simple terminal-based authenticator app written in Rust that generates TOTP codes.

</div>

## Features

- Generate TOTP codes with remaining time
- Add/Delete entries
- Import/Export entries as TOML files
- Copy codes to clipboard (requires wl-copy)
- Terminal UI with keyboard controls

## Usage

### Controls

- `a`: Add new entry
- `d`: Delete selected entry
- `i`: Import entries
- `e`: Export entries
- `↑/k`: Move selection up
- `↓/j`: Move selection down
- `Enter`: Copy code to clipboard
- `q`: Quit

### Storage

Entries are stored in `~/.local/share/auth/entries.toml`

### Building

```
cargo build --release
```

## License

GPL-2.0