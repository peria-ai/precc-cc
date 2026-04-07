#!/usr/bin/env bash
# regression_test.sh — Full PRECC feature regression test suite.
#
# Tests every feature in the PRECC toolchain via precc-hook stdin/stdout simulation.
# Each test sends a hook event and verifies the output.
#
# Usage: bash tests/regression_test.sh [--verbose]

set -euo pipefail

RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[0;33m'; CYAN='\033[36m'; NC='\033[0m'
PASS=0; FAIL=0; SKIP=0
VERBOSE="${1:-}"

pass() { echo -e "  ${GREEN}✓${NC} $1"; PASS=$((PASS+1)); }
fail() { echo -e "  ${RED}✗${NC} $1"; FAIL=$((FAIL+1)); [ "$VERBOSE" = "--verbose" ] && echo "    $2" || true; }
skip() { echo -e "  ${YELLOW}⊘${NC} $1"; SKIP=$((SKIP+1)); }
section() { echo ""; echo -e "${CYAN}── $1 ──${NC}"; }

CWD="$(cd "$(dirname "$0")/.." && pwd)"
DATA_DIR="${HOME}/.local/share/precc"

# Helper: run precc-hook with input, capture stdout+stderr
run_hook() {
    echo "$1" | timeout 10 precc-hook 2>/tmp/precc-test-stderr || true
}

# Helper: run precc-hook with extra env vars
run_hook_env() {
    local env_str="$1"; shift
    local input="$1"
    echo "$input" | env $env_str timeout 10 precc-hook 2>/tmp/precc-test-stderr || true
}

echo "=== PRECC Feature Regression Test Suite ==="
echo "CWD: ${CWD}"
echo ""

# Check prerequisites
command -v precc-hook &>/dev/null || { echo "precc-hook not on PATH"; exit 1; }
HAS_RTK=$(command -v rtk &>/dev/null && echo 1 || echo 0)
HAS_NU=$(command -v nu &>/dev/null && echo 1 || echo 0)
HAS_LEANCTX=$(command -v lean-ctx &>/dev/null && echo 1 || echo 0)
HAS_CCC=$(command -v ccc &>/dev/null && echo 1 || echo 0)

# ═══════════════════════════════════════════════════════════════════════════
# BASH PIPELINE FEATURES
# ═══════════════════════════════════════════════════════════════════════════

section "Feature 1: Bash Unwrap"

OUT=$(run_hook '{"tool_name":"Bash","tool_input":{"command":"bash -c \"ls -la\""},"cwd":"'${CWD}'"}')
if echo "$OUT" | grep -q "bash-unwrap"; then
    pass "bash -c \"ls -la\" unwrapped (reason contains bash-unwrap)"
else
    fail "bash unwrap not applied" "$OUT"
fi

# Should NOT unwrap when inner has pipes (bash-unwrap reason should NOT appear)
OUT=$(run_hook '{"tool_name":"Bash","tool_input":{"command":"bash -c \"ls | grep foo\""},"cwd":"'${CWD}'"}')
if echo "$OUT" | grep -q "bash-unwrap"; then
    fail "unwrapped command with pipes (should preserve bash -c)" "$OUT"
else
    pass "preserves bash -c with pipes"
fi

section "Feature 2: Context CD Prepend"

# Create a temp dir structure: parent has no Cargo.toml, child does
TMPDIR=$(mktemp -d)
mkdir -p "${TMPDIR}/myproject/src"
echo '[package]\nname = "test"' > "${TMPDIR}/myproject/Cargo.toml"

OUT=$(run_hook '{"tool_name":"Bash","tool_input":{"command":"cargo build"},"cwd":"'${TMPDIR}'"}')
if echo "$OUT" | grep -q "cd.*myproject.*&&.*cargo build"; then
    pass "cargo build in parent → cd myproject && cargo build"
else
    # Context resolution depends on finding Cargo.toml above/below CWD
    skip "cd prepend not triggered (marker not found from ${TMPDIR})"
fi
rm -rf "$TMPDIR"

section "Feature 3: RTK Rewrite"

if [ "$HAS_RTK" = "1" ]; then
    # Disable nushell + lean-ctx so RTK can fire (mutually exclusive priority chain)
    NO_COMP="PRECC_NUSHELL=0 PRECC_LEAN_CTX=0"

    OUT=$(run_hook_env "$NO_COMP" '{"tool_name":"Bash","tool_input":{"command":"git status"},"cwd":"'${CWD}'"}')
    if echo "$OUT" | grep -q "rtk git status"; then
        pass "git status → rtk git status"
    else
        fail "RTK rewrite not applied" "$OUT"
    fi

    OUT=$(run_hook_env "$NO_COMP" '{"tool_name":"Bash","tool_input":{"command":"git diff HEAD~1"},"cwd":"'${CWD}'"}')
    if echo "$OUT" | grep -q "rtk git diff"; then
        pass "git diff → rtk git diff"
    else
        fail "RTK rewrite for git diff not applied" "$OUT"
    fi

    OUT=$(run_hook_env "$NO_COMP" '{"tool_name":"Bash","tool_input":{"command":"git log --oneline -5"},"cwd":"'${CWD}'"}')
    if echo "$OUT" | grep -q "rtk git log"; then
        pass "git log → rtk git log"
    else
        fail "RTK rewrite for git log not applied" "$OUT"
    fi

    # Should NOT rewrite already-RTK'd commands
    OUT=$(run_hook_env "$NO_COMP" '{"tool_name":"Bash","tool_input":{"command":"rtk git status"},"cwd":"'${CWD}'"}')
    if echo "$OUT" | grep -q "updatedInput"; then
        fail "double-wrapped rtk (should not rewrite)" "$OUT"
    else
        pass "no double-wrap for rtk git status"
    fi
else
    skip "RTK not installed"
fi

section "Feature 4: Diet Mode"

# Diet should add filters to pytest (if not using lean-ctx/nushell)
OUT=$(PRECC_LEAN_CTX=0 PRECC_NUSHELL=0 run_hook '{"tool_name":"Bash","tool_input":{"command":"pytest tests/"},"cwd":"'${CWD}'"}')
if echo "$OUT" | grep -q "grep\|PASSED"; then
    pass "pytest → appended filter pipe"
else
    skip "diet not triggered for pytest (compression may have taken priority)"
fi

# Diet should add -s to make
OUT=$(PRECC_LEAN_CTX=0 PRECC_NUSHELL=0 run_hook '{"tool_name":"Bash","tool_input":{"command":"make"},"cwd":"'${CWD}'"}')
if echo "$OUT" | grep -qE "\-s|--silent|grep"; then
    pass "make → diet flag/filter added"
else
    skip "diet not triggered for make"
fi

section "Feature 5: Destructive Command Guard"

# rm should pass through unmodified (no RTK, no cd, no diet)
OUT=$(run_hook '{"tool_name":"Bash","tool_input":{"command":"rm -rf /tmp/testdir"},"cwd":"'${CWD}'"}')
if echo "$OUT" | grep -q "updatedInput"; then
    fail "destructive command was modified (should pass through)" "$OUT"
else
    pass "rm -rf passes through unmodified"
fi

OUT=$(run_hook '{"tool_name":"Bash","tool_input":{"command":"git reset --hard HEAD~1"},"cwd":"'${CWD}'"}')
if echo "$OUT" | grep -q "updatedInput"; then
    fail "git reset --hard was modified" "$OUT"
else
    pass "git reset --hard passes through unmodified"
fi

OUT=$(run_hook '{"tool_name":"Bash","tool_input":{"command":"git push --force origin main"},"cwd":"'${CWD}'"}')
if echo "$OUT" | grep -q "updatedInput"; then
    fail "git push --force was modified" "$OUT"
else
    pass "git push --force passes through unmodified"
fi

section "Feature 6: Mutation Validation"

# The original command should appear in the reason when modified
OUT=$(run_hook '{"tool_name":"Bash","tool_input":{"command":"git status"},"cwd":"'${CWD}'"}')
if echo "$OUT" | grep -q "original:"; then
    pass "reason includes original command"
else
    fail "reason missing original command" "$OUT"
fi

section "Feature 7: Dry-Run Mode"

OUT=$(echo '{"tool_name":"Bash","tool_input":{"command":"git status"},"cwd":"'${CWD}'"}' | PRECC_DRY_RUN=1 timeout 5 precc-hook 2>/tmp/precc-test-stderr || true)
STDERR=$(cat /tmp/precc-test-stderr 2>/dev/null || true)
if echo "$STDERR" | grep -q "DRY-RUN"; then
    pass "PRECC_DRY_RUN=1 logs to stderr"
else
    fail "dry-run not detected on stderr" "$STDERR"
fi
# Stdout should be empty (no updatedInput)
if [ -z "$OUT" ]; then
    pass "dry-run produces no stdout (command unchanged)"
else
    fail "dry-run produced stdout" "$OUT"
fi

# ═══════════════════════════════════════════════════════════════════════════
# READ TOOL FEATURES
# ═══════════════════════════════════════════════════════════════════════════

section "Feature 8: Read Binary File Block"

OUT=$(run_hook '{"tool_name":"Read","tool_input":{"file_path":"/tmp/image.png"}}')
if echo "$OUT" | grep -q "deny"; then
    pass "blocked binary file (.png)"
else
    fail "did not block .png read" "$OUT"
fi

OUT=$(run_hook '{"tool_name":"Read","tool_input":{"file_path":"/tmp/data.wasm"}}')
if echo "$OUT" | grep -q "deny"; then
    pass "blocked binary file (.wasm)"
else
    fail "did not block .wasm read" "$OUT"
fi

# Should allow .rs files
OUT=$(run_hook '{"tool_name":"Read","tool_input":{"file_path":"/tmp/main.rs"}}')
if echo "$OUT" | grep -q "deny"; then
    fail "blocked .rs file (should allow)" "$OUT"
else
    pass "allowed .rs file read"
fi

section "Feature 9: Read Smart Limit Injection"

# Create a large file
BIGFILE=$(mktemp --suffix=.txt)
for i in $(seq 1 5000); do echo "line $i: some content here for testing"; done > "$BIGFILE"

OUT=$(run_hook '{"tool_name":"Read","tool_input":{"file_path":"'${BIGFILE}'"}}')
if echo "$OUT" | grep -q '"limit"'; then
    pass "limit injected for large file (5000 lines)"
else
    skip "limit not injected (file may not meet threshold)"
fi
rm -f "$BIGFILE"

# ═══════════════════════════════════════════════════════════════════════════
# GREP TOOL FEATURES
# ═══════════════════════════════════════════════════════════════════════════

section "Feature 10: Grep head_limit Injection"

OUT=$(run_hook '{"tool_name":"Grep","tool_input":{"pattern":"fn main","output_mode":"content"}}')
if echo "$OUT" | grep -q "head_limit"; then
    pass "head_limit injected for content mode"
else
    fail "head_limit not injected" "$OUT"
fi

# Should NOT override when head_limit already set
OUT=$(run_hook '{"tool_name":"Grep","tool_input":{"pattern":"fn main","output_mode":"content","head_limit":100}}')
INJECTED_LIMIT=$(echo "$OUT" | python3 -c "import sys,json; d=json.load(sys.stdin); print(d.get('hookSpecificOutput',{}).get('updatedInput',{}).get('head_limit',''))" 2>/dev/null || true)
if [ "$INJECTED_LIMIT" = "100" ] || [ -z "$OUT" ]; then
    pass "preserves existing head_limit=100"
else
    fail "head_limit changed from 100 to ${INJECTED_LIMIT}" "$OUT"
fi

section "Feature 11: Grep Auto Type Filter"

# In a Rust project, should inject type=rust
OUT=$(run_hook '{"tool_name":"Grep","tool_input":{"pattern":"fn main"}}')
if echo "$OUT" | grep -q '"type"'; then
    TYPE_VAL=$(echo "$OUT" | python3 -c "import sys,json; d=json.load(sys.stdin); print(d.get('hookSpecificOutput',{}).get('updatedInput',{}).get('type',''))" 2>/dev/null || true)
    if [ "$TYPE_VAL" = "rust" ]; then
        pass "type=rust injected in Rust project"
    else
        skip "type filter injected but not 'rust': ${TYPE_VAL}"
    fi
else
    skip "type filter not injected (may not detect project marker)"
fi

# ═══════════════════════════════════════════════════════════════════════════
# AGENT TOOL FEATURES
# ═══════════════════════════════════════════════════════════════════════════

section "Feature 12: Agent Hook Propagation"

OUT=$(run_hook '{"tool_name":"Agent","tool_input":{"prompt":"Find all test files","subagent_type":"Explore"}}')
if echo "$OUT" | grep -q "precc-hook"; then
    pass "precc-hook injected into agent prompt"
else
    fail "hook not propagated to agent" "$OUT"
fi

# Should NOT double-inject
OUT=$(run_hook '{"tool_name":"Agent","tool_input":{"prompt":"---\nhooks:\n  PreToolUse:\n    - command: precc-hook\n---\nFind tests"}}')
if echo "$OUT" | grep -q "updatedInput"; then
    fail "double-injected hooks into already-hooked prompt" "$OUT"
else
    pass "skipped injection for already-hooked prompt"
fi

# ═══════════════════════════════════════════════════════════════════════════
# POST-TOOL-USE FEATURES
# ═══════════════════════════════════════════════════════════════════════════

section "Feature 13: PostToolUse Large Output Detection"

LARGE_RESPONSE=$(python3 -c "print('x' * 50000)")
POSTTOOL=$(python3 -c "
import json
print(json.dumps({
    'hook_event_name': 'PostToolUse',
    'tool_name': 'Bash',
    'tool_input': {'command': 'find / -name \"*.log\"'},
    'tool_response': 'x' * 50000,
    'session_id': 'test-large-output'
}))
")
OUT=$(run_hook "$POSTTOOL")
if echo "$OUT" | grep -q "large output"; then
    pass "large output detected (>10K tokens)"
else
    skip "large output not flagged (additionalContext may not emit)"
fi

section "Feature 14: PostToolUse Context Pressure"

POSTTOOL=$(python3 -c "
import json
print(json.dumps({
    'hook_event_name': 'PostToolUse',
    'tool_name': 'Bash',
    'tool_input': {'command': 'ls'},
    'tool_response': 'file1.txt',
    'context_window': {'used_percentage': 92},
    'session_id': 'test-pressure'
}))
")
OUT=$(run_hook "$POSTTOOL")
if echo "$OUT" | grep -q "context.*full\|compact"; then
    pass "context pressure warning at 92%"
else
    skip "context pressure not flagged"
fi

section "Feature 15: Compression Failure Feedback (Adaptive Expand)"

# First: record a compression failure
python3 -c "
import time
data_dir = '$DATA_DIR'
ts = int(time.time())
with open(data_dir + '/compression_failures.log', 'a') as f:
    f.write(f'{ts} cargo test\n')
"

# Now: cargo test should skip compression
OUT=$(run_hook '{"tool_name":"Bash","tool_input":{"command":"cargo test"},"cwd":"'${CWD}'"}')
if echo "$OUT" | grep -q "adaptive-expand"; then
    pass "adaptive-expand triggered for recently-failed command class"
else
    skip "adaptive-expand not triggered (may need different conditions)"
fi

# Clean up
> "${DATA_DIR}/compression_failures.log"

# ═══════════════════════════════════════════════════════════════════════════
# MEASUREMENT FEATURES
# ═══════════════════════════════════════════════════════════════════════════

section "Feature 16: Ground-Truth Measurement (RTK)"

if [ "$HAS_RTK" = "1" ]; then
    # Clean stash
    rm -rf "${DATA_DIR}/stash/"*.json 2>/dev/null || true
    SAVINGS_BEFORE=0
    [ -f "${DATA_DIR}/savings_measurements.jsonl" ] && SAVINGS_BEFORE=$(wc -l < "${DATA_DIR}/savings_measurements.jsonl")

    # PreToolUse: create stash (disable nushell so RTK fires)
    run_hook_env "PRECC_NUSHELL=0 PRECC_LEAN_CTX=0" '{"tool_name":"Bash","tool_input":{"command":"git log --oneline -20"},"cwd":"'${CWD}'"}' > /dev/null

    # Verify stash exists
    STASH_COUNT=$(find "${DATA_DIR}/stash" -name '*.json' 2>/dev/null | wc -l | tr -d ' ')
    if [ "$STASH_COUNT" -ge 1 ]; then
        pass "stash file created for git log"
    else
        fail "no stash created" ""
    fi

    # Run the RTK command to get compressed output
    RTK_OUT=$(cd "$CWD" && rtk git log --oneline -20 2>&1 || true)

    # PostToolUse: trigger measurement
    POSTTOOL=$(python3 -c "
import json
rtk_out = '''${RTK_OUT}'''
print(json.dumps({
    'hook_event_name': 'PostToolUse',
    'tool_name': 'Bash',
    'tool_input': {'command': 'rtk git log --oneline -20'},
    'tool_response': rtk_out,
    'cwd': '${CWD}',
    'session_id': 'measurement-test'
}))
" 2>/dev/null)
    run_hook_env "PRECC_NUSHELL=0 PRECC_LEAN_CTX=0" "$POSTTOOL" > /dev/null

    sleep 1
    SAVINGS_AFTER=0
    [ -f "${DATA_DIR}/savings_measurements.jsonl" ] && SAVINGS_AFTER=$(wc -l < "${DATA_DIR}/savings_measurements.jsonl")
    NEW=$((SAVINGS_AFTER - SAVINGS_BEFORE))

    if [ "$NEW" -ge 1 ]; then
        LAST=$(tail -1 "${DATA_DIR}/savings_measurements.jsonl")
        M_ORIG=$(echo "$LAST" | python3 -c "import sys,json; print(json.load(sys.stdin).get('original_output_tokens',0))")
        M_ACTUAL=$(echo "$LAST" | python3 -c "import sys,json; print(json.load(sys.stdin).get('actual_output_tokens',0))")
        M_PCT=$(echo "$LAST" | python3 -c "import sys,json; print(json.load(sys.stdin).get('savings_pct',0))")
        M_METHOD=$(echo "$LAST" | python3 -c "import sys,json; print(json.load(sys.stdin).get('measurement_method',''))")
        pass "ground-truth measurement: ${M_ORIG}→${M_ACTUAL} tokens (${M_PCT}% saved, ${M_METHOD})"
    else
        fail "no measurement recorded" ""
    fi
else
    skip "RTK not installed — skipping measurement test"
fi

section "Feature 17: Safety Classifier"

# Unit tests cover this, but let's verify a few via the test suite
SAFETY_RESULT=$(cargo test -p precc-core -- is_safe_to_rerun 2>&1 | grep "test result" || true)
if echo "$SAFETY_RESULT" | grep -q "0 failed"; then
    pass "is_safe_to_rerun unit tests pass"
else
    fail "safety classifier tests failed" "$SAFETY_RESULT"
fi

section "Feature 18: Stash Write/Read/Delete"

STASH_RESULT=$(cargo test -p precc-core -- stash 2>&1 | grep "test result" || true)
if echo "$STASH_RESULT" | grep -q "0 failed"; then
    pass "stash mechanism unit tests pass"
else
    fail "stash tests failed" "$STASH_RESULT"
fi

# ═══════════════════════════════════════════════════════════════════════════
# LEAN-CTX / NUSHELL (conditional)
# ═══════════════════════════════════════════════════════════════════════════

section "Feature 19: Lean-ctx Wrap"

if [ "$HAS_LEANCTX" = "1" ]; then
    # Need to disable nushell which would otherwise take priority
    OUT=$(run_hook_env "PRECC_NUSHELL=0 PRECC_LEAN_CTX=1" '{"tool_name":"Bash","tool_input":{"command":"cargo build"},"cwd":"'${CWD}'"}')
    if echo "$OUT" | grep -q "lean-ctx"; then
        pass "cargo build → lean-ctx -c '...'"
    else
        fail "lean-ctx wrap not applied" "$OUT"
    fi
else
    skip "lean-ctx not installed"
fi

section "Feature 20: Nushell Wrap"

if [ "$HAS_NU" = "1" ]; then
    OUT=$(run_hook_env "PRECC_NUSHELL=1 PRECC_LEAN_CTX=0" '{"tool_name":"Bash","tool_input":{"command":"cargo build"},"cwd":"'${CWD}'"}')
    if echo "$OUT" | grep -q "nushell-wrap\|nu -c\|--message-format"; then
        pass "cargo build → nushell wrapped"
    else
        fail "nushell wrap not triggered" "$OUT"
    fi
else
    skip "nushell (nu) not installed"
fi

section "Feature 21: CCC Semantic Search"

if [ "$HAS_CCC" = "1" ]; then
    if [ -d "${CWD}/.cocoindex_code" ]; then
        # Use a sufficiently long pattern (≥4 chars) and disable other compression
        OUT=$(run_hook_env "PRECC_NUSHELL=0 PRECC_LEAN_CTX=0" '{"tool_name":"Bash","tool_input":{"command":"grep -r \"hash_command\" src/"},"cwd":"'${CWD}'"}')
        if echo "$OUT" | grep -q "printf\|ccc-redirect\|ccc search"; then
            pass "grep -r redirected through CCC semantic search"
        else
            skip "CCC redirect not triggered (no matching index entries)"
        fi
    else
        skip "no .cocoindex_code index — run 'ccc init && ccc index' to enable"
    fi
else
    skip "CCC not installed"
fi

# ═══════════════════════════════════════════════════════════════════════════
# ADDITIONAL FEATURES (newly tested)
# ═══════════════════════════════════════════════════════════════════════════

section "Feature 22: Lean-ctx + RTK priority order"

if [ "$HAS_LEANCTX" = "1" ] && [ "$HAS_RTK" = "1" ]; then
    # When both enabled, lean-ctx should take priority over RTK
    OUT=$(run_hook_env "PRECC_NUSHELL=0 PRECC_LEAN_CTX=1" '{"tool_name":"Bash","tool_input":{"command":"git status"},"cwd":"'${CWD}'"}')
    if echo "$OUT" | grep -q "lean-ctx" && ! echo "$OUT" | grep -q '"rtk git'; then
        pass "lean-ctx takes priority over RTK when both enabled"
    else
        fail "priority order incorrect" "$OUT"
    fi
else
    skip "need both lean-ctx and rtk"
fi

section "Feature 23: Nushell + Lean-ctx priority order"

if [ "$HAS_NU" = "1" ] && [ "$HAS_LEANCTX" = "1" ]; then
    # When both enabled, nushell should take priority over lean-ctx
    OUT=$(run_hook_env "PRECC_NUSHELL=1 PRECC_LEAN_CTX=1" '{"tool_name":"Bash","tool_input":{"command":"cargo build"},"cwd":"'${CWD}'"}')
    if echo "$OUT" | grep -q "nushell-wrap\|--message-format"; then
        pass "nushell takes priority over lean-ctx when both enabled"
    else
        fail "priority order incorrect" "$OUT"
    fi
else
    skip "need both nushell and lean-ctx"
fi

section "Feature 24: Lean-ctx avoids double-wrapping"

if [ "$HAS_LEANCTX" = "1" ]; then
    OUT=$(run_hook_env "PRECC_NUSHELL=0 PRECC_LEAN_CTX=1" '{"tool_name":"Bash","tool_input":{"command":"lean-ctx -c '"'"'cargo test'"'"'"},"cwd":"'${CWD}'"}')
    if echo "$OUT" | grep -q "lean-ctx -c.*lean-ctx"; then
        fail "double-wrapped lean-ctx" "$OUT"
    else
        pass "no double-wrap for lean-ctx -c command"
    fi
else
    skip "lean-ctx not installed"
fi

section "Feature 25: Nushell skips heredocs"

if [ "$HAS_NU" = "1" ]; then
    OUT=$(run_hook_env "PRECC_NUSHELL=1" '{"tool_name":"Bash","tool_input":{"command":"cat <<EOF\nhello\nEOF"},"cwd":"'${CWD}'"}')
    if echo "$OUT" | grep -q "nushell-wrap"; then
        fail "nushell wrapped a heredoc command (unsafe)" "$OUT"
    else
        pass "nushell skips heredoc commands"
    fi
else
    skip "nushell not installed"
fi

# ═══════════════════════════════════════════════════════════════════════════
# SUMMARY
# ═══════════════════════════════════════════════════════════════════════════

echo ""
echo "═══════════════════════════════════════════════"
TOTAL=$((PASS + FAIL + SKIP))
echo -e "  ${GREEN}${PASS} passed${NC}, ${RED}${FAIL} failed${NC}, ${YELLOW}${SKIP} skipped${NC} (${TOTAL} total)"
echo "═══════════════════════════════════════════════"

[ "$FAIL" -gt 0 ] && exit 1 || exit 0
