#!/usr/bin/env bash
# precc-ccc-hook.sh — PreToolUse hook that intercepts grep/rg commands
# and redirects them through cocoindex-code (ccc) for AST-aware semantic search.
#
# Saves token usage by returning precise, structure-aware code chunks
# instead of raw grep output. Logs savings to ~/.precc/ccc-metrics.jsonl.
#
# Protocol: reads Claude Code PreToolUse JSON on stdin, writes hook response to stdout.

set -euo pipefail

METRICS_FILE="${HOME}/.precc/ccc-metrics.jsonl"

# ── Read hook input ──────────────────────────────────────────────────────────
INPUT="$(cat)"
TOOL_NAME="$(echo "$INPUT" | jq -r '.tool_name // empty')"

# Only intercept Bash tool calls
if [[ "$TOOL_NAME" != "Bash" ]]; then
    exit 0
fi

COMMAND="$(echo "$INPUT" | jq -r '.tool_input.command // empty')"
CWD="$(echo "$INPUT" | jq -r '.cwd // empty')"

# ── Check if ccc is available ────────────────────────────────────────────────
if ! command -v ccc &>/dev/null; then
    exit 0
fi

# ── Detect grep/rg patterns worth redirecting ────────────────────────────────
# Match: grep -r "pattern" [path]
#        grep -rn "pattern" [path]
#        grep -rl "pattern" [path]
#        rg "pattern" [path]
#        rg -l "pattern" [path]
# Skip: grep on specific files (pipe, single file), complex pipelines
PATTERN=""
LANG_FILTER=""
PATH_FILTER=""

# Don't intercept piped commands or complex expressions
if echo "$COMMAND" | grep -qE '\||&&|;|>|<|\$\('; then
    exit 0
fi

# Extract search pattern from grep -r variants
if echo "$COMMAND" | grep -qE '^grep\s+(-[a-zA-Z]*r[a-zA-Z]*\s+)'; then
    # grep with -r flag (recursive)
    PATTERN="$(echo "$COMMAND" | sed -nE "s/^grep\s+(-[a-zA-Z]*\s+)*['\"]?([^'\"]+)['\"]?\s*.*$/\2/p")"
    # Try to extract path argument
    PATH_FILTER="$(echo "$COMMAND" | sed -nE "s/^grep\s+(-[a-zA-Z]*\s+)*['\"]?[^'\"]+['\"]?\s+(.+)$/\2/p")"
# Extract search pattern from rg
elif echo "$COMMAND" | grep -qE '^rg\s+'; then
    PATTERN="$(echo "$COMMAND" | sed -nE "s/^rg\s+(-[a-zA-Z]*\s+)*['\"]?([^'\"]+)['\"]?\s*.*$/\2/p")"
    PATH_FILTER="$(echo "$COMMAND" | sed -nE "s/^rg\s+(-[a-zA-Z]*\s+)*['\"]?[^'\"]+['\"]?\s+(.+)$/\2/p")"
fi

# No pattern extracted — let the original command run
if [[ -z "$PATTERN" ]]; then
    exit 0
fi

# Skip very short patterns (likely too broad for semantic search)
if [[ ${#PATTERN} -lt 4 ]]; then
    exit 0
fi

# ── Check if project is indexed ──────────────────────────────────────────────
if [[ -n "$CWD" ]] && [[ ! -d "${CWD}/.cocoindex_code" ]]; then
    exit 0
fi

# ── Run ccc search ───────────────────────────────────────────────────────────
CCC_ARGS=("search" "$PATTERN" "--limit" "10")

if [[ -n "$PATH_FILTER" ]] && [[ "$PATH_FILTER" != "." ]]; then
    CCC_ARGS+=("--path" "$PATH_FILTER")
fi

CCC_OUTPUT="$(cd "$CWD" && ccc "${CCC_ARGS[@]}" 2>/dev/null)" || {
    # ccc failed — fall through to original command
    exit 0
}

# If ccc returned nothing useful, let grep run
if [[ -z "$CCC_OUTPUT" ]] || [[ "$(echo "$CCC_OUTPUT" | wc -l)" -lt 2 ]]; then
    exit 0
fi

# ── Compare output sizes ────────────────────────────────────────────────────
# Run the original command to measure what grep would have returned
GREP_OUTPUT="$(cd "$CWD" && eval "$COMMAND" 2>/dev/null | head -500)" || GREP_OUTPUT=""

GREP_BYTES="${#GREP_OUTPUT}"
CCC_BYTES="${#CCC_OUTPUT}"

# Only redirect if ccc result is actually smaller or grep was very large
if [[ "$GREP_BYTES" -gt 0 ]] && [[ "$CCC_BYTES" -ge "$GREP_BYTES" ]]; then
    # ccc didn't save tokens — let original run
    exit 0
fi

# ── Log metrics ──────────────────────────────────────────────────────────────
mkdir -p "$(dirname "$METRICS_FILE")"
SAVED_BYTES=$(( GREP_BYTES - CCC_BYTES ))
TIMESTAMP="$(date -u +%Y-%m-%dT%H:%M:%SZ)"

echo "{\"ts\":\"$TIMESTAMP\",\"pattern\":\"$PATTERN\",\"grep_bytes\":$GREP_BYTES,\"ccc_bytes\":$CCC_BYTES,\"saved_bytes\":$SAVED_BYTES,\"cwd\":\"$CWD\"}" \
    >> "$METRICS_FILE" 2>/dev/null || true

# ── Rewrite the command ──────────────────────────────────────────────────────
# Replace grep/rg with an echo of the ccc output
# We use printf to safely handle the output
ESCAPED_OUTPUT="$(echo "$CCC_OUTPUT" | jq -Rs .)"

NEW_COMMAND="printf '%s\n' ${ESCAPED_OUTPUT}"

jq -n \
    --arg cmd "$NEW_COMMAND" \
    --arg reason "PRECC: redirected grep/rg to cocoindex-code semantic search (saved ${SAVED_BYTES} bytes)" \
    '{
        hookSpecificOutput: {
            hookEventName: "PreToolUse",
            permissionDecision: "allow",
            permissionDecisionReason: $reason,
            updatedInput: {
                command: $cmd
            }
        }
    }'
