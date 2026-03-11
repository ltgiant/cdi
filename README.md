# cdi

Quick directory navigation with numeric indices.

Assign temporary numeric indices to subdirectories and jump into them instantly.

## Usage

```
$ cdi
  0) .git
  1) src
  2) tests

$ cdi 1
# moves into ./src
```

```
$ cdi --help
$ cdi --version
```

## Installation

### Build from source

Requires [Rust](https://rustup.rs/).

```bash
git clone https://github.com/ltgiant/cdi.git
cd cdi
cargo build --release
```

The binary will be at `target/release/cdi`.

### Shell integration

A subprocess cannot change the parent shell's directory, so a shell wrapper is required. Add **one line** to your shell config:

**Zsh** (`~/.zshrc`):
```zsh
eval "$(cdi init zsh)"
```

**Bash** (`~/.bashrc`):
```bash
eval "$(cdi init bash)"
```

**Fish** (`~/.config/fish/config.fish`):
```fish
cdi init fish | source
```

**PowerShell** (`$PROFILE`):
```powershell
Invoke-Expression (& cdi init powershell)
```

## Features

- Lists subdirectories (including hidden) with numeric indices
- Case-insensitive alphabetical sorting
- Navigate by index number
- Cross-platform: macOS, Linux, Windows
- Shell integrations: zsh, bash, fish, PowerShell

## License

MIT
