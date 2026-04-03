#!/usr/bin/env bash
# PRECC demo — Developer / Claude Code Power User
# Shows: install, skill portfolio, jj translation, savings, email report
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
PRECC="$REPO_ROOT/target/release/precc"

t() { printf '\033[1;36m▶ %s\033[0m\n' "$*"; }
ok() { printf '\033[0;32m✓ %s\033[0m\n' "$*"; }
dim() { printf '\033[2m%s\033[0m\n' "$*"; }

sleep 0.3

# ── 1. Installation ───────────────────────────────────────────────────────────
t "STEP 1 — Install PRECC (one command)"
dim "  \$ curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash"
sleep 0.5
echo "  Downloading precc v0.1.0 for linux-x86_64 ..."
echo "  Installing to ~/.local/bin/ ..."
echo "  Writing hook to ~/.claude/settings.json ..."
ok "Installed. Next Claude Code session: PRECC is active."
echo ""
sleep 0.4

# ── 2. Init ───────────────────────────────────────────────────────────────────
t "STEP 2 — Initialise"
dim "  \$ precc init"
sleep 0.2
"$PRECC" init 2>&1 | head -10 || true
echo ""
sleep 0.4

# ── 3. Skill portfolio ────────────────────────────────────────────────────────
t "STEP 3 — Skill portfolio (multiple fixes per command)"
echo ""
printf '  \033[2mCommand:  cargo clippy\033[0m\n'
sleep 0.3
printf '  \033[0;32m→ prepend_cd  (wrong dir, conf=0.92)\033[0m\n'
sleep 0.2
printf '  \033[0;32m→ suggest:    run warn-identify (conf=0.85)\033[0m\n'
sleep 0.2
printf '  \033[0;32m→ RTK:        rtk cargo clippy\033[0m\n'
sleep 0.2
printf '  \033[2mThree improvements, one hook call, 2.93ms\033[0m\n'
echo ""
sleep 0.5

# ── 4. jj translation ─────────────────────────────────────────────────────────
t "STEP 4 — Jujutsu (jj) translation in colocated repos"
echo ""
printf '  \033[2m.jj/ detected — git commands auto-translated:\033[0m\n'
sleep 0.3
printf '  git status  →  \033[0;32mjj st\033[0m\n'
sleep 0.2
printf '  git add .   →  \033[0;32mtrue  # jj: changes implicitly staged\033[0m\n'
sleep 0.2
printf '  git commit  →  \033[0;32mjj commit\033[0m\n'
sleep 0.2
printf '  git log     →  \033[0;32mjj log\033[0m\n'
printf '  \033[2m~40 tokens saved per git add cycle\033[0m\n'
echo ""
sleep 0.5

# ── 5. Mine sessions ──────────────────────────────────────────────────────────
t "STEP 5 — Learn from past sessions"
dim "  \$ precc ingest --force demo/session.jsonl"
sleep 0.2
"$PRECC" ingest --force "$REPO_ROOT/demo/session.jsonl" 2>&1
echo ""
sleep 0.4

# ── 6. Skills ─────────────────────────────────────────────────────────────────
t "STEP 6 — Skills database (30+ RTK rules + builtin skills)"
dim "  \$ precc skills list"
sleep 0.2
"$PRECC" skills list 2>&1 | head -14 || true
echo ""
sleep 0.4

# ── 7. License ────────────────────────────────────────────────────────────────
t "STEP 7 — License status"
dim "  \$ precc license status"
sleep 0.2
"$PRECC" license status 2>&1 || true
echo ""
sleep 0.4

# ── 8. Savings ────────────────────────────────────────────────────────────────
t "STEP 8 — Token savings"
dim "  \$ precc savings"
sleep 0.2
"$PRECC" savings 2>&1 | head -18 || true
echo ""
sleep 0.3

# ── 9. Summary ────────────────────────────────────────────────────────────────
printf '\033[1;32m'
echo "  ┌────────────────────────────────────────────────────────────┐"
echo "  │  PRECC — Zero-config. Portfolio skills. jj-aware.          │"
echo "  │  34% cheaper  •  98% failures prevented  •  2.93ms         │"
echo "  │  precc mail report you@email.com  →  share your savings     │"
echo "  └────────────────────────────────────────────────────────────┘"
printf '\033[0m\n'
sleep 0.5
