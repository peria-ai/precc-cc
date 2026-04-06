#!/usr/bin/env bash
# build-book.sh — Build the PRECC mdbook from templates + SQLite DB
#
# Architecture:
#   1. Templates (docs/book/templates/*.md.tpl) use {{key}} for DB strings
#      and <span data-stat="key">fallback</span> for live stats
#   2. This script resolves {{key}} per language from book.db
#   3. Live stats are fetched client-side via JS (no rebuild needed)
#
# Usage:
#   bash scripts/build-book.sh              # build only
#   bash scripts/build-book.sh --deploy     # build + deploy to /var/www/precc.cc
#   bash scripts/build-book.sh --stats      # update stats.json from telemetry

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
BOOK_DIR="${PROJECT_DIR}/docs/book"
DB="${BOOK_DIR}/db/book.db"
TPL_DIR="${BOOK_DIR}/templates"
OUT_DIR="${PROJECT_DIR}/target/book"
FINAL_DIR="${PROJECT_DIR}/target/book-all"
DEPLOY_DIR="/var/www/precc.cc"
STATS_JSON="${DEPLOY_DIR}/api/stats.json"
TELEMETRY_JSONL="${HOME}/.local/share/precc/telemetry.jsonl"

# UN official languages first, then others
LANGUAGES=("en" "zh" "es" "fr" "ru" "ar" "zt" "ja" "ko" "vi" "th" "my" "mn" "bo" "fa" "tr" "pt" "de" "nl" "hu" "pl" "it" "da" "sv" "fi" "is2" "ro" "cs")

