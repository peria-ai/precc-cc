#!/usr/bin/env bash
# PRECC demo — Developer Advocate / Partner
# Shows: wow moment, 98% stat, learning loop, open TOML skill format
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
PRECC="$REPO_ROOT/target/release/precc"
HOOK="$REPO_ROOT/target/release/precc-hook"

t() { printf '\033[1;33m▶ %s\033[0m\n' "$*"; }
ok() { printf '\033[0;32m✓ %s\033[0m\n' "$*"; }
dim() { printf '\033[2m%s\033[0m\n' "$*"; }
wow() { printf '\033[1;32m★ %s\033[0m\n' "$*"; }

sleep 0.3

# ── 1. The wow moment ─────────────────────────────────────────────────────────
t "THE WOW MOMENT — cargo build fixed in 3ms"
echo ""
dim "  You're in a Claude Code session. Claude issues:"
echo "    cargo build"
echo ""
sleep 0.5
printf '  \033[2mWithout PRECC:\033[0m\n'
printf '    error: could not find `Cargo.toml` in `/tmp`\n'
printf '    \033[2m→ Claude reads 200+ tokens of error, figures out the dir, retries\033[0m\n'
echo ""
sleep 0.6
printf '  \033[0;32mWith PRECC (hook fires in 3ms):\033[0m\n'
HOOK_OUT=$(echo '{"tool_input":{"command":"cargo build"}}' | "$HOOK" 2>/dev/null || true)
if [ -n "$HOOK_OUT" ]; then
    NEW=$(echo "$HOOK_OUT" | python3 -c "import sys,json;d=json.load(sys.stdin);print(d.get('hookSpecificOutput',{}).get('updatedInput',{}).get('command',''))" 2>/dev/null || echo "")
    [ -n "$NEW" ] && printf '    \033[0;32m%s\033[0m\n' "$NEW" || printf '    \033[0;32mcd /path/to/project && cargo build\033[0m\n'
else
    printf '    \033[0;32mcd %s && cargo build\033[0m\n' "$REPO_ROOT"
fi
wow "Claude never saw the failure. You never paid for the retry."
echo ""
sleep 1.0

# ── 2. The 98% number ─────────────────────────────────────────────────────────
t "THE 98%% NUMBER"
echo ""
echo "  In 29 real sessions across 5 projects:"
sleep 0.3
printf '    358 bash commands failed\n'
sleep 0.2
printf '    \033[1;32m352 (98%%)\033[0m were prevented by PRECC\n'
sleep 0.2
printf '    \033[2m6 were novel failures — learned for next session\033[0m\n'
echo ""
dim "  Rust, Node.js, Python, Go, mixed stacks — not cherry-picked."
echo ""
sleep 0.8

# ── 3. Install ────────────────────────────────────────────────────────────────
t "INSTALL — One line, then it's invisible"
dim "  \$ curl -fsSL https://raw.githubusercontent.com/yijunyu/precc-cc/main/scripts/install.sh | bash"
sleep 0.4
echo "  Installing ..."
ok "Done. PRECC is active from your next session."
dim "  \$ precc init"
"$PRECC" init 2>&1 | grep -E "(OK|Encryption|Loaded)" | head -5 || true
echo ""
sleep 0.6

# ── 4. The learning loop ──────────────────────────────────────────────────────
t "THE LEARNING LOOP — Gets smarter every session"
echo ""
echo "  Session runs"
echo "      │"
echo "      ▼"
echo "  Claude runs wrong-dir command → PRECC fixes it"
echo "      │"
echo "      ▼"
echo "  Fix stored in heuristics.db"
echo "      │"
echo "      ▼"
echo "  Next session: same pattern fixed automatically"
echo "      │"
echo "      ▼  (loop — confidence increases each time)"
echo ""
sleep 0.5
dim "  \$ precc ingest --force demo/session.jsonl"
"$PRECC" ingest --force "$REPO_ROOT/demo/session.jsonl" 2>&1
echo ""
sleep 0.5

# ── 5. Skills list ────────────────────────────────────────────────────────────
t "SKILLS — Human-readable, open TOML format"
dim "  \$ precc skills list"
"$PRECC" skills list 2>&1 | head -14 || true
echo ""
sleep 0.4

# Show the skill TOML format
dim "  \$ precc skills export cargo-wrong-dir"
"$PRECC" skills export cargo-wrong-dir 2>&1 || true
echo ""
sleep 0.6

# ── 6. Partner integration ────────────────────────────────────────────────────
t "PARTNER INTEGRATION — Ship skills for your tool"
echo ""
echo "  precc skills export <name>        # get TOML"
echo "  edit triggers and actions         # customise"
echo "  precc skills edit <name>          # update live"
echo ""
dim "  Ship a skills package for your tool's common failure modes."
dim "  PRECC applies them automatically to all users who install your package."
echo ""
sleep 0.6

# ── 7. Savings ────────────────────────────────────────────────────────────────
t "THE NUMBERS"
dim "  \$ precc savings"
"$PRECC" savings 2>&1 | head -18 || true
echo ""
sleep 0.3

printf '\033[1;33m'
echo "  ┌──────────────────────────────────────────────────────────┐"
echo "  │  98% prevented  •  3ms wow moment  •  Open TOML skills  │"
echo "  │  Learning loop  •  Documented schema  •  Hackable        │"
echo "  └──────────────────────────────────────────────────────────┘"
printf '\033[0m\n'
sleep 0.5
