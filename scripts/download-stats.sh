#!/usr/bin/env bash
# download-stats.sh — Show download counts per PRECC release
#
# Usage: bash scripts/download-stats.sh [--repo owner/repo]

set -euo pipefail

REPO="${1:-peria-ai/precc-cc}"

echo "Download stats for ${REPO}"
echo "========================================"
printf "%-12s %8s %s\n" "VERSION" "DOWNLOADS" "DATE"
echo "----------------------------------------"

gh api "repos/${REPO}/releases" --paginate --jq '
  .[] |
  {
    tag: .tag_name,
    date: (.published_at | split("T")[0]),
    total: ([.assets[].download_count] | add // 0)
  }
' | jq -s '
  group_by(.tag) |
  map({
    tag: .[0].tag,
    date: .[0].date,
    total: (map(.total) | add)
  }) |
  sort_by(.date) |
  reverse |
  .[]
' | jq -r '"\(.tag)\t\(.total)\t\(.date)"' | while IFS=$'\t' read -r tag total date; do
  printf "%-12s %8d   %s\n" "${tag}" "${total}" "${date}"
done

echo "----------------------------------------"

# Grand total
GRAND=$(gh api "repos/${REPO}/releases" --paginate --jq '
  [.[].assets[].download_count] | add // 0
')
printf "%-12s %8d\n" "TOTAL" "${GRAND}"
