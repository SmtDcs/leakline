#!/usr/bin/env bash
# Show the three Leakline classes on local hex. No network. No keys.
set -euo pipefail
cd "$(dirname "$0")/.."

cargo build --offline --quiet
bin="./target/debug/leakline"

show() {
  local policy="$1"
  local file="$2"
  local want="$3"
  set +e
  local out
  out="$("$bin" --policy "$policy" "$file" 2>&1)"
  local code=$?
  set -e
  printf '\n# %s  --policy %s  exit %s (want %s)\n%s\n' "$file" "$policy" "$code" "$want" "$out"
  if [[ "$code" != "$want" ]]; then
    echo "demo: unexpected exit for $file" >&2
    exit 1
  fi
}

show FullPrivacy examples/transparent-v5.hex 1
show AllowRevealedRecipients examples/transparent-v5.hex 0
show FullPrivacy examples/crossing-v5.hex 1
show AllowRevealedAmounts examples/crossing-v5.hex 0
show FullPrivacy examples/orchard-fee-v5.hex 0
show NoPrivacy examples/unknown-v9.hex 1
show FullPrivacy examples/empty-v5.hex 0
show FullPrivacy examples/empty-nu6.pczt.hex 0

set +e
"$bin" examples/truncated.hex >/dev/null
code=$?
set -e
printf '\n# truncated header  exit %s (want 2)\n' "$code"
if [[ "$code" != "2" ]]; then
  echo "demo: truncated input should exit 2" >&2
  exit 1
fi

echo
echo "demo ok. nothing was uploaded."
