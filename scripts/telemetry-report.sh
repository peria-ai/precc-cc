#!/usr/bin/env bash
# telemetry-report.sh — Summarize PRECC telemetry data from the server
#
# Deduplicates reports: keeps only the latest per (IP, version) pair,
# since counters are cumulative and multiple uploads are duplicates.
#
# Usage: bash scripts/telemetry-report.sh [path/to/telemetry.jsonl]

set -euo pipefail

DATA="${1:-$HOME/.local/share/precc/telemetry.jsonl}"

if [[ ! -f "${DATA}" ]]; then
    echo "No telemetry data found at ${DATA}" >&2
    exit 1
fi

TOTAL=$(wc -l < "${DATA}")

# Deduplicate: keep only the latest report per (IP, version) pair
DEDUPED=$(mktemp)
trap 'rm -f "${DEDUPED}"' EXIT
jq -sc '
  group_by([._remote_ip // "unknown", .precc_version // "unknown"])
  | map(sort_by(._received_at // 0) | last)
  | .[]
' "${DATA}" > "${DEDUPED}"

UNIQUE=$(wc -l < "${DEDUPED}")
echo "PRECC Telemetry Report"
echo "======================"
echo "Data file: ${DATA}"
echo "Total reports: ${TOTAL} (${UNIQUE} unique after dedup)"
echo ""

echo "── Users by tier ──────────────────────────"
jq -r '.tier // "unknown"' "${DEDUPED}" | sort | uniq -c | sort -rn
echo ""

echo "── Users by OS/arch ───────────────────────"
jq -r '"\(.os // "?")/\(.arch // "?")"' "${DEDUPED}" | sort | uniq -c | sort -rn
echo ""

echo "── Users by version ───────────────────────"
jq -r '.precc_version // "?"' "${DEDUPED}" | sort | uniq -c | sort -rn
echo ""

echo "── Aggregate token savings ────────────────"
jq -s '{
  tokens_saved: ([.[] | [
    .pillars.rtk_tokens_saved // 0,
    .pillars.cd_tokens_saved // 0,
    .pillars.skill_tokens_saved // 0,
    .pillars.mined_tokens_saved // 0,
    .pillars.lean_ctx_tokens_saved // 0
  ] | add] | add),
  total_api_tokens: ([.[].total_api_tokens // 0] | add)
} | "Total est. tokens saved: \(.tokens_saved)\nTotal API tokens:        \(.total_api_tokens)\nSaving ratio:            \(if .total_api_tokens > 0 then (.tokens_saved / .total_api_tokens * 100 * 10 | round / 10 | tostring) + "%" else "n/a (no baseline)" end)"' -r "${DEDUPED}"
echo ""

echo "── Pillar breakdown (totals) ────────────────"
jq -s '{
  rtk_rewrites:       ([.[].pillars.rtk_rewrites       // 0] | add),
  rtk_tokens:         ([.[].pillars.rtk_tokens_saved    // 0] | add),
  cd_prepends:        ([.[].pillars.cd_prepends         // 0] | add),
  cd_tokens:          ([.[].pillars.cd_tokens_saved     // 0] | add),
  skill_activations:  ([.[].pillars.skill_activations   // 0] | add),
  skill_tokens:       ([.[].pillars.skill_tokens_saved  // 0] | add),
  mined_preventions:  ([.[].pillars.mined_preventions   // 0] | add),
  mined_tokens:       ([.[].pillars.mined_tokens_saved  // 0] | add),
  lean_ctx_wraps:     ([.[].pillars.lean_ctx_wraps      // 0] | add),
  lean_ctx_tokens:    ([.[].pillars.lean_ctx_tokens_saved // 0] | add)
}' "${DEDUPED}" | jq -r 'to_entries[] | "\(.key): \(.value)"'
echo ""

echo "── Hook latency (across all users) ──────────"
jq -s '{
  avg_p50_ms: ([.[].hook_latency.p50_ms // 0] | add / length),
  avg_p99_ms: ([.[].hook_latency.p99_ms // 0] | add / length),
  total_invocations: ([.[].hook_latency.count // 0] | add)
}' "${DEDUPED}" | jq -r 'to_entries[] | "\(.key): \(.value)"'
echo ""

echo "── Top skills by activation ─────────────────"
jq -r '.skills[]? | "\(.name)\t\(.activated)\t\(.est_tokens_saved)"' "${DEDUPED}" \
    | sort -t$'\t' -k2 -rn \
    | head -20 \
    | awk -F'\t' 'BEGIN {printf "%-30s %10s %15s\n", "SKILL", "ACTIVATIONS", "TOKENS_SAVED"} {printf "%-30s %10s %15s\n", $1, $2, $3}'
echo ""

echo "── Per-version breakdown ──────────────────────"
jq -s 'group_by(.precc_version) | map({
  version: .[0].precc_version,
  users: length,
  hooks: ([.[].hook_latency.count // 0] | add),
  saved: ([.[] | [.pillars.rtk_tokens_saved, .pillars.cd_tokens_saved, .pillars.skill_tokens_saved, .pillars.mined_tokens_saved, .pillars.lean_ctx_tokens_saved] | map(. // 0) | add] | add),
  api: ([.[].total_api_tokens // 0] | add),
  pct: (if ([.[].total_api_tokens // 0] | add) > 0 then (([.[] | [.pillars.rtk_tokens_saved, .pillars.cd_tokens_saved, .pillars.skill_tokens_saved, .pillars.mined_tokens_saved, .pillars.lean_ctx_tokens_saved] | map(. // 0) | add] | add) / ([.[].total_api_tokens // 0] | add) * 1000 | round / 10) else 0 end)
}) | sort_by(.version) | reverse | .[] | "v\(.version): \(.users) user(s), \(.hooks) hooks, \(.saved) saved, \(.pct)% ratio"' -r "${DEDUPED}"
echo ""

echo "── Unique users (latest report per IP) ────────"
jq -r '._remote_ip // "unknown"' "${DEDUPED}" | sort -u | wc -l | xargs -I{} echo "{} unique IP(s)"
