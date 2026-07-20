#!/usr/bin/env bash
# obmc install script
# Usage: curl -fsSL https://github.com/MrDwarf7/obmc.rs/releases/latest/download/install.sh | bash
set -euo pipefail

REPO="MrDwarf7/obmc.rs"
BIN="obmc"

# ---- detect platform ----
ARCH="$(uname -m)"
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"

case "$OS" in
  linux)
    TARGET="${ARCH}-unknown-linux-gnu"
    ;;
  darwin)
    [[ "$ARCH" == "arm64" ]] && ARCH="aarch64"
    TARGET="${ARCH}-apple-darwin"
    ;;
  mingw*|msys*|cygwin*)
    TARGET="${ARCH}-pc-windows-msvc"
    BIN="${BIN}.exe"
    ;;
  *)
    echo "Unsupported OS: $OS"
    exit 1
    ;;
esac

# ---- fetch latest release tag ----
echo "Fetching latest release..."
LATEST="$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" | sed -n 's/.*"tag_name": "\([^"]*\)".*/\1/p')"
[[ -z "$LATEST" ]] && { echo "Could not find latest release"; exit 1; }

ARCHIVE="${BIN%.exe}-${TARGET}-${LATEST}.zip"
DOWNLOAD="https://github.com/$REPO/releases/download/${LATEST}/${ARCHIVE}"

echo "Downloading obmc ${LATEST} for ${TARGET}..."
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

curl -fsSL "$DOWNLOAD" -o "$TMP/obmc.zip"

# ---- extract ----
cd "$TMP"
unzip -q obmc.zip
cd - >/dev/null

# ---- install ----
if [[ "$OS" == "mingw"* || "$OS" == "msys"* ]]; then
  INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"
else
  INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"
fi

mkdir -p "$INSTALL_DIR"
cp "$TMP/$BIN" "$INSTALL_DIR/$BIN"
chmod +x "$INSTALL_DIR/$BIN"

echo "Installed obmc ${LATEST} to ${INSTALL_DIR}/${BIN}"
echo "Run 'obmc --help' to get started."
