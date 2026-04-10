#!/usr/bin/env bash
# lint_report.sh — Compile (if needed) and run the Rust lint_report binary,
#                  then write a Markdown report via ruff + mypy.
# Usage: ./lint_report.sh [src_dir] [output_file]
#   src_dir     directory to lint (default: src)
#   output_file report path      (default: lint_report.md)
set -euo pipefail

SRC_DIR="${1:-src}"
OUTPUT="${2:-lint_report.md}"

# ── paths ──────────────────────────────────────────────────────────────────────
# The Rust tool lives entirely inside scripts/ — it never touches src/ or
# Cargo.toml, so it doesn't interfere with the library crate.
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RUST_SRC="$SCRIPT_DIR/lint_report.rs"
BINARY="$SCRIPT_DIR/lint_report_bin"

# ── helpers ────────────────────────────────────────────────────────────────────
command_exists() { command -v "$1" &>/dev/null; }

check_tool() {
  if ! command_exists "$1"; then
    echo "ERROR: '$1' not found. Install it with: ${2:-install $1}" >&2
    exit 1
  fi
}

# ── pre-flight ─────────────────────────────────────────────────────────────────
if [ ! -d "$SRC_DIR" ]; then
  echo "ERROR: Source directory '$SRC_DIR' not found." >&2
  exit 1
fi

check_tool ruff  "pip install ruff"
check_tool mypy  "pip install mypy"
check_tool rustc "curl https://sh.rustup.rs -sSf | sh"

if [ ! -f "$RUST_SRC" ]; then
  echo "ERROR: Rust source '$RUST_SRC' not found." >&2
  exit 1
fi

# ── build (only when source is newer than binary) ──────────────────────────────
if [ ! -f "$BINARY" ] || [ "$RUST_SRC" -nt "$BINARY" ]; then
  echo "Building Rust binary…"
  rustc -O "$RUST_SRC" -o "$BINARY"
  echo "Build complete."
else
  echo "Rust binary is up to date, skipping build."
fi

# ── run ────────────────────────────────────────────────────────────────────────
echo "Running lint_report…"
"$BINARY" "$SRC_DIR" "$OUTPUT"
EXIT_CODE=$?

if [ $EXIT_CODE -eq 0 ]; then
  echo "✅ All checks passed. Report: $OUTPUT"
else
  echo "❌ Issues found. Report: $OUTPUT"
fi

exit $EXIT_CODE
