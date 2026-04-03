#!/usr/bin/env bash
# PRECC demo — Enterprise CTO / Procurement Buyer
# Shows: security, license enforcement, SMTP audit reports, fail-open, ROI
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
PRECC="$REPO_ROOT/target/release/precc"

t() { printf '\033[1;31m▶ %s\033[0m\n' "$*"; }
ok() { printf '\033[0;32m✓ %s\033[0m\n' "$*"; }
dim() { printf '\033[2m%s\033[0m\n' "$*"; }
secure() { printf '\033[0;36m🔒 %s\033[0m\n' "$*"; }

sleep 0.3

# ── 1. Security architecture ──────────────────────────────────────────────────
t "SECURITY — Zero-network, fail-open, encrypted at rest"
echo ""
secure "Data at rest:    AES-256 via SQLCipher"
sleep 0.2
secure "Key derivation:  HKDF-SHA256(machine-id + username)"
sleep 0.2
secure "Network calls:   ZERO at runtime"
sleep 0.2
secure "SMTP:            Only when explicitly invoked by user"
sleep 0.2
secure "Fail-open:       Hook crash → Claude Code unaffected (exit 0)"
sleep 0.2
secure "License:         HMAC-SHA256, machine-bound keys"
echo ""
sleep 0.6

# ── 2. License enforcement ────────────────────────────────────────────────────
t "LICENSE — Machine-bound key enforcement"
echo ""
printf '  Format: PRECC-XXXXXXXX-XXXXXXXX-XXXXXXXX-XXXXXXXX\n'
printf '  \n'
printf '  Payload:  machine_tag(4B) | expiry_days(4B) | edition_flags(4B)\n'
printf '  MAC:      HMAC-SHA256(payload)[0..4] with build-time secret\n'
printf '  Binding:  SHA-256(hostname+username)[0..4]\n'
printf '  \n'
printf '  Copied key on different machine: \033[0;31mMAC valid but fingerprint mismatch → rejected\033[0m\n'
echo ""
dim "  \$ precc license fingerprint"
"$PRECC" license fingerprint 2>&1 || true
echo ""
dim "  \$ precc license status"
"$PRECC" license status 2>&1 || true
echo ""
sleep 0.6

# ── 3. Audit and reporting ────────────────────────────────────────────────────
t "AUDIT — Per-engineer ROI reports delivered by email"
echo ""
dim "  \$ precc report"
"$PRECC" report 2>&1 | head -20 || true
echo ""
printf '  \033[2mDeliver to procurement:\033[0m\n'
dim "  \$ precc mail setup   # one-time SMTP config"
dim "  \$ precc mail report ciso@yourco.com --attach report.txt"
sleep 0.4
ok "Auditable per-machine data emailed. No dashboard required."
echo ""
sleep 0.5

# ── 4. Deployment ─────────────────────────────────────────────────────────────
t "DEPLOYMENT — Three commands, all platforms"
echo ""
printf '  \033[1m# Install\033[0m\n'
dim "  \$ curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash"
printf '  \033[1m# Initialise\033[0m\n'
dim "  \$ precc init"
printf '  \033[1m# Activate enterprise license\033[0m\n'
dim "  \$ precc license activate PRECC-XXXXXXXX-XXXXXXXX-XXXXXXXX-XXXXXXXX"
echo ""
printf '  Platforms: Linux x86_64/ARM64  •  macOS Intel/ARM  •  Windows x86_64\n'
printf '  No Claude Code restart required.\n'
echo ""
sleep 0.5

# ── 5. Risk matrix ────────────────────────────────────────────────────────────
t "RISK — All mitigated"
echo ""
printf '  %-30s %s\n' "Risk" "Mitigation"
printf '  %s\n' "------------------------------------------------------"
for row in \
    "PRECC crashes|Fail-open: Claude Code unaffected" \
    "Wrong correction|conf ≥ 0.7 threshold; logged" \
    "Data exfiltration|Zero network; AES-256 local" \
    "License abuse|Machine-bound keys reject copies" \
    "Vendor lock-in|Remove 1 line from settings.json"
do
    IFS='|' read -r risk mit <<< "$row"
    printf '  \033[0;31m%-30s\033[0m \033[0;32m%s\033[0m\n' "$risk" "$mit"
    sleep 0.15
done || true
echo ""
sleep 0.5

# ── 6. ROI ────────────────────────────────────────────────────────────────────
printf '\033[1;32m'
echo "  ┌──────────────────────────────────────────────────────────────┐"
echo "  │  PRECC Enterprise: secure, auditable, zero-config ROI        │"
echo "  │  34% saving  •  AES-256  •  HMAC license  •  SMTP reports    │"
echo "  │  Fail-open guarantee  •  No network calls at runtime          │"
echo "  └──────────────────────────────────────────────────────────────┘"
printf '\033[0m\n'
sleep 0.5
