# Auth

A simple terminal-based authenticator app written in Rust that generates TOTP codes.

## Installation

### Arch Linux

Auth is available on the [AUR](https://aur.archlinux.org/packages/auth-tui)

```bash
paru -S auth-tui
```

### Releases

Prebuilt binaries can be found in the [releases](https://github.com/nnyyxxxx/auth/releases) page

```bash
curl -fsSL https://github.com/nnyyxxxx/auth/releases/latest/download/auth -o auth
chmod +x auth
sudo mv auth /usr/bin
```

### Building

The built binary will be located inside of `target/release/`, Then it can be placed in `/usr/bin/`.

```bash
sudo pacman -S --needed rust git base-devel
git clone https://github.com/nnyyxxxx/auth.git
cd auth
cargo build --release
```
