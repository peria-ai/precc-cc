#!/usr/bin/env bash
# rtk_measurement_test.sh — End-to-end regression test for RTK compression
# and ground-truth token savings measurement.
#
# Tests the full pipeline:
#   1. PreToolUse rewrites `git status` → `rtk git status`
#   2. Stash file is created with original command
#   3. PostToolUse receives compressed output and measures ground truth
#   4. savings_measurements.jsonl records before/after tokens
#
# Prerequisites: rtk and precc-hook must be on PATH
# Usage: bash tests/rtk_measurement_test.sh

set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m'

PASS=0
FAIL=0
SKIP=0

pass() { echo -e "  ${GREEN}✓${NC} $1"; PASS=$((PASS+1)); }
fail() { echo -e "  ${RED}✗${NC} $1"; FAIL=$((FAIL+1)); }
skip() { echo -e "  ${YELLOW}⊘${NC} $1"; SKIP=$((SKIP+1)); }

DATA_DIR="${HOME}/.local/share/precc"
STASH_DIR="${DATA_DIR}/stash"
SAVINGS_LOG="${DATA_DIR}/savings_measurements.jsonl"
CWD="$(cd "$(dirname "$0")/.." && pwd)"

echo "=== RTK + Measurement Regression Test ==="
echo ""

# ── Prerequisites ──────────────────────────────────────────────────────────
echo "Prerequisites:"
if ! command -v rtk &>/dev/null; then
    skip "rtk not installed — skipping test"
    echo ""; echo "=== Results: ${SKIP} skipped ==="; exit 0
fi
pass "rtk $(rtk --version)"

if ! command -v precc-hook &>/dev/null; then
    skip "precc-hook not on PATH — skipping test"
    echo ""; echo "=== Results: ${SKIP} skipped ==="; exit 0
fi
pass "precc-hook on PATH"

