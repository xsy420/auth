# Auth

A simple terminal-based authenticator CLI written in CPP that generates TOTP codes.

## Overview

It is recommended that you have a keyring installed, otherwise this won't work as totp secrets are
stored in your keyring. For the list of dependencies see the submodules. The keyring requirement
will eventually change in the future, there will eventually be an option to store entries inside
of the database in plaintext if the user so desires.

## Installation

### Arch Linux

Auth is available on the [AUR](https://aur.archlinux.org/packages/auth-cli)

```bash
paru -S auth-cli
```

### Releases

Yoink the binary from the [releases page](https://github.com/nnyyxxxx/auth/releases/latest)

### Building

```bash
make release; sudo make install
```
