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

A subprocess cannot change the parent shell's directory, so a shell wrapper is required.

**Zsh** — add to `~/.zshrc`:
```zsh
eval "$(cdi init zsh)"
```
Or run this to append automatically:
```zsh
echo 'eval "$(cdi init zsh)"' >> ~/.zshrc
```

**Bash** — add to `~/.bashrc`:
```bash
eval "$(cdi init bash)"
```
Or run this to append automatically:
```bash
echo 'eval "$(cdi init bash)"' >> ~/.bashrc
```

**Fish** — add to `~/.config/fish/config.fish`:
```fish
cdi init fish | source
```
Or run this to append automatically:
```fish
echo 'cdi init fish | source' >> ~/.config/fish/config.fish
```

**PowerShell** — add to `$PROFILE`:
```powershell
Invoke-Expression ((& cdi.exe init powershell) -join "`n")
```
Or run this to append automatically:
```powershell
if (!(Test-Path $PROFILE)) { New-Item $PROFILE -Force | Out-Null }; Add-Content $PROFILE 'Invoke-Expression ((& cdi.exe init powershell) -join "`n")'
```

## Features

- Lists subdirectories (including hidden) with numeric indices
- Case-insensitive alphabetical sorting
- Navigate by index number
- Cross-platform: macOS, Linux, Windows
- Shell integrations: zsh, bash, fish, PowerShell

## License

MIT
