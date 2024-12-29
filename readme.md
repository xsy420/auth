# Auth

![](.github/auth.png)

<div align="center">

A simple terminal-based authenticator app written in Rust that generates TOTP codes.

</div>

## Security

> [!WARNING]
> TOTP secrets are stored unencrypted in `~/.local/share/auth/entries.toml`. This will be changed later in the future.

## Features

- Generate TOTP codes with remaining time
- Add/Delete entries
- Import/Export entries as TOML files
- Copy codes to clipboard (requires wl-copy)
- Terminal UI with keyboard controls

## Usage

### Controls

- `a`: Add new entry
- `E`: Edit selected entry
- `d`: Delete selected entry
- `i`: Import entries
- `e`: Export entries
- `↑/k`: Move selection up
- `↓/j`: Move selection down
- `Enter`: Copy code to clipboard
- `q`: Quit

### Building

```
cargo build --release
```

## License

GPL-2.0