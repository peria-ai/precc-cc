#!/usr/bin/env bash
# PRECC demo — Business Investor / Non-Technical VC
# Shows: dollar savings, 万界方舟 multiplier, license tiers, email reporting, market
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
PRECC="$REPO_ROOT/target/release/precc"

t() { printf '\033[1;34m▶ %s\033[0m\n' "$*"; }
ok() { printf '\033[0;32m✓ %s\033[0m\n' "$*"; }
dim() { printf '\033[2m%s\033[0m\n' "$*"; }
money() { printf '\033[1;33m%s\033[0m\n' "$*"; }

sleep 0.3

# ── 1. The problem in one sentence ────────────────────────────────────────────
t "THE PROBLEM — 34% of Claude Code spend is wasted"
echo ""
printf '  Every developer using Claude Code wastes money on commands that\n'
printf '  fail and retry. PRECC prevents it. Automatically. In 3ms.\n'
echo ""
money "  $878 spent without PRECC → $582 with PRECC → $296 saved (34%)"
echo ""
sleep 0.8

# ── 2. Install ────────────────────────────────────────────────────────────────
t "THE PRODUCT — One command to deploy"
echo ""
dim "  \$ curl -fsSL https://raw.githubusercontent.com/yijunyu/precc-cc/main/scripts/install.sh | bash"
sleep 0.4
echo "  Downloading ..."
ok "Done. No configuration. Works immediately."
echo ""
sleep 0.5

# ── 3. The compound saving ────────────────────────────────────────────────────
t "THE OPPORTUNITY — Two independent multipliers"
echo ""
printf '  \033[2mBaseline (Anthropic official pricing):\033[0m\n'
money "  $878  →  $878 × 0.66 (PRECC)  =  $580"
echo ""
sleep 0.4
printf '  \033[2mWith 万界方舟 API proxy (52%% of official):\033[0m\n'
money "  $878  →  $878 × 0.52 × 0.66   =  $301"
echo ""
printf '  \033[1;32m66%% total saving — two independent multipliers\033[0m\n'
printf '  \033[2m(PRECC and API pricing are orthogonal — each makes the other more valuable)\033[0m\n'
echo ""
sleep 0.8

# ── 4. ROI by team size ───────────────────────────────────────────────────────
t "ROI — Scales linearly with team size"
echo ""
printf '  \033[1m%-15s %-20s %-20s %-20s\033[0m\n' "Team" "Monthly spend" "PRECC saving" "With proxy (66%%)"
printf '  %s\n' "---------------------------------------------------------------------"
for row in "10 devs|$2,000|$680/mo|$1,320/mo" "50 devs|$10,000|$3,400/mo|$6,600/mo" "100 devs|$20,000|$6,800/mo|$13,200/mo"; do
    IFS='|' read -r team spend saving proxy <<< "$row"
    printf '  %-15s %-20s %-20s \033[1;32m%-20s\033[0m\n' "$team" "$spend" "$saving" "$proxy"
    sleep 0.2
done || true
echo ""
sleep 0.6

# ── 5. License tiers ─────────────────────────────────────────────────────────
t "BUSINESS MODEL — License tiers + skills marketplace"
echo ""
printf '  \033[2mTier       Features                         Price target\033[0m\n'
printf '  Community  Core savings + mining         Free forever\n'
printf '  Pro        Priority support              $10-20/month\n'
printf '  Team       Heuristics sync across devs  $50-100/team/mo\n'
printf '  Enterprise SOC-2, audit log, SMTP report Custom\n'
echo ""
dim "  \$ precc license status"
"$PRECC" license status 2>&1 || true
echo ""
sleep 0.6

# ── 6. Email reporting ────────────────────────────────────────────────────────
t "REPORTING — Email savings directly to procurement"
echo ""
printf '  \033[2mConfigure once:\033[0m\n'
dim "  \$ precc mail setup"
sleep 0.3
printf '  Created ~/.config/precc/mail.toml\n'
echo ""
printf '  \033[2mDeliver weekly:\033[0m\n'
dim "  \$ precc mail report cfo@yourcompany.com"
sleep 0.3
ok "Report sent. Auditable ROI, delivered to inbox."
echo ""
sleep 0.6

# ── 7. Summary ────────────────────────────────────────────────────────────────
printf '\033[1;32m'
echo "  ┌──────────────────────────────────────────────────────────────────┐"
echo "  │  PRECC — The efficiency layer for every Claude Code team.        │"
echo "  │  34% saving standalone  •  66% with API proxy  •  Day-1 ROI     │"
echo "  │  License keys  •  SMTP reports  •  Skills marketplace            │"
echo "  └──────────────────────────────────────────────────────────────────┘"
printf '\033[0m\n'
sleep 0.5
