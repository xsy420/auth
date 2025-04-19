# Auth

A simple terminal-based authenticator CLI written in CPP that generates TOTP codes.

## Installation

### Arch Linux

Auth is available on the [AUR](https://aur.archlinux.org/packages/auth-cli)

```bash
paru -S auth-cli
```

### Releases

Yoink the binary from the
[releases page](https://github.com/nnyyxxxx/auth/releases/latest)

### Building

```bash
make release; sudo make install
```

## Overview

It is recommended that you have a keyring installed, otherwise this won't work as
totp secrets are stored in your keyring. For the list of dependencies see the
submodules. The keyring requirement will eventually change in the future, there
will eventually be an option to store entries inside of the database in plaintext
if the user so desires.

### Common mistakes

#### My code says Invalid Key!?!?! What do I do??

It is 100% likely that your keyring is not running.

#### I can't add my super special secret with cool symbols!!1!

If your secret contains anything other than letters, numbers, spaces, or hyphens,
it will be rejected. Base32 encoding only uses those characters.

#### My 12-digit TOTP code doesn't work?!

The app only supports 6-8 digit codes because that's what every sane
authentication app uses. If you need more digits, you might also need therapy.

#### Auth says "Period cannot be 0"

The time period has to be greater than 0. Common sense.

#### I keep getting "Entry not found" when I KNOW it's there!

You can refer to entries by name or #.

#### Where is my database stored? I want to do dangerous things to it!

By default, it's at `~/.local/share/auth/auth.db`, but you can change it with the
`AUTH_DATABASE_DIR` environment variable.

#### I just installed auth but it says it can't save my entries?!

This happens when you ignored the first recommendation about having a keyring
installed. Install and run one of these: GNOME Keyring, KDE Wallet.
Yes, this means you have to actually READ the documentation.

#### Why can't I export to my favorite obscure file format?

Because we only support TOML and JSON like normal people.

#### I added a bunch of entries and now I regret everything!

Use the `wipe` command to delete everything.

#### I can't edit my entry because "Cannot edit entry with unavailable secret"?

This happens when you're trying to edit an entry but the secret can't be
retrieved from the keyring. Please make sure your keyring is running.

#### Help! I'm trying to import but nothing happens!

Double-check your file format. If you claim it's JSON but it's actually a picture
of your cat, the import will fail.
