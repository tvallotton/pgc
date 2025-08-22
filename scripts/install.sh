#!/usr/bin/env bash
set -euo pipefail

# Defaults
VERSION="${PGC_VERSION:-v0.1.0-alpha}"
INSTALL_DIR_DEFAULT="/usr/local/bin"
INSTALL_DIR="${PGC_INSTALL_DIR:-}"

usage() {
  cat <<EOF
pgc installer
Usage: [env PGC_VERSION=<tag>] [env PGC_INSTALL_DIR=<dir>] install.sh [--version <tag>] [--to <dir>]

Options:
  --version <tag>   Release tag to install (default: ${VERSION})
  --to <dir>        Install directory (default: writable /usr/local/bin, else ~/.local/bin)

Environment:
  PGC_VERSION       Same as --version
  PGC_INSTALL_DIR   Same as --to
EOF
}

# Parse args (simple)
while [[ $# -gt 0 ]]; do
  case "$1" in
    --version|-v) VERSION="$2"; shift 2;;
    --to|-t) INSTALL_DIR="$2"; shift 2;;
    --help|-h) usage; exit 0;;
    *) echo "Unknown arg: $1"; usage; exit 1;;
  esac
done

# Detect OS/arch
uname_s="$(uname -s)"
uname_m="$(uname -m)"

case "$uname_s" in
  Darwin)  PLATFORM_SUFFIX="apple-darwin" ;;
  Linux)   PLATFORM_SUFFIX="unknown-linux-gnu" ;;
  *) echo "Unsupported OS: $uname_s"; exit 1 ;;
esac

case "$uname_m" in
  x86_64|amd64) ARCH="x86_64" ;;
  arm64|aarch64) ARCH="aarch64" ;;
  *) echo "Unsupported arch: $uname_m"; exit 1 ;;
esac

ASSET="pgc-${ARCH}-${PLATFORM_SUFFIX}"
URL="https://github.com/tvallotton/pgc/releases/download/${VERSION}/${ASSET}"

# Pick install dir
if [[ -z "${INSTALL_DIR}" ]]; then
  if [[ -w "${INSTALL_DIR_DEFAULT}" ]]; then
    INSTALL_DIR="${INSTALL_DIR_DEFAULT}"
    SUDO=""
  else
    INSTALL_DIR="${HOME}/.local/bin"
    mkdir -p "${INSTALL_DIR}"
    SUDO=""
  fi
fi

TMP="$(mktemp -d)"
cleanup() { rm -rf "$TMP"; }
trap cleanup EXIT

echo "-> Downloading ${URL}"
if command -v curl >/dev/null 2>&1; then
  curl -fL "$URL" -o "${TMP}/pgc"
elif command -v wget >/dev/null 2>&1; then
  wget -q "$URL" -O "${TMP}/pgc"
else
  echo "Need curl or wget to download." >&2
  exit 1
fi

chmod +x "${TMP}/pgc"

# If target dir not writable, try sudo
if [[ ! -w "${INSTALL_DIR}" ]]; then
  if command -v sudo >/dev/null 2>&1; then
    echo "-> Installing to ${INSTALL_DIR} (sudo)"
    sudo mkdir -p "${INSTALL_DIR}"
    sudo mv "${TMP}/pgc" "${INSTALL_DIR}/pgc"
  else
    echo "Target ${INSTALL_DIR} not writable and sudo not available." >&2
    echo "Try: PGC_INSTALL_DIR=\$HOME/.local/bin" >&2
    exit 1
  fi
else
  echo "-> Installing to ${INSTALL_DIR}"
  mv "${TMP}/pgc" "${INSTALL_DIR}/pgc"
fi

# PATH hint
if ! command -v pgc >/dev/null 2>&1; then
  echo "Installed to ${INSTALL_DIR}, but 'pgc' not on PATH."
  echo "Add to PATH, e.g.: export PATH=\"${INSTALL_DIR}:\$PATH\""
fi

echo "-> pgc $( "${INSTALL_DIR}/pgc" --version || true )"
echo "Done."