# ── Helper: resolve {{key}} placeholders from DB for a given language ────────
resolve_templates() {
    local lang="$1"
    local src_dir="${BOOK_DIR}/src/${lang}"

    # For each template file, resolve placeholders using Python
    # (bash string substitution breaks on multiline content with special chars)
    for tpl in "${TPL_DIR}"/*.md.tpl; do
        [ -f "$tpl" ] || continue
        local base
        base="$(basename "$tpl" .tpl)"
        local outfile="${src_dir}/${base}"

        python3 -c "
import sqlite3, re, sys

db = sqlite3.connect('${DB}')
lang = '${lang}'

def lookup(key):
    try:
        row = db.execute('SELECT {} FROM strings WHERE key=?'.format(lang), (key,)).fetchone()
        if row and row[0]:
            return row[0]
    except Exception:
        pass
    # Fall back to English
    row = db.execute('SELECT en FROM strings WHERE key=?', (key,)).fetchone()
    return row[0] if row and row[0] else '??' + key + '??'

with open('${tpl}') as f:
    content = f.read()

# Resolve {{i18n:key}} first, then {{key}}
content = re.sub(r'\{\{i18n:(\w+)\}\}', lambda m: lookup(m.group(1)), content)
content = re.sub(r'\{\{(\w+)\}\}', lambda m: lookup(m.group(1)), content)

with open('${outfile}', 'w') as f:
    f.write(content)
"
        echo "  ${lang}/${base} <- $(basename "$tpl")"
    done
}

# ── Helper: update stats.json from telemetry ─────────────────────────────────
update_stats() {
    if [ ! -f "$TELEMETRY_JSONL" ]; then
        echo "No telemetry data at ${TELEMETRY_JSONL}"
        return 1
    fi

    local tmpjson
    tmpjson="$(mktemp)"

    python3 -c "
import json, sys, time
from collections import defaultdict

reports = [json.loads(line) for line in open('$TELEMETRY_JSONL')]

def saved(r):
    p = r.get('pillars', {})
    return sum(p.get(k, 0) or 0 for k in ['rtk_tokens_saved','cd_tokens_saved','skill_tokens_saved','mined_tokens_saved','lean_ctx_tokens_saved'])

# Deduplicate: group by (IP, version), keep only the latest report per user per version.
# Since counters are cumulative, multiple reports from the same user are duplicates.
by_user_ver = {}
for r in reports:
    ip = r.get('_remote_ip', 'unknown')
    ver = r.get('precc_version', 'unknown')
    ts = r.get('_received_at', 0)
    key = (ip, ver)
    if key not in by_user_ver or ts > by_user_ver[key].get('_received_at', 0):
        by_user_ver[key] = r

deduped = list(by_user_ver.values())

# Group by version
by_ver = defaultdict(list)
for r in deduped:
    by_ver[r.get('precc_version', 'unknown')].append(r)

def combined(r):
    \"\"\"Total savings: pillar savings + compression savings.\"\"\"
    # Use combined_tokens_saved if available (new telemetry schema)
    c = r.get('combined_tokens_saved', 0) or 0
    if c > 0:
        return c
    # Fallback: pillar savings + compression estimate
    s = saved(r)
    comp = r.get('compression_tokens_saved', 0) or 0
    return s + comp

# Per-version stats (each report is now unique per user)
version_stats = []
for ver, rs in sorted(by_ver.items(), reverse=True):
    ts = sum(combined(r) for r in rs)
    pillar = sum(saved(r) for r in rs)
    comp = sum(r.get('compression_tokens_saved', 0) or 0 for r in rs)
    api = sum(r.get('total_api_tokens', 0) or 0 for r in rs)
    hooks = sum((r.get('hook_latency') or {}).get('count', 0) or 0 for r in rs)
    pct = round(ts / api * 100, 1) if api > 0 else 0
    version_stats.append({
        'version': ver, 'users': len(rs), 'tokens_saved': int(ts),
        'pillar_savings': int(pillar), 'compression_savings': int(comp),
        'total_api_tokens': api, 'total_invocations': hooks, 'saving_pct': pct
    })

# Latest version stats
latest_ver = sorted(by_ver.keys())[-1]
latest = by_ver[latest_ver]

lt_combined = sum(combined(r) for r in latest)
lt_pillar = sum(saved(r) for r in latest)
lt_comp = sum(r.get('compression_tokens_saved', 0) or 0 for r in latest)
lt_api = sum(r.get('total_api_tokens', 0) or 0 for r in latest)
lt_hooks = sum((r.get('hook_latency') or {}).get('count', 0) or 0 for r in latest)
lt_p50 = [((r.get('hook_latency') or {}).get('p50_ms', 0) or 0) for r in latest]

# Aggregate measured data from latest version reports
def get_measured(r):
    return r.get('measured', {})

lt_measured = [get_measured(r) for r in latest if get_measured(r)]
measured_agg = {
    'original_output_tokens': sum(m.get('original_output_tokens', 0) or 0 for m in lt_measured),
    'actual_output_tokens': sum(m.get('actual_output_tokens', 0) or 0 for m in lt_measured),
    'savings_tokens': sum(m.get('savings_tokens', 0) or 0 for m in lt_measured),
    'ground_truth_count': sum(m.get('ground_truth_count', 0) or 0 for m in lt_measured),
    'measurement_count': sum(m.get('measurement_count', 0) or 0 for m in lt_measured),
}
orig = measured_agg['original_output_tokens']
measured_agg['savings_pct'] = round(measured_agg['savings_tokens'] / orig * 100, 1) if orig > 0 else 0

# Aggregate per-rewrite-type across users
rt_agg = {}
for m in lt_measured:
    for rt in m.get('by_rewrite_type', []):
        name = rt.get('rewrite_type', 'unknown')
        if name not in rt_agg:
            rt_agg[name] = {'count': 0, 'total_savings': 0, 'pct_sum': 0}
        rt_agg[name]['count'] += rt.get('count', 0)
        rt_agg[name]['total_savings'] += rt.get('total_savings_tokens', 0)
        rt_agg[name]['pct_sum'] += rt.get('avg_savings_pct', 0) * rt.get('count', 1)
measured_agg['by_rewrite_type'] = [
    {'rewrite_type': k, 'count': v['count'],
     'avg_savings_pct': round(v['pct_sum'] / max(v['count'], 1), 1),
     'total_savings_tokens': v['total_savings']}
    for k, v in sorted(rt_agg.items(), key=lambda x: -x[1]['total_savings'])
]

out = {
    'current_version': latest_ver,
    'total_tokens_saved': int(lt_combined),
    'pillar_savings': int(lt_pillar),
    'compression_savings': int(lt_comp),
    'rtk_rewrites': sum(r.get('pillars', {}).get('rtk_rewrites', 0) or 0 for r in latest),
    'cd_prepends': sum(r.get('pillars', {}).get('cd_prepends', 0) or 0 for r in latest),
    'skill_activations': sum(r.get('pillars', {}).get('skill_activations', 0) or 0 for r in latest),
    'mined_preventions': sum(r.get('pillars', {}).get('mined_preventions', 0) or 0 for r in latest),
    'total_api_tokens': lt_api,
    'total_invocations': lt_hooks,
    'avg_latency_p50_ms': round(sum(lt_p50) / len(lt_p50), 2) if lt_p50 else 0,
    'unique_users': len(latest),
    'saving_pct': round(lt_combined / lt_api * 100, 1) if lt_api > 0 else 0,
    'measured': measured_agg,
    'by_version': version_stats,
    'updated_at': time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime()),
}
json.dump(out, sys.stdout, indent=2)
" > "$tmpjson"

    sudo mkdir -p "$(dirname "$STATS_JSON")"
    sudo cp "$tmpjson" "$STATS_JSON"
    sudo chmod 644 "$STATS_JSON"
    rm -f "$tmpjson"

    echo "Updated ${STATS_JSON}"
    cat "$STATS_JSON"
}

# ── Handle --stats flag ──────────────────────────────────────────────────────
if [[ "${1:-}" == "--stats" ]]; then
    update_stats
    exit 0
fi

# ── Main build ────────────────────────────────────────────────────────────────
echo "=== PRECC Book Build ==="
echo ""

# Step 1: Resolve templates per language
echo "Resolving templates from DB..."
for lang in "${LANGUAGES[@]}"; do
    resolve_templates "$lang"
done
echo ""

# Step 2: Build mdbook per language
rm -rf "$FINAL_DIR"
for lang in "${LANGUAGES[@]}"; do
    echo "Building ${lang}..."
    sed -i "s|^src = .*|src = \"src/${lang}\"|" "${BOOK_DIR}/book.toml"
    mdbook build "$BOOK_DIR" 2>&1 | grep -v '^$'
    mkdir -p "${FINAL_DIR}/${lang}"
    cp -r "${OUT_DIR}/"* "${FINAL_DIR}/${lang}/"
done

# Restore default
sed -i 's|^src = .*|src = "src/en"|' "${BOOK_DIR}/book.toml"

# Step 3: Create index.html
cat > "${FINAL_DIR}/index.html" <<'INDEXEOF'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>PRECC Documentation</title>
    <meta name="description" content="PRECC User Guide — Predictive Error Correction for Claude Code. Available in 5 languages.">
    <link rel="canonical" href="https://precc.cc/">
    <style>
        *, *::before, *::after { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
            background: #0a0e1a; color: #e2e8f0;
            min-height: 100vh; display: flex; align-items: center; justify-content: center;
        }
        .container { max-width: 640px; text-align: center; padding: 48px 24px; }
        h1 { font-size: 42px; font-weight: 800; letter-spacing: -1px; margin-bottom: 8px; }
        h1 span { color: #10b981; }
        .sub { color: #94a3b8; font-size: 17px; margin-bottom: 48px; line-height: 1.6; }
        .langs { display: flex; flex-wrap: wrap; justify-content: center; gap: 12px; margin-bottom: 48px; }
        .langs a {
            display: inline-block; padding: 14px 32px;
            background: #151c2c; border: 1px solid #1e2a3a;
            color: #e2e8f0; text-decoration: none; border-radius: 10px;
            font-size: 17px; font-weight: 600; transition: all 0.2s;
        }
        .langs a:hover { border-color: #10b981; background: #1a2236; }
        .links { font-size: 14px; color: #64748b; }
        .links a { color: #10b981; text-decoration: none; }
        .links a:hover { text-decoration: underline; }
    </style>
</head>
<body>
    <div class="container">
        <h1><span>PRECC</span> Documentation</h1>
        <p class="sub">Predictive Error Correction for Claude Code</p>
        <div class="langs">
            <a href="en/">English</a>
            <a href="zh/">&#x7B80;&#x4F53;&#x4E2D;&#x6587;</a>
            <a href="es/">Espa&ntilde;ol</a>
            <a href="fr/">Fran&ccedil;ais</a>
            <a href="ru/">&#x0420;&#x0443;&#x0441;&#x0441;&#x043A;&#x0438;&#x0439;</a>
            <a href="ar/">&#x0627;&#x0644;&#x0639;&#x0631;&#x0628;&#x064A;&#x0629;</a>
            <a href="zt/">&#x7E41;&#x9AD4;&#x4E2D;&#x6587;</a>
            <a href="ja/">&#x65E5;&#x672C;&#x8A9E;</a>
            <a href="ko/">&#xD55C;&#xAD6D;&#xC5B4;</a>
            <a href="vi/">Ti&#x1EBF;ng Vi&#x1EC7;t</a>
            <a href="th/">&#x0E44;&#x0E17;&#x0E22;</a>
            <a href="my/">&#x1019;&#x103C;&#x1014;&#x103A;&#x1019;&#x102C;</a>
            <a href="mn/">&#x041C;&#x043E;&#x043D;&#x0433;&#x043E;&#x043B;</a>
            <a href="bo/">&#x0F56;&#x0F7C;&#x0F51;&#x0F0B;&#x0F66;&#x0F90;&#x0F51;</a>
            <a href="fa/">&#x0641;&#x0627;&#x0631;&#x0633;&#x06CC;</a>
            <a href="tr/">T&uuml;rk&ccedil;e</a>
            <a href="pt/">Portugu&ecirc;s</a>
            <a href="de/">Deutsch</a>
            <a href="nl/">Nederlands</a>
            <a href="hu/">Magyar</a>
            <a href="pl/">Polski</a>
            <a href="it/">Italiano</a>
            <a href="da/">Dansk</a>
            <a href="sv/">Svenska</a>
            <a href="fi/">Suomi</a>
            <a href="is2/">&#xCD;slenska</a>
            <a href="ro/">Rom&acirc;n&#x103;</a>
            <a href="cs/">&#x010C;e&#x0161;tina</a>
        </div>
        <p class="links">
            <a href="https://github.com/peria-ai/precc-cc">GitHub</a> &middot;
            <a href="https://peria.ai">peria.ai</a>
        </p>
    </div>
</body>
</html>
INDEXEOF

echo ""
echo "Book built to ${FINAL_DIR}/"

# Step 4: Deploy if requested
if [[ "${1:-}" == "--deploy" ]]; then
    echo ""
    echo "Deploying to ${DEPLOY_DIR}..."
    sudo mkdir -p "${DEPLOY_DIR}"
    sudo cp -r "${FINAL_DIR}/"* "${DEPLOY_DIR}/"
    sudo chmod -R 755 "${DEPLOY_DIR}"
    sudo find "${DEPLOY_DIR}" -type f -exec chmod 644 {} \;

    # Update live stats
    update_stats || true

    echo "Deployed to https://precc.cc/"
fi
