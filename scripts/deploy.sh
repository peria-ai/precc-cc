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
# Step 0: Bump Cargo.toml workspace version to match the release tag
# ---------------------------------------------------------------------------
BARE_VERSION="${VERSION#v}"   # strip leading 'v'
sed -i "s/^version = \".*\"/version = \"${BARE_VERSION}\"/" Cargo.toml
echo "==> Bumped Cargo.toml version to ${BARE_VERSION}"

# ---------------------------------------------------------------------------
# Step 1: Deploy public repo as a single squashed commit (orphan branch)
#         Only files listed in deploy.toml [mappings] are included.
# ---------------------------------------------------------------------------
echo "==> Step 1: Building squashed deploy commit..."
cd "$REPO_DIR"

DEPLOY_BRANCH="$(grep '^remote_branch' deploy.toml | sed 's/.*= *"\(.*\)"/\1/')"
COMMIT_MSG="$(grep '^commit_message' deploy.toml | sed 's/.*= *"\(.*\)"/\1/')"

# Save current branch to restore later
ORIGINAL_BRANCH="$(git rev-parse --abbrev-ref HEAD)"

# Create a temporary orphan branch (no history)
git checkout --orphan _deploy_tmp
git rm -rf --cached . > /dev/null 2>&1

# Stage only the mapped files into a temp working tree
DEPLOY_TREE="$(mktemp -d)"
while IFS='=' read -r src dst; do
    src="$(echo "$src" | sed 's/^[[:space:]]*"//;s/"[[:space:]]*$//')"
    dst="$(echo "$dst" | sed 's/^[[:space:]]*"//;s/"[[:space:]]*$//')"
    [[ -z "$src" || "$src" == "["* || "$src" == "#"* ]] && continue
    [[ "$src" == *"="* ]] && continue

    if [[ -f "$src" ]]; then
        mkdir -p "$DEPLOY_TREE/$(dirname "$dst")"
        cp "$src" "$DEPLOY_TREE/$dst"
        echo "  $src → $dst"
    else
        echo "  (skipped missing: $src)"
    fi
done < <(sed -n '/^\[mappings\]/,/^\[/p' deploy.toml | grep -v '^\[')

GIT_WORK_TREE="$DEPLOY_TREE" git add -A
git commit -m "${COMMIT_MSG} ${VERSION}"

# Push the squashed orphan commit to the public remote
echo "==> Pushing to ${PUBLIC_REMOTE}/${DEPLOY_BRANCH}..."
git push "${PUBLIC_REMOTE}" "_deploy_tmp:${DEPLOY_BRANCH}" --force

# Clean up: restore original branch, delete temp branch and tree
git checkout -f "$ORIGINAL_BRANCH"
git branch -D _deploy_tmp
rm -rf "$DEPLOY_TREE"

# ---------------------------------------------------------------------------
# Step 2: Build binaries
# ---------------------------------------------------------------------------
# PRECC_LICENSE_SECRET is baked into the binary at compile time via option_env!().
# Set it via the environment before calling this script, e.g.:
#   PRECC_LICENSE_SECRET=<secret> bash scripts/deploy.sh v0.2.0
# If not set, builds fall back to the public default (community/open builds only).

LINUX_TARGETS=()
MACOS_TARGETS=()

# Step 2a: Build Linux binaries
if command -v cargo-zigbuild &>/dev/null; then
    echo "==> Step 2a: Building Linux binaries (zigbuild, glibc 2.17)..."
    PRECC_LICENSE_SECRET="${PRECC_LICENSE_SECRET:-}" \
    cargo zigbuild --release \
        -p precc-hook \
        -p precc-cli \
        -p precc-miner \
        --target x86_64-unknown-linux-gnu.2.17 \
        --target aarch64-unknown-linux-gnu.2.17
    LINUX_TARGETS+=(x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu)
else
    echo "==> Step 2a: Building native Linux binary (cargo-zigbuild not found)..."
    PRECC_LICENSE_SECRET="${PRECC_LICENSE_SECRET:-}" \
    cargo build --release \
        -p precc-hook \
        -p precc-cli \
        -p precc-miner
    # Native build puts binaries in target/release/, symlink to target/<triple>/release/
    NATIVE_TARGET="$(rustc -vV | grep host | sed 's/host: //')"
    mkdir -p "target/${NATIVE_TARGET}/release"
    for bin in precc precc-hook precc-miner; do
        if [[ -f "target/release/${bin}" ]]; then
            cp "target/release/${bin}" "target/${NATIVE_TARGET}/release/${bin}"
        fi
    done
    LINUX_TARGETS+=("${NATIVE_TARGET}")
