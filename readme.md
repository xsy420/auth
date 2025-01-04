<div align="center">

# Auth

![](.github/auth.png)

A simple terminal-based authenticator app written in Rust that generates TOTP codes.

</div>

## Security

> [!WARNING]
> TOTP secrets are stored encrypted in `~/.local/share/auth/entries.toml`. The key to unencrypt them is stored in `~/.local/share/auth/key`.

## Building

The built binary will be located inside of `target/release/`, Then it can be placed in `/usr/bin/`.

```bash
# Replace `pacman` with your package manager
sudo pacman -S --needed rust git base-devel
git clone https://github.com/nnyyxxxx/auth.git
cd auth
cargo run --release
```

## License

Copyright (C) 2024 [Nyx](https://github.com/nnyyxxxx)

This program is free software; you can redistribute it and/or modify it under the terms of the GNU General Public License version 2 as published by the Free Software Foundation.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program; if not, write to the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301, USA or see <https://www.gnu.org/licenses/old-licenses/gpl-2.0.txt>

The full license can be found in the [license](license) file.
