#!/usr/bin/env bash
# Regenerate style/soma-ui.css from the Tailwind preset and component sources.
# Run this whenever you add new Tailwind classes to any .rs file in src/.
#
# Requires: Node.js (npx is used to pull tailwindcss@3 on demand)
# Must be run from the repo root (soma-ui/web/) or with adjusted paths below.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"   # soma-ui/web/
UI_DIR="$SCRIPT_DIR"                            # packages/ui/
THEME_DIR="$REPO_ROOT/theme"
OUT="$UI_DIR/style/soma-ui.css"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

# ── Tailwind config (references the shared preset) ───────────────────────────
cat > "$TMP/tw.config.js" << JSEOF
const preset = require('$THEME_DIR/tailwind.preset.js');
module.exports = {
  presets: [preset],
  darkMode: 'class',
  content: ['$UI_DIR/src/**/*.rs'],
};
JSEOF

# ── Input CSS: Google Fonts @import + design tokens + Tailwind layers ─────────
# The Google Fonts @import replaces the self-hosted @font-face rules so no
# binary woff2 assets need to ship in the crate.
python3 - << PYEOF
import re, pathlib

tokens = pathlib.Path('$THEME_DIR/tokens.css').read_text()
# Strip @font-face blocks (replaced by Google Fonts @import below)
tokens = re.sub(r'@font-face\s*\{[^}]*\}', '', tokens)
tokens = re.sub(r'/\* [─]+ Self-hosted fonts [─]+ \*/\n?', '', tokens)
tokens = re.sub(r'\n{3,}', '\n\n', tokens).strip()

out = pathlib.Path('$TMP/tw.input.css')
out.write_text(
    "@import url('https://fonts.googleapis.com/css2?family=Outfit:wght@400;500;600;700&family=Rajdhani:wght@400;500;600;700&display=swap');\n\n"
    + tokens
    + "\n\n@tailwind base;\n@tailwind components;\n@tailwind utilities;\n"
)
PYEOF

# ── Run Tailwind ──────────────────────────────────────────────────────────────
npx -y tailwindcss@3 \
  -c "$TMP/tw.config.js" \
  -i "$TMP/tw.input.css" \
  -o "$OUT" \
  --minify

echo "Wrote $OUT ($(wc -c < "$OUT") bytes)"
