#!/bin/sh
set -eu
if (set -o pipefail) 2>/dev/null; then
  set -o pipefail
fi

REPO="JinyangWang27/iztro-rs"
BASE_URL="${IZTRO_GUI_RELEASE_URL:-https://github.com/${REPO}/releases/latest/download}"
BIN_DIR="${XDG_BIN_HOME:-${HOME}/.local/bin}"

case "$BASE_URL" in
  https://*) ;;
  *)
    echo "error: release URL must use HTTPS: $BASE_URL" >&2
    exit 1
    ;;
esac

require_command() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "error: required command not found: $1" >&2
    exit 1
  fi
}

download_file() {
  url="$1"
  output="$2"
  curl --proto '=https' --tlsv1.2 -fLsS "$url" -o "$output"
}

sha256_file() {
  file="$1"
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$file" | awk '{print $1}'
  elif command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$file" | awk '{print $1}'
  else
    echo "error: sha256sum or shasum is required to verify the download" >&2
    exit 1
  fi
}

os="$(uname -s)"
arch="$(uname -m)"

case "$os:$arch" in
  Darwin:arm64 | Darwin:aarch64)
    artifact="iztro-gui-aarch64-apple-darwin.tar.gz"
    ;;
  Darwin:x86_64 | Darwin:amd64)
    artifact="iztro-gui-x86_64-apple-darwin.tar.gz"
    ;;
  Linux:x86_64 | Linux:amd64)
    artifact="iztro-gui-x86_64-unknown-linux-gnu.tar.gz"
    ;;
  *)
    echo "error: unsupported platform: $os $arch" >&2
    exit 1
    ;;
esac

require_command curl
require_command tar
require_command awk

tmp_dir="$(mktemp -d)"
cleanup() {
  rm -rf "$tmp_dir"
}
trap cleanup EXIT INT TERM

archive="$tmp_dir/$artifact"
checksum="$archive.sha256"
artifact_dir="${artifact%.tar.gz}"

echo "Downloading $artifact"
download_file "$BASE_URL/$artifact" "$archive"
download_file "$BASE_URL/$artifact.sha256" "$checksum"

expected="$(awk '{print $1}' "$checksum" | tr 'A-F' 'a-f')"
actual="$(sha256_file "$archive" | tr 'A-F' 'a-f')"
if [ "$expected" != "$actual" ]; then
  echo "error: checksum verification failed for $artifact" >&2
  echo "expected: $expected" >&2
  echo "actual:   $actual" >&2
  exit 1
fi

tar -xzf "$archive" -C "$tmp_dir"
source_binary="$tmp_dir/$artifact_dir/iztro-gui"
if [ ! -f "$source_binary" ]; then
  echo "error: archive did not contain iztro-gui" >&2
  exit 1
fi

mkdir -p "$BIN_DIR"
target="$BIN_DIR/iztro-gui"
if [ -d "$target" ]; then
  echo "error: install target is a directory: $target" >&2
  exit 1
fi
if [ -e "$target" ]; then
  echo "Replacing existing $target"
fi

tmp_target="$BIN_DIR/.iztro-gui.tmp.$$"
cp "$source_binary" "$tmp_target"
chmod 755 "$tmp_target"
mv "$tmp_target" "$target"

echo "Installed iztro-gui to $target"
echo "Run it with: $target"
if ! command -v iztro-gui >/dev/null 2>&1; then
  echo "Note: $BIN_DIR is not on PATH; run the full path above or add it to PATH."
fi