fi

# Step 2b: Build macOS binaries (osxcross via Docker, skipped if unavailable)
DOCKER_IMAGE="joseluisq/rust-linux-darwin-builder:latest"
OSXCROSS_BIN="/usr/local/osxcross/target/bin"
CARGO_BIN="/root/.cargo/bin"

build_macos() {
    local TARGET="$1"
    local TRIPLE="${TARGET//-/_}"
    local DARWIN_TRIPLE
    if [[ "$TARGET" == "aarch64-apple-darwin" ]]; then
        DARWIN_TRIPLE="aarch64-apple-darwin22.4"
    else
        DARWIN_TRIPLE="x86_64-apple-darwin22.4"
    fi
    echo "==> Step 2b: Building ${TARGET} via Docker (osxcross)..."
    sg docker -c "docker run --rm \
        -v '$(pwd):/workspace' \
        -w /workspace \
        -e PATH=${CARGO_BIN}:${OSXCROSS_BIN}:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin \
        -e PRECC_LICENSE_SECRET="${PRECC_LICENSE_SECRET:-}" \
        -e CC_${TRIPLE//-/_}=${DARWIN_TRIPLE}-clang \
        -e CXX_${TRIPLE//-/_}=${DARWIN_TRIPLE}-clang++ \
        -e AR_${TRIPLE//-/_}=${DARWIN_TRIPLE}-ar \
        -e RANLIB_${TRIPLE//-/_}=${DARWIN_TRIPLE}-ranlib \
        -e CARGO_TARGET_$(echo "${TRIPLE}" | tr '[:lower:]' '[:upper:]')_LINKER=${DARWIN_TRIPLE}-clang \
        -e OPENSSL_BUILD_RANLIB=${DARWIN_TRIPLE}-ranlib \
        ${DOCKER_IMAGE} \
        sh -c 'cargo build --release -p precc-hook -p precc-cli -p precc-miner --target ${TARGET} 2>&1 | tail -5'"
    MACOS_TARGETS+=("$TARGET")
}

if command -v docker &>/dev/null && docker info &>/dev/null 2>&1; then
    build_macos "x86_64-apple-darwin"
    build_macos "aarch64-apple-darwin"
else
    echo "==> Step 2b: Skipping macOS builds (Docker not available)"
fi

# ---------------------------------------------------------------------------
# Step 3: Package archives
# ---------------------------------------------------------------------------
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT
ASSETS=()
ALL_TARGETS=("${LINUX_TARGETS[@]}" "${MACOS_TARGETS[@]}")

for TARGET in "${ALL_TARGETS[@]}"; do
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

# Generate SHA256SUMS for all archives
echo "==> Generating SHA256SUMS..."
(cd "$TMP" && sha256sum *.tar.gz > SHA256SUMS 2>/dev/null || shasum -a 256 *.tar.gz > SHA256SUMS)
ASSETS+=("${TMP}/SHA256SUMS")

# ---------------------------------------------------------------------------
# Step 4: Get public repo SHA (already pushed in Step 1)
# ---------------------------------------------------------------------------
echo "==> Step 4: Resolving public repo SHA..."
PUBLIC_SHA=$(git ls-remote "${PUBLIC_REMOTE}" "refs/heads/${DEPLOY_BRANCH}" | cut -f1)

# Delete existing release and tag on public repo if present (idempotent redeploy)
gh release delete "${VERSION}" --repo "${PUBLIC_REPO}" --yes 2>/dev/null || true
git push "${PUBLIC_REMOTE}" ":refs/tags/${VERSION}" 2>/dev/null || true

# ---------------------------------------------------------------------------
# Step 5: Create GitHub release pinned to public SHA (no tag pushed = no auto source archives)
# ---------------------------------------------------------------------------
echo "==> Step 5: Creating GitHub release ${VERSION} at ${PUBLIC_SHA}..."
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
    --target "${PUBLIC_SHA}" \
    "${ASSETS[@]}"

echo "==> Step 6: Committing and pushing private repo..."
git commit -am "deployed ${VERSION}" || echo "(nothing to commit)"
git push origin master
git push origin demo 2>/dev/null || true

echo ""
echo "Done. PRECC ${VERSION} is live at https://github.com/${PUBLIC_REPO}/releases/tag/${VERSION}"
