#!/bin/bash
# deploy.sh — Build, tag, release, and push PRECC to yijunyu/precc-cc
#
# Usage:
#   bash scripts/deploy.sh <version>          # e.g. bash scripts/deploy.sh v0.7.0
#   bash scripts/deploy.sh <version> --title "Release title"
#   bash scripts/deploy.sh <version> --notes "Release notes"

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
PUBLIC_REMOTE="public"
PUBLIC_REPO="yijunyu/precc-cc"

# ---------------------------------------------------------------------------
# Parse arguments
# ---------------------------------------------------------------------------
VERSION="${1:-}"
TITLE=""
NOTES=""

if [[ -z "$VERSION" || "$VERSION" == "--help" || "$VERSION" == "-h" ]]; then
    echo "Usage: bash scripts/deploy.sh <version> [--title '...'] [--notes '...']" >&2
    echo "  e.g. bash scripts/deploy.sh v0.7.0 --title 'PRECC v0.7.0 — New Feature'" >&2
    exit 0
fi

shift || true
while [[ $# -gt 0 ]]; do
    case "$1" in
        --title) TITLE="$2"; shift 2 ;;
        --notes) NOTES="$2"; shift 2 ;;
        *) echo "Unknown argument: $1" >&2; exit 1 ;;
    esac
done

# Ensure version starts with 'v'
[[ "$VERSION" == v* ]] || VERSION="v${VERSION}"

echo "==> Deploying PRECC ${VERSION} to ${PUBLIC_REPO}"

# ---------------------------------------------------------------------------
# Step 1: Sync public files via deploy-demo
# ---------------------------------------------------------------------------
echo "==> Step 1: Syncing public repo..."
cd "$REPO_DIR"
../precc/target/release/deploy-demo --config deploy.toml

# ---------------------------------------------------------------------------
# Step 2: Build release binaries
# ---------------------------------------------------------------------------
echo "==> Step 2: Building release binaries (glibc 2.17 compatible)..."
cargo zigbuild --release \
    -p precc-hook \
    -p precc-cli \
    -p precc-miner \
    --target x86_64-unknown-linux-gnu.2.17 \
    --target aarch64-unknown-linux-gnu.2.17

# ---------------------------------------------------------------------------
# Step 3: Package archives
# ---------------------------------------------------------------------------
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT
ASSETS=()

for TARGET in x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu; do
    STAGING="precc-${VERSION}-${TARGET}"
    ARCHIVE="${STAGING}.tar.gz"
    echo "==> Step 3: Packaging ${ARCHIVE}..."
    mkdir -p "${TMP}/${STAGING}"
    cp "target/${TARGET}/release/precc"       "${TMP}/${STAGING}/"
    cp "target/${TARGET}/release/precc-hook"  "${TMP}/${STAGING}/"
    cp "target/${TARGET}/release/precc-miner" "${TMP}/${STAGING}/"
    tar -czf "${TMP}/${ARCHIVE}" -C "$TMP" "$STAGING"
    ASSETS+=("${TMP}/${ARCHIVE}")
done

# ---------------------------------------------------------------------------
# Step 4: Push main to public repo and tag
# ---------------------------------------------------------------------------
echo "==> Step 4: Tagging ${VERSION} and pushing tag to public repo..."
git tag "${VERSION}" 2>/dev/null || { echo "Tag ${VERSION} already exists locally — delete it first."; exit 1; }
git push "${PUBLIC_REMOTE}" "${VERSION}"

# ---------------------------------------------------------------------------
# Step 5: Create GitHub release and upload binary asset only
# ---------------------------------------------------------------------------
echo "==> Step 5: Creating GitHub release..."
RELEASE_TITLE="${TITLE:-PRECC ${VERSION}}"
RELEASE_NOTES="${NOTES:-## ${VERSION}

- See commit history for changes.

### Install
\`\`\`bash
curl -fsSL https://raw.githubusercontent.com/${PUBLIC_REPO}/main/scripts/install.sh | bash
precc init
\`\`\`}"

gh release create "${VERSION}" \
    --repo "${PUBLIC_REPO}" \
    --title "${RELEASE_TITLE}" \
    --notes "${RELEASE_NOTES}" \
    --latest \
    "${ASSETS[@]}"

echo "==> Step 6: Committing and pushing private repo..."
git commit -am "deployed ${VERSION}" || echo "(nothing to commit)"
git push origin master
git push origin demo

echo ""
echo "Done. PRECC ${VERSION} is live at https://github.com/${PUBLIC_REPO}/releases/tag/${VERSION}"
