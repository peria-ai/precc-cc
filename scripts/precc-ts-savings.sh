#!/usr/bin/env bash
# precc-ts-savings.sh — Report token savings from context file compression.
# Reads ~/.precc/ts-metrics.jsonl and outputs a summary.

set -euo pipefail

METRICS_FILE="${HOME}/.precc/ts-metrics.jsonl"

if [[ ! -f "$METRICS_FILE" ]] || [[ ! -s "$METRICS_FILE" ]]; then
    echo "No context compression savings recorded yet."
    echo "Run 'node precc-ts-compress.js' in your project to compress CLAUDE.md and memory files."
    exit 0
fi

# ── Compute aggregates ──────────────────────────────────────────────────────
STATS="$(jq -s '
{
    total_runs: length,
    total_files: (map(.files) | add),
    total_original_tokens: (map(.total_original_tokens) | add),
    total_compressed_tokens: (map(.total_compressed_tokens) | add),
    total_saved_tokens: (map(.total_saved_tokens) | add),
    first_ts: (map(.ts) | sort | first),
    last_ts: (map(.ts) | sort | last)
}' "$METRICS_FILE")"

TOTAL_RUNS="$(echo "$STATS" | jq -r '.total_runs')"
TOTAL_FILES="$(echo "$STATS" | jq -r '.total_files')"
ORIG_TOKENS="$(echo "$STATS" | jq -r '.total_original_tokens')"
SAVED_TOKENS="$(echo "$STATS" | jq -r '.total_saved_tokens')"
FIRST_TS="$(echo "$STATS" | jq -r '.first_ts')"
LAST_TS="$(echo "$STATS" | jq -r '.last_ts')"

if [[ "$ORIG_TOKENS" -gt 0 ]]; then
    PCT=$(( SAVED_TOKENS * 100 / ORIG_TOKENS ))
else
    PCT=0
fi

# Estimate cost savings ($3/MTok input for Sonnet, every API call loads these files)
# Assume ~50 API calls/session, context files loaded each time
SAVED_COST="$(echo "scale=2; $SAVED_TOKENS * 50 * 0.003 / 1000000" | bc 2>/dev/null || echo "?")"

# ── Format tokens ───────────────────────────────────────────────────────────
fmt_tokens() {
    local t=$1
    if (( t >= 1000000 )); then
        printf "%.1fM" "$(echo "scale=1; $t / 1000000" | bc)"
    elif (( t >= 1000 )); then
        printf "%.1fK" "$(echo "scale=1; $t / 1000" | bc)"
    else
        echo "$t"
    fi
}

# ── Print report ─────────────────────────────────────────────────────────────
echo "┌─────────────────────────────────────────────────────────┐"
echo "│       PRECC × token-saver Context Compression           │"
echo "├─────────────────────────────────────────────────────────┤"
printf "│  Period:             %-36s│\n" "${FIRST_TS%T*} → ${LAST_TS%T*}"
printf "│  Compression runs:   %-36s│\n" "$TOTAL_RUNS"
printf "│  Files compressed:   %-36s│\n" "$TOTAL_FILES"
printf "│  Original tokens:    %-36s│\n" "$(fmt_tokens "$ORIG_TOKENS")"
printf "│  Tokens saved:       %-36s│\n" "$(fmt_tokens "$SAVED_TOKENS") (${PCT}%%)"
printf "│  Est. cost saved:    %-36s│\n" "\$${SAVED_COST}/session (×50 API calls)"
echo "├─────────────────────────────────────────────────────────┤"
echo "│  Last compression:                                      │"

# Show details from most recent run
tail -1 "$METRICS_FILE" | jq -r '
    .details[] |
    "│    \(.file)  saved \(.saved) tokens (\(.pct)%)                      │"
' 2>/dev/null | head -5 || true

echo "└─────────────────────────────────────────────────────────┘"
