#!/bin/bash
# SessionStart hook: install the `rtk` CLI proxy (token-optimized output) for
# Claude Code on the web. See: https://github.com/rtk-ai/rtk
#
# `rtk` wraps common dev commands (git, test, log, diff, find, read, ...) and
# summarizes their output before it reaches the model, reducing token usage.
set -euo pipefail

# Only run in the remote (Claude Code on the web) environment.
if [ "${CLAUDE_CODE_REMOTE:-}" != "true" ]; then
  exit 0
fi

RTK_VERSION="v0.42.4"
# SHA256 of the exact pinned release archives (from the v0.42.4 checksums.txt).
# The download is verified against these before extraction; a mismatch aborts
# the install rather than running unverified code.
RTK_SHA256_x86_64="34975116da11e09e502501daf758143e0b22ed3a42a10eb67fb693a6270d9e36"
RTK_SHA256_aarch64="cc2b91c064eb670c097c184913c8fbcb1a943d53d7fe505375e96ba0c5b6459f"
INSTALL_DIR="${HOME}/.local/bin"
RTK_BIN="${INSTALL_DIR}/rtk"

mkdir -p "${INSTALL_DIR}"

# Persist PATH for the rest of the session.
if [ -n "${CLAUDE_ENV_FILE:-}" ]; then
  echo "export PATH=\"${INSTALL_DIR}:\$PATH\"" >> "${CLAUDE_ENV_FILE}"
fi
export PATH="${INSTALL_DIR}:${PATH}"

# Idempotent: skip download if the pinned version is already installed.
if [ -x "${RTK_BIN}" ] && "${RTK_BIN}" --version 2>/dev/null | grep -q "${RTK_VERSION#v}"; then
  echo "rtk ${RTK_VERSION} already installed."
  exit 0
fi

case "$(uname -m)" in
  x86_64|amd64)  ARCH="x86_64-unknown-linux-musl" ; SHA256="${RTK_SHA256_x86_64}" ;;
  aarch64|arm64) ARCH="aarch64-unknown-linux-gnu" ; SHA256="${RTK_SHA256_aarch64}" ;;
  *) echo "rtk: unsupported arch $(uname -m); skipping." ; exit 0 ;;
esac

URL="https://github.com/rtk-ai/rtk/releases/download/${RTK_VERSION}/rtk-${ARCH}.tar.gz"
TMP="$(mktemp -d)"
trap 'rm -rf "${TMP}"' EXIT

echo "Installing rtk ${RTK_VERSION} (${ARCH})..."
if ! curl -sSfL -o "${TMP}/rtk.tgz" "${URL}"; then
  echo "rtk: download failed (network policy may block GitHub releases); skipping."
  exit 0
fi

# Verify the archive against the pinned checksum before extracting/installing.
if ! echo "${SHA256}  ${TMP}/rtk.tgz" | sha256sum -c - >/dev/null 2>&1; then
  echo "rtk: checksum verification failed for ${ARCH}; refusing to install." >&2
  exit 0
fi

tar -xzf "${TMP}/rtk.tgz" -C "${TMP}"
install -m 0755 "${TMP}/rtk" "${RTK_BIN}"
"${RTK_BIN}" --version || true
