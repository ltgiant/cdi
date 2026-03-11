#!/bin/sh
set -e

REPO="ltgiant/cdi"
INSTALL_DIR="${CDI_INSTALL_DIR:-/usr/local/bin}"

# Detect platform
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Darwin)
    case "$ARCH" in
      arm64) TARGET="aarch64-apple-darwin" ;;
      x86_64) TARGET="x86_64-apple-darwin" ;;
      *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
    esac
    ;;
  Linux)
    case "$ARCH" in
      x86_64) TARGET="x86_64-unknown-linux-gnu" ;;
      *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
    esac
    ;;
  *) echo "Unsupported OS: $OS"; exit 1 ;;
esac

# Get latest version
VERSION="$(curl -sI "https://github.com/$REPO/releases/latest" | grep -i '^location:' | sed 's/.*tag\///' | tr -d '\r\n')"
if [ -z "$VERSION" ]; then
  echo "Failed to fetch latest version"
  exit 1
fi

URL="https://github.com/$REPO/releases/download/$VERSION/cdi-$TARGET.tar.gz"

echo "Installing cdi $VERSION for $TARGET..."

TMPDIR="$(mktemp -d)"
trap 'rm -rf "$TMPDIR"' EXIT

curl -sL "$URL" | tar xz -C "$TMPDIR"

if [ -w "$INSTALL_DIR" ]; then
  mv "$TMPDIR/cdi" "$INSTALL_DIR/cdi"
else
  sudo mv "$TMPDIR/cdi" "$INSTALL_DIR/cdi"
fi

echo "Installed cdi to $INSTALL_DIR/cdi"
echo ""
echo "Add to your shell config:"
echo '  eval "$(cdi init zsh)"    # zsh'
echo '  eval "$(cdi init bash)"   # bash'
echo '  cdi init fish | source    # fish'
