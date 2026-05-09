#!/usr/bin/env bash
# faith — agent-first Bible CLI demo
# Run with:  bash scripts/demo.sh
# Record with: asciinema rec -c "bash scripts/demo.sh" demo.cast

set -uo pipefail

if ! command -v faith >/dev/null 2>&1; then
  echo "error: 'faith' not on PATH. Run 'cargo install faith' (or 'cargo install --path .' from the repo) first." >&2
  exit 127
fi
if ! command -v jq >/dev/null 2>&1; then
  echo "error: 'jq' not on PATH. Install with 'brew install jq'." >&2
  exit 127
fi

# Speed knobs (override via env)
TYPE_DELAY="${TYPE_DELAY:-0.020}"   # per-char delay when typing commands
PAUSE_BEFORE="${PAUSE_BEFORE:-0.6}" # pause before each command runs
PAUSE_AFTER="${PAUSE_AFTER:-1.2}"   # pause after each command output

# Colors (asciinema renders ANSI nicely)
BOLD='\033[1m'
DIM='\033[2m'
GREEN='\033[32m'
CYAN='\033[36m'
YELLOW='\033[33m'
RESET='\033[0m'

prompt() { printf "${GREEN}${BOLD}\$${RESET} "; }

# Type a command character-by-character for a "live" feel, then run it.
run() {
  local cmd="$1"
  prompt
  local i=0
  while [ $i -lt ${#cmd} ]; do
    printf "%s" "${cmd:$i:1}"
    sleep "$TYPE_DELAY"
    i=$((i + 1))
  done
  printf "\n"
  sleep "$PAUSE_BEFORE"
  bash -c "$cmd"
  sleep "$PAUSE_AFTER"
}

caption() {
  printf "\n${DIM}# %s${RESET}\n" "$1"
  sleep 0.4
}

clear

printf "${BOLD}${CYAN}"
cat <<'BANNER'
   __       _ _   _
  / _| __ _(_) |_| |__
 | |_ / _` | | __| '_ \      The Bible. For agents.
 |  _| (_| | | |_| | | |     Universal. Open.
 |_|  \__,_|_|\__|_| |_|
BANNER
printf "${RESET}\n"
sleep 1.2

caption "1) Discover capabilities — agents call this on startup."
run 'faith manifest | jq "{schema, version, translations: [.translations[].id], tools: [.tools[].name]}"'

caption "2) Single verse, English (KJV)."
run 'faith get "John 3:16" --tr KJV | jq'

caption "3) Same verse, Portuguese (ONBV) — multi-locale by design."
run 'faith get "João 3:16" --tr ONBV | jq'

caption "4) Two translations in one call (parallel, deterministic order)."
run 'faith get "Romans 8:28" --tr KJV,ONBV | jq'

caption "5) Batch: 3 refs, 1 process, 1 JSON array — token-efficient for agent loops."
run "echo '[\"John 3:16\",\"Ps 23:1\",\"Romans 8:28\"]' | faith batch --tr KJV | jq"

printf "\n${BOLD}${YELLOW}→ cargo install faith${RESET}\n"
printf "${DIM}  https://github.com/V-Gutierrez/faith${RESET}\n\n"
sleep 1.5
