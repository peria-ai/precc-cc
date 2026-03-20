#!/usr/bin/env bash
# precc-ccc-savings.sh — Report token savings from grep→ccc redirections.
# Reads ~/.precc/ccc-metrics.jsonl and outputs a summary table.

set -euo pipefail

METRICS_FILE="${HOME}/.precc/ccc-metrics.jsonl"

if [[ ! -f "$METRICS_FILE" ]] || [[ ! -s "$METRICS_FILE" ]]; then
    echo "No cocoindex-code savings recorded yet."
    echo "Savings are logged when PRECC redirects grep/rg commands through ccc search."
    exit 0
fi

# ── Compute aggregates ──────────────────────────────────────────────────────
STATS="$(jq -s '
{
    total_intercepts: length,
    total_grep_bytes: (map(.grep_bytes) | add),
    total_ccc_bytes: (map(.ccc_bytes) | add),
    total_saved_bytes: (map(.saved_bytes) | add),
    first_ts: (map(.ts) | sort | first),
    last_ts: (map(.ts) | sort | last)
}' "$METRICS_FILE")"

TOTAL="$(echo "$STATS" | jq -r '.total_intercepts')"
GREP_BYTES="$(echo "$STATS" | jq -r '.total_grep_bytes')"
CCC_BYTES="$(echo "$STATS" | jq -r '.total_ccc_bytes')"
SAVED_BYTES="$(echo "$STATS" | jq -r '.total_saved_bytes')"
FIRST_TS="$(echo "$STATS" | jq -r '.first_ts')"
LAST_TS="$(echo "$STATS" | jq -r '.last_ts')"

# Estimate tokens (1 token ≈ 4 bytes)
SAVED_TOKENS=$(( SAVED_BYTES / 4 ))
GREP_TOKENS=$(( GREP_BYTES / 4 ))

if [[ "$GREP_BYTES" -gt 0 ]]; then
    PCT=$(( SAVED_BYTES * 100 / GREP_BYTES ))
else
    PCT=0
fi

# ── Format byte sizes ───────────────────────────────────────────────────────
fmt_bytes() {
    local b=$1
    if (( b >= 1048576 )); then
        echo "$(( b / 1048576 ))M"
    elif (( b >= 1024 )); then
        echo "$(( b / 1024 ))K"
    else
        echo "${b}B"
    fi
}

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
echo "│         PRECC × cocoindex-code Savings Report           │"
echo "├─────────────────────────────────────────────────────────┤"
printf "│  Period:           %-37s│\n" "${FIRST_TS} → ${LAST_TS}"
printf "│  grep/rg intercepted:  %-34s│\n" "$TOTAL"
printf "│  grep output (raw):    %-34s│\n" "$(fmt_bytes "$GREP_BYTES") (~$(fmt_tokens "$GREP_TOKENS") tokens)"
printf "│  ccc output (AST):     %-34s│\n" "$(fmt_bytes "$CCC_BYTES")"
printf "│  Bytes saved:          %-34s│\n" "$(fmt_bytes "$SAVED_BYTES") (${PCT}%%)"
printf "│  Est. tokens saved:    %-34s│\n" "~$(fmt_tokens "$SAVED_TOKENS")"
echo "├─────────────────────────────────────────────────────────┤"
echo "│  Recent redirections:                                   │"

# Show last 5 entries
tail -5 "$METRICS_FILE" | jq -r '
    "│    \(.ts[0:16])  \(.pattern[0:20] | . + " " * (20 - length))  saved \(.saved_bytes)B  │"
' 2>/dev/null || true

echo "└─────────────────────────────────────────────────────────┘"
