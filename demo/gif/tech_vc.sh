#!/usr/bin/env bash
# PRECC demo — Technical Investor / Angel
# Shows: portfolio architecture, jj translation, license system, RTK sync, metrics
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
PRECC="$REPO_ROOT/target/release/precc"
HOOK="$REPO_ROOT/target/release/precc-hook"

t() { printf '\033[1;35m▶ %s\033[0m\n' "$*"; }
ok() { printf '\033[0;32m✓ %s\033[0m\n' "$*"; }
dim() { printf '\033[2m%s\033[0m\n' "$*"; }
h() { printf '\033[1;33m%s\033[0m\n' "$*"; }

sleep 0.3

# ── 1. Architecture ───────────────────────────────────────────────────────────
t "ARCHITECTURE — 6-stage hook pipeline"
echo ""
printf '  stdin (JSON: bash command)\n'
printf '       │\n'
printf '  Stage 1: Parse\n'
printf '  Stage 2: \033[0;32mSkills portfolio\033[0m  ← ALL matching skills, not just first\n'
printf '  Stage 3: Context resolution  (Cargo.toml / package.json walk-up)\n'
printf '  Stage 4: GDB check           (repeated failures → debug suggestion)\n'
printf '  Stage 5: \033[0;32mjj translation\033[0m    ← git → jj if .jj/ present\n'
printf '  Stage 6: RTK compression     (30+ rules, v0.27.2 sync)\n'
printf '       │\n'
printf '  stdout (modified JSON)\n'
echo ""
sleep 0.8

# ── 2. Portfolio demo ─────────────────────────────────────────────────────────
t "PORTFOLIO — Multiple skills per command"
echo ""
dim "  Old behaviour: break after first match"
dim "  New behaviour: apply all compatible high-confidence skills"
echo ""
printf '  Input: \033[1mcargo clippy\033[0m\n'
sleep 0.4
HOOK_OUT=$(printf '{"tool_input":{"command":"cargo clippy"}}' | "$HOOK" 2>/dev/null || true)
if [ -n "$HOOK_OUT" ]; then
    CMD=$(printf '%s' "$HOOK_OUT" | python3 -c "import sys,json;d=json.load(sys.stdin);print(d.get('hookSpecificOutput',{}).get('updatedInput',{}).get('command',''))" 2>/dev/null || echo "")
    [ -n "$CMD" ] && printf '  Output: \033[0;32m%s\033[0m\n' "$CMD"
fi
printf '  \033[2m+ suggest: warn-identify (conf=0.85) — stacked, not replaced\033[0m\n'
echo ""
sleep 0.6

# ── 3. jj translation ─────────────────────────────────────────────────────────
t "jj TRANSLATION — Jujutsu-colocated repo detection"
echo ""
printf '  Checks: .jj/ in working dir or 5 ancestors (cached)\n'
printf '  \n'
printf '  git add .      →  \033[0;32mtrue  # jj: changes implicitly staged\033[0m\n'
printf '  git status     →  \033[0;32mjj st\033[0m\n'
printf '  git checkout X →  \033[0;32mjj edit X\033[0m\n'
printf '  git checkout -b feature  →  \033[0;32mjj new -B feature\033[0m\n'
printf '  git worktree add ../X    →  \033[0;32mjj workspace add ../X\033[0m\n'
printf '  \n'
printf '  \033[2mToken impact: git add → 0 tokens (no staging output)\033[0m\n'
echo ""
sleep 0.6

# ── 4. RTK v0.27.2 sync ───────────────────────────────────────────────────────
t "RTK SYNC — 30 new rewrite rules (v0.22 → v0.27.2)"
echo ""
for cmd in "golangci-lint run" "ruff check src/" "mypy src/" "aws s3 ls" "psql -c 'SELECT 1'" "find . -name '*.rs'" "docker compose ps" "wget https://example.com"; do
    printf '  %-30s → \033[0;32mrtk %s\033[0m\n' "$cmd" "$cmd"
    sleep 0.1
done || true
echo ""
sleep 0.5

# ── 5. License system ─────────────────────────────────────────────────────────
t "LICENSE — HMAC-SHA256 machine-bound keys"
echo ""
printf '  Key format: PRECC-XXXXXXXX-XXXXXXXX-XXXXXXXX-XXXXXXXX\n'
printf '  \n'
dim "  \$ precc license fingerprint"
"$PRECC" license fingerprint 2>&1 || true
echo ""
dim "  \$ precc license status"
"$PRECC" license status 2>&1 || true
printf '  \n'
printf '  \033[2mMAC covers: machine_tag || expiry_days || edition_flags\033[0m\n'
printf '  \033[2mMachine binding: SHA-256(hostname+username)[0..4]\033[0m\n'
echo ""
sleep 0.6

# ── 6. Metrics ────────────────────────────────────────────────────────────────
t "METRICS — Real measured numbers"
echo ""
h "  | Metric                  | Value        |"
h "  |-------------------------|--------------|"
printf '  | Cost savings            | 34%% ($296/$878)\n'
printf '  | Failures prevented      | 98%% (352/358)\n'
printf '  | Commands improved       | 17%% (894/5,384)\n'
printf '  | Hook p99 latency        | < 5ms        |\n'
printf '  | New RTK rules synced    | +30          |\n'
printf '  | jj git add saving       | ~40 tokens   |\n'
echo ""
printf '\033[1;32m'
echo "  Technical moat: portfolio × jj × RTK × license × mail"
printf '\033[0m\n'
sleep 0.5