# Clean up stash and savings log from previous runs
rm -rf "${STASH_DIR}"/*.json 2>/dev/null || true
SAVINGS_BEFORE=0
if [ -f "$SAVINGS_LOG" ]; then
    SAVINGS_BEFORE=$(wc -l < "$SAVINGS_LOG")
fi

echo ""

# ── Test 1: RTK rewrites git status ────────────────────────────────────────
echo "Test 1: RTK rewrite via PreToolUse"

PRETOOL_INPUT='{"hook_event_name":"PreToolUse","tool_name":"Bash","tool_input":{"command":"git status"},"cwd":"'"${CWD}"'"}'
PRETOOL_OUTPUT=$(echo "$PRETOOL_INPUT" | timeout 5 precc-hook 2>/dev/null || true)

if echo "$PRETOOL_OUTPUT" | grep -q '"rtk git status"'; then
    pass "git status → rtk git status"
else
    fail "RTK rewrite not applied"
    echo "    Output: ${PRETOOL_OUTPUT:0:200}"
fi

if echo "$PRETOOL_OUTPUT" | grep -q "rtk-rewrite"; then
    pass "permissionDecisionReason contains rtk-rewrite"
else
    fail "reason missing rtk-rewrite"
fi

if echo "$PRETOOL_OUTPUT" | grep -q 'original:'; then
    pass "reason includes original command"
else
    fail "reason missing original command"
fi

# ── Test 2: Stash file created ─────────────────────────────────────────────
echo ""
echo "Test 2: Measurement stash"

STASH_COUNT=$(ls "${STASH_DIR}"/*.json 2>/dev/null | wc -l || echo 0)
if [ "$STASH_COUNT" -ge 1 ]; then
    pass "stash file created (${STASH_COUNT} file(s))"
    STASH_FILE=$(ls "${STASH_DIR}"/*.json 2>/dev/null | head -1)
    STASH_CONTENT=$(cat "$STASH_FILE")

    if echo "$STASH_CONTENT" | python3 -c "import sys,json; d=json.load(sys.stdin); assert d['original_cmd']=='git status'" 2>/dev/null; then
        pass "stash.original_cmd = 'git status'"
    else
        fail "stash original_cmd mismatch"
    fi

    if echo "$STASH_CONTENT" | python3 -c "import sys,json; d=json.load(sys.stdin); assert 'rtk' in d['rewritten_cmd']" 2>/dev/null; then
        pass "stash.rewritten_cmd contains 'rtk'"
    else
        fail "stash rewritten_cmd missing rtk"
    fi
else
    fail "no stash file in ${STASH_DIR}"
fi

# ── Test 3: Actual output comparison ───────────────────────────────────────
echo ""
echo "Test 3: Output size comparison (manual)"

ORIG_OUTPUT=$(cd "$CWD" && git status 2>&1)
ORIG_BYTES=${#ORIG_OUTPUT}

RTK_OUTPUT=$(cd "$CWD" && rtk git status 2>&1)
RTK_BYTES=${#RTK_OUTPUT}

echo "  Original output: ${ORIG_BYTES} bytes (~$((ORIG_BYTES / 4)) tokens)"
echo "  RTK output:      ${RTK_BYTES} bytes (~$((RTK_BYTES / 4)) tokens)"

if [ "$ORIG_BYTES" -gt "$RTK_BYTES" ] && [ "$ORIG_BYTES" -gt 50 ]; then
    SAVED=$((ORIG_BYTES - RTK_BYTES))
    PCT=$(python3 -c "print(f'{($SAVED/$ORIG_BYTES)*100:.1f}')")
    pass "RTK reduced output by ${SAVED} bytes (${PCT}%)"
elif [ "$ORIG_BYTES" -le 50 ]; then
    skip "output too small to show compression (${ORIG_BYTES} bytes)"
else
    fail "RTK did not reduce output (orig=${ORIG_BYTES}, rtk=${RTK_BYTES})"
fi

# ── Test 4: PostToolUse ground-truth measurement ───────────────────────────
echo ""
echo "Test 4: PostToolUse ground-truth measurement"

# Simulate PostToolUse: Claude Code ran `rtk git status` and got RTK_OUTPUT
POSTTOOL_INPUT=$(python3 -c "
import json
rtk_out = open('/dev/stdin').read()
print(json.dumps({
    'hook_event_name': 'PostToolUse',
    'tool_name': 'Bash',
    'tool_input': {'command': 'rtk git status'},
    'tool_response': rtk_out,
    'cwd': '${CWD}',
    'session_id': 'regression-test-$$'
}))
" <<< "$RTK_OUTPUT")

echo "$POSTTOOL_INPUT" | timeout 30 precc-hook 2>/dev/null || true

# Wait briefly for file writes
sleep 1

# Check savings measurement
if [ -f "$SAVINGS_LOG" ]; then
    SAVINGS_AFTER=$(wc -l < "$SAVINGS_LOG")
    NEW_ENTRIES=$((SAVINGS_AFTER - SAVINGS_BEFORE))

    if [ "$NEW_ENTRIES" -ge 1 ]; then
        pass "savings measurement logged (${NEW_ENTRIES} new entries)"

        LAST=$(tail -1 "$SAVINGS_LOG")
        M_ORIG=$(echo "$LAST" | python3 -c "import sys,json; print(json.load(sys.stdin).get('original_output_tokens',0))")
        M_ACTUAL=$(echo "$LAST" | python3 -c "import sys,json; print(json.load(sys.stdin).get('actual_output_tokens',0))")
        M_SAVED=$(echo "$LAST" | python3 -c "import sys,json; print(json.load(sys.stdin).get('savings_tokens',0))")
        M_PCT=$(echo "$LAST" | python3 -c "import sys,json; print(json.load(sys.stdin).get('savings_pct',0))")
        M_METHOD=$(echo "$LAST" | python3 -c "import sys,json; print(json.load(sys.stdin).get('measurement_method',''))")
        M_CLASS=$(echo "$LAST" | python3 -c "import sys,json; print(json.load(sys.stdin).get('cmd_class',''))")

        echo ""
        echo "  ┌─────────────────────────────────────────────┐"
        echo "  │ Ground-Truth Measurement Result              │"
        echo "  ├─────────────────────────────────────────────┤"
        printf "  │ Original tokens (without PRECC): %-10s │\n" "$M_ORIG"
        printf "  │ Actual tokens   (with PRECC):    %-10s │\n" "$M_ACTUAL"
        printf "  │ Tokens saved:                    %-10s │\n" "$M_SAVED"
        printf "  │ Savings ratio:                   %-10s │\n" "${M_PCT}%"
        printf "  │ Method:                          %-10s │\n" "$M_METHOD"
        printf "  │ Command class:                   %-10s │\n" "$M_CLASS"
        echo "  └─────────────────────────────────────────────┘"
        echo ""

        [ "$M_METHOD" = "ground_truth" ] && pass "method = ground_truth" || fail "expected ground_truth, got ${M_METHOD}"
        [ "$M_CLASS" = "git status" ] && pass "class = git status" || fail "expected 'git status', got '${M_CLASS}'"
        [ "$M_ORIG" -gt 0 ] && pass "original tokens > 0" || fail "original tokens = 0"
    else
        fail "no new savings entries (before=${SAVINGS_BEFORE}, after=${SAVINGS_AFTER})"
    fi
else
    fail "savings_measurements.jsonl not found"
fi

# ── Test 5: Stash consumed ─────────────────────────────────────────────────
echo ""
echo "Test 5: Stash cleanup"

STASH_REMAINING=$(find "${STASH_DIR}" -name '*.json' 2>/dev/null | wc -l | tr -d ' ')
if [ "${STASH_REMAINING}" -eq 0 ]; then
    pass "stash file consumed by PostToolUse"
else
    fail "${STASH_REMAINING} stash file(s) remaining"
fi

# ── Summary ────────────────────────────────────────────────────────────────
echo ""
echo "=== Results ==="
TOTAL=$((PASS + FAIL + SKIP))
echo -e "  ${GREEN}${PASS} passed${NC}, ${RED}${FAIL} failed${NC}, ${YELLOW}${SKIP} skipped${NC} (${TOTAL} total)"

[ "$FAIL" -gt 0 ] && exit 1 || exit 0
