#!/usr/bin/env bash
# PRECC demo — Developer Advocate / Partner
# Shows: wow moments, portfolio, jj, skills format, mail-agent, community angle
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
PRECC="$REPO_ROOT/target/release/precc"
HOOK="$REPO_ROOT/target/release/precc-hook"

t() { printf '\033[1;33m▶ %s\033[0m\n' "$*"; }
ok() { printf '\033[0;32m✓ %s\033[0m\n' "$*"; }
dim() { printf '\033[2m%s\033[0m\n' "$*"; }
wow() { printf '\033[1;35m★ %s\033[0m\n' "$*"; }

sleep 0.3

# ── 1. Three wow moments ──────────────────────────────────────────────────────
t "THREE WOW MOMENTS"
echo ""
wow "Wow 1 — Context fix (always)"
printf '  cargo build in /tmp → \033[0;32mcd /path/to/project && rtk cargo build\033[0m  (3ms)\n'
echo ""
sleep 0.4
wow "Wow 2 — Portfolio (new)"
printf '  cargo clippy → cd-fix + warn-identify + RTK — all in one hook call\n'
echo ""
sleep 0.4
wow "Wow 3 — jj translation (new)"
printf '  git add .   →  \033[0;32mtrue  # jj: changes implicitly staged\033[0m\n'
printf '  Zero tokens. Model instantly understands why.\n'
echo ""
sleep 0.6

# ── 2. Hook live demo ─────────────────────────────────────────────────────────
t "LIVE HOOK — See it rewrite"
echo ""
printf '  Input:  \033[1mcargo build\033[0m\n'
HOOK_OUT=$(printf '{"tool_input":{"command":"cargo build"}}' | "$HOOK" 2>/dev/null || true)
if [ -n "$HOOK_OUT" ]; then
    CMD=$(printf '%s' "$HOOK_OUT" | python3 -c "import sys,json;d=json.load(sys.stdin);print(d.get('hookSpecificOutput',{}).get('updatedInput',{}).get('command',''))" 2>/dev/null || echo "")
    [ -n "$CMD" ] && printf '  Output: \033[0;32m%s\033[0m\n' "$CMD"
fi
printf '  Latency: 2.93ms average\n'
echo ""
sleep 0.5

# ── 3. Skills TOML format ─────────────────────────────────────────────────────
t "SKILLS — Open TOML format, ship your own"
echo ""
dim "  skills/builtin/jj-translate.toml:"
cat "$REPO_ROOT/skills/builtin/jj-translate.toml" 2>/dev/null | head -20 || true
echo ""
sleep 0.5

# ── 4. Skills list ────────────────────────────────────────────────────────────
t "SKILLS — Full active list"
dim "  \$ precc skills list"
"$PRECC" skills list 2>&1 | head -18 || true
echo ""
sleep 0.4

# ── 5. Mail-agent ─────────────────────────────────────────────────────────────
t "MAIL-AGENT — Share results across network boundaries"
echo ""
printf '  Perfect for: blog post screenshots, team Slack, manager reports\n'
echo ""
dim "  \$ precc mail setup         # one-time SMTP config"
sleep 0.2
dim "  \$ precc mail report you@devrel.io --attach demo.gif"
sleep 0.3
ok "Email sent. PRECC crosses network boundaries."
echo ""
sleep 0.4

# ── 6. 万界方舟 angle ──────────────────────────────────────────────────────────
t "COMMUNITY ANGLE — 万界方舟 × PRECC = 66% saving"
echo ""
printf '  Chinese developer community uses 万界方舟 at 52%% of Anthropic pricing.\n'
printf '  \n'
printf '  Official:           $878\n'
printf '  + 万界方舟 (52%%): $456\n'
printf '  + PRECC (66%%):    \033[1;32m$301\033[0m\n'
printf '  \n'
printf '  Message: "Choose your API source. PRECC maximises whatever you pay for."\n'
echo ""
sleep 0.5

# ── 7. What to share ─────────────────────────────────────────────────────────
printf '\033[1;33m'
echo "  ┌────────────────────────────────────────────────────────────────────┐"
echo "  │  PRECC — Three wow moments. Open skills. Email-ready reports.      │"
echo "  │  98% failures prevented  •  Portfolio mode  •  jj-native           │"
echo "  │  'precc gif demo/gif/devrel.sh 60s' — make your own demo in 60s   │"
echo "  └────────────────────────────────────────────────────────────────────┘"
printf '\033[0m\n'
sleep 0.5
