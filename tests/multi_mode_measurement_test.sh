#!/usr/bin/env bash
# multi_mode_measurement_test.sh — Measure token savings across compression modes.
#
# For a set of read-only commands, runs each in 4 modes and compares output sizes:
#   1. basic     — no compression (original command)
#   2. rtk       — RTK wrapped
#   3. nushell   — nushell wrapped
#   4. lean-ctx  — lean-ctx wrapped
#
# Reports per-command savings table so we can see which compression mode wins
# for which command class.
#
# Usage: bash tests/multi_mode_measurement_test.sh

set -euo pipefail

CWD="$(cd "$(dirname "$0")/.." && pwd)"
cd "$CWD"

CYAN='\033[36m'; GREEN='\033[0;32m'; YELLOW='\033[0;33m'; NC='\033[0m'

# Tools available?
HAS_RTK=$(command -v rtk &>/dev/null && echo 1 || echo 0)
HAS_NU=$(command -v nu &>/dev/null && echo 1 || echo 0)
HAS_LEANCTX=$(command -v lean-ctx &>/dev/null && echo 1 || echo 0)

echo "=== Multi-Mode Token Savings Measurement ==="
echo "Tools available: rtk=${HAS_RTK} nushell=${HAS_NU} lean-ctx=${HAS_LEANCTX}"
echo ""

# Test commands (all read-only, all RTK-supported)
COMMANDS=(
    "git status"
    "git log --oneline -10"
    "git diff HEAD~1"
    "ls -la"
)

# Helper: measure output bytes of a command
measure_cmd() {
    local cmd="$1"
    timeout 15 bash -c "$cmd" 2>&1 | wc -c
}

# Header
printf "%-30s | %10s | %10s | %10s | %10s\n" "Command" "Basic (B)" "RTK (B)" "Nushell (B)" "Lean-ctx (B)"
printf "%-30s-+-%10s-+-%10s-+-%10s-+-%10s\n" "$(printf '%.s-' {1..30})" "----------" "----------" "----------" "----------"

# Per-mode totals for summary
TOTAL_BASIC=0
TOTAL_RTK=0
TOTAL_NU=0
TOTAL_LEANCTX=0

# Results CSV
RESULTS_FILE="/tmp/multi_mode_results.csv"
echo "command,basic,rtk,nushell,lean_ctx,rtk_pct,nushell_pct,lean_ctx_pct" > "$RESULTS_FILE"

for cmd in "${COMMANDS[@]}"; do
    # 1. Basic
    BASIC=$(measure_cmd "$cmd")

    # 2. RTK
    if [ "$HAS_RTK" = "1" ]; then
        RTK=$(measure_cmd "rtk $cmd" 2>/dev/null || echo "$BASIC")
    else
        RTK="-"
    fi

    # 3. Nushell — only certain commands have nushell rules
    if [ "$HAS_NU" = "1" ]; then
        # Use precc-hook to get the nushell-wrapped version
        WRAPPED=$(echo "{\"tool_name\":\"Bash\",\"tool_input\":{\"command\":\"$cmd\"},\"cwd\":\"${CWD}\"}" | \
            env PRECC_NUSHELL=1 PRECC_LEAN_CTX=0 timeout 5 precc-hook 2>/dev/null | \
            python3 -c "import sys,json; d=json.load(sys.stdin); print(d.get('hookSpecificOutput',{}).get('updatedInput',{}).get('command',''))" 2>/dev/null || true)
        if [ -n "$WRAPPED" ] && echo "$WRAPPED" | grep -qE "nu |--message-format|--porcelain|-json"; then
            NU=$(measure_cmd "$WRAPPED" 2>/dev/null || echo "$BASIC")
        else
            NU="(no rule)"
        fi
    else
        NU="-"
    fi

    # 4. Lean-ctx
    if [ "$HAS_LEANCTX" = "1" ]; then
        LEANCTX=$(measure_cmd "lean-ctx -c '$cmd'" 2>/dev/null || echo "$BASIC")
    else
        LEANCTX="-"
    fi

    # Print row
    printf "%-30s | %10s | %10s | %10s | %10s\n" "$cmd" "$BASIC" "$RTK" "$NU" "$LEANCTX"

    # Compute percentages and append to CSV
    RTK_PCT="-"
    if [[ "$RTK" =~ ^[0-9]+$ ]] && [ "$BASIC" -gt 0 ]; then
        RTK_PCT=$(python3 -c "print(f'{((${BASIC}-${RTK})/${BASIC})*100:.1f}')")
        TOTAL_BASIC=$((TOTAL_BASIC + BASIC))
        TOTAL_RTK=$((TOTAL_RTK + RTK))
    fi
    NU_PCT="-"
    if [[ "$NU" =~ ^[0-9]+$ ]] && [ "$BASIC" -gt 0 ]; then
        NU_PCT=$(python3 -c "print(f'{((${BASIC}-${NU})/${BASIC})*100:.1f}')")
        TOTAL_NU=$((TOTAL_NU + NU))
    fi
    LEANCTX_PCT="-"
    if [[ "$LEANCTX" =~ ^[0-9]+$ ]] && [ "$BASIC" -gt 0 ]; then
        LEANCTX_PCT=$(python3 -c "print(f'{((${BASIC}-${LEANCTX})/${BASIC})*100:.1f}')")
        TOTAL_LEANCTX=$((TOTAL_LEANCTX + LEANCTX))
    fi

    echo "${cmd},${BASIC},${RTK},${NU},${LEANCTX},${RTK_PCT},${NU_PCT},${LEANCTX_PCT}" >> "$RESULTS_FILE"
done

echo ""
echo "── Savings percentages ──"
printf "%-30s | %10s | %10s | %10s\n" "Command" "RTK %" "Nushell %" "Lean-ctx %"
printf "%-30s-+-%10s-+-%10s-+-%10s\n" "$(printf '%.s-' {1..30})" "----------" "----------" "----------"
tail -n +2 "$RESULTS_FILE" | while IFS=, read -r cmd basic rtk nu lc rtk_pct nu_pct lc_pct; do
    printf "%-30s | %9s%% | %9s%% | %9s%%\n" "$cmd" "$rtk_pct" "$nu_pct" "$lc_pct"
done

echo ""
echo "── Aggregate totals ──"
printf "%-15s : %10d bytes\n" "Basic total" "$TOTAL_BASIC"
[ "$TOTAL_RTK" -gt 0 ] && printf "%-15s : %10d bytes (%.1f%% saved vs basic)\n" "RTK total" "$TOTAL_RTK" \
    "$(python3 -c "print((($TOTAL_BASIC-$TOTAL_RTK)/$TOTAL_BASIC)*100)")"
[ "$TOTAL_NU" -gt 0 ] && printf "%-15s : %10d bytes (%.1f%% saved vs basic)\n" "Nushell total" "$TOTAL_NU" \
    "$(python3 -c "print((($TOTAL_BASIC-$TOTAL_NU)/$TOTAL_BASIC)*100)")"
[ "$TOTAL_LEANCTX" -gt 0 ] && printf "%-15s : %10d bytes (%.1f%% saved vs basic)\n" "Lean-ctx total" "$TOTAL_LEANCTX" \
    "$(python3 -c "print((($TOTAL_BASIC-$TOTAL_LEANCTX)/$TOTAL_BASIC)*100)")"

echo ""
echo "Results CSV: ${RESULTS_FILE}"
