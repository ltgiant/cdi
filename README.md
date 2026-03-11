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

### Quick install (macOS / Linux)

```bash
curl -fsSL https://raw.githubusercontent.com/ltgiant/cdi/master/install.sh | sh
```

### Homebrew (macOS / Linux)

```bash
brew install ltgiant/tap/cdi
```

### Scoop (Windows)

```powershell
scoop bucket add ltgiant https://github.com/ltgiant/scoop-bucket
scoop install cdi
```

### Windows (manual)

Download `cdi-windows-x86_64.exe` from the [latest release](https://github.com/ltgiant/cdi/releases/latest), rename to `cdi.exe`, and add to your PATH.

### Build from source

Requires [Rust](https://rustup.rs/).

```bash
git clone https://github.com/ltgiant/cdi.git
cd cdi
cargo build --release
```

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
