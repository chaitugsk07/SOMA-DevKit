
#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"

usage() { echo "usage: $0 [web|flutter|catalog]  (default: web)"; }

# Ensure git uses the repo's hooks/ dir (idempotent — only sets if not already set)
[ "$(git config --get core.hooksPath 2>/dev/null)" = "hooks" ] || git config core.hooksPath hooks

# Native target needed to run catalog (workspace default is wasm32)
NATIVE_TARGET=$(rustc -vV 2>/dev/null | sed -n 's/host: //p')

TARGET="${1:-web}"

case "$TARGET" in
  -h|--help|help)
    usage; exit 0 ;;
  web)
    rustup target list --installed 2>/dev/null | grep -q wasm32-unknown-unknown \
      || rustup target add wasm32-unknown-unknown
    command -v trunk >/dev/null \
      || { echo "trunk not found — install with: brew install trunk  (or: cargo install trunk)"; exit 1; }
    # Free any stale process on port 9876 (BSD-safe: no xargs -r)
    # shellcheck disable=SC2046
    pids=$(lsof -ti tcp:9876 2>/dev/null || true)
    [ -n "$pids" ] && kill -9 $pids 2>/dev/null || true
    cd web
    cargo run -q -p catalog --target "$NATIVE_TARGET"
    echo "soma-ui playground → http://127.0.0.1:9876/"
    exec trunk serve ;;
  catalog)
    cd web
    cargo run -q -p catalog --target "$NATIVE_TARGET"
    exit 0 ;;
  flutter)
    command -v flutter >/dev/null \
      || { echo "flutter not found — install from https://flutter.dev/docs/get-started/install"; exit 1; }
    echo "soma-ui Flutter gallery → launching on ${DEVICE:-chrome}"
    cd flutter/example
    exec flutter run -d "${DEVICE:-chrome}" ;;
  *)
    echo "unknown target: $TARGET"; usage; exit 1 ;;
esac
