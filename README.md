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

A subprocess cannot change the parent shell's directory, so a thin shell wrapper is required. Add the following to your shell config:

**Zsh** (`~/.zshrc`):
```zsh
export PATH="/path/to/cdi/target/release:$PATH"
source /path/to/cdi/shell/cdi.zsh
```

**Bash** (`~/.bashrc`):
```bash
export PATH="/path/to/cdi/target/release:$PATH"
source /path/to/cdi/shell/cdi.bash
```

**Fish** (`~/.config/fish/config.fish`):
```fish
set -gx PATH /path/to/cdi/target/release $PATH
source /path/to/cdi/shell/cdi.fish
```

**PowerShell** (`$PROFILE`):
```powershell
$env:Path = "C:\path\to\cdi\target\release;" + $env:Path
. C:\path\to\cdi\shell\cdi.ps1
```

## Features

- Lists subdirectories (including hidden) with numeric indices
- Case-insensitive alphabetical sorting
- Navigate by index number
- Cross-platform: macOS, Linux, Windows
- Shell integrations: zsh, bash, fish, PowerShell

## License

MIT
