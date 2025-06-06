This repository is originally posted at [not found now](https://github.com/nnyyxxxx/auth). But for unknown reason, the repository and user is deleted from GitHub.
The whole backup is at [nnyyxxxx](../../tree/nnyyxxxx) branch.
<div align="center">
  <h1>Auth</h1>

[![Code Quality Badge]][Code Quality]
[![Rust CI Badge]][Rust CI]
[![License Badge]][License]
</div>
A simple terminal-based authenticator app written in Rust that generates TOTP codes.

## Installation

### Arch Linux

Auth is available on my [repo](https://github.com/xsy420-arch/repo/blob/main/auth/PKGBUILD)

### Releases

Pre-built binaries can be found in the [releases](https://github.com/xsy420/auth/releases) page

```bash
curl -fsSL https://github.com/xsy420/auth/releases/latest/download/auth -o auth
chmod +x auth
sudo mv auth /usr/bin
```

### Building

The built binary will be located inside `target/release/`, Then it can be placed in `/usr/bin/`.

```bash
sudo pacman -S --needed rust git base-devel
git clone https://github.com/xsy420/auth.git
cd auth
cargo build --release
```

[Code Quality]: https://github.com/xsy420/project-setup/actions/workflows/code_quality.yml
[Rust CI]: https://github.com/xsy420/project-setup/actions/workflows/rust.yml
[License]: https://opensource.org/license/GPL-2.0

[Code Quality Badge]: https://img.shields.io/github/actions/workflow/status/xsy420/auth/code_quality.yml?style=flat-square&logo=githubactions&logoColor=ffffff&label=Code+Quality&labelColor=2088FF&color=347D39&event=push
[Rust CI Badge]: https://img.shields.io/github/actions/workflow/status/xsy420/auth/rust.yml?style=flat-square&logo=rust&logoColor=ffffff&label=Rust+CI&labelColor=BC826A&color=347D39&event=push
[License Badge]: https://img.shields.io/badge/License-GPL--2.0--only-blue.svg?style=flat-square
