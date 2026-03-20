#!/bin/bash
# release.sh — Bump version, commit, push, and deploy PRECC in one command.
#
# This is the single workflow script that replaces the manual sequence of:
#   1. Edit 5 version files
#   2. git add + commit
#   3. git push
#   4. bash scripts/deploy.sh <version>
#
# Usage:
#   bash scripts/release.sh <version> [-m "commit message"] [--title "..."] [--notes "..."]
#   bash scripts/release.sh patch          # auto-bump 0.1.8 → 0.1.9
#   bash scripts/release.sh minor          # auto-bump 0.1.8 → 0.2.0
#   bash scripts/release.sh major          # auto-bump 0.1.8 → 1.0.0
#   bash scripts/release.sh --dry-run ...  # show what would change without doing it
#
# Examples:
#   bash scripts/release.sh v0.2.0 -m "feat: new advisor" --title "PRECC v0.2.0"
#   bash scripts/release.sh patch

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_DIR"

# ---------------------------------------------------------------------------
# Colour helpers (disabled if not a terminal)
# ---------------------------------------------------------------------------
if [[ -t 1 ]]; then
    GREEN='\033[0;32m'; YELLOW='\033[1;33m'; RED='\033[0;31m'; BOLD='\033[1m'; RESET='\033[0m'
else
    GREEN=''; YELLOW=''; RED=''; BOLD=''; RESET=''
fi
info()  { echo -e "${GREEN}==>${RESET} $*"; }
warn()  { echo -e "${YELLOW}warn:${RESET} $*"; }
die()   { echo -e "${RED}error:${RESET} $*" >&2; exit 1; }

# ---------------------------------------------------------------------------
# Parse arguments
# ---------------------------------------------------------------------------
VERSION_ARG=""
COMMIT_MSG=""
TITLE=""
NOTES=""
DRY_RUN=false

show_help() {
    cat <<'USAGE'
Usage: bash scripts/release.sh <version> [options]

Version:
  v0.2.0           Explicit version (with or without 'v' prefix)
  patch            Auto-bump patch: 0.1.8 → 0.1.9
  minor            Auto-bump minor: 0.1.8 → 0.2.0
  major            Auto-bump major: 0.1.8 → 1.0.0

Options:
  -m "message"     Custom commit message (default: auto-generated)
  --title "..."    GitHub release title (default: "PRECC v<version>")
  --notes "..."    GitHub release notes (default: auto-generated)
  --dry-run        Show what would change without modifying anything
  -h, --help       Show this help
USAGE
    exit 0
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        -h|--help)    show_help ;;
        --dry-run)    DRY_RUN=true; shift ;;
        -m)           COMMIT_MSG="$2"; shift 2 ;;
        --title)      TITLE="$2"; shift 2 ;;
        --notes)      NOTES="$2"; shift 2 ;;
        -*)           die "Unknown option: $1 (see --help)" ;;
        *)
            [[ -z "$VERSION_ARG" ]] || die "Multiple version arguments: '$VERSION_ARG' and '$1'"
            VERSION_ARG="$1"; shift ;;
    esac
done

[[ -n "$VERSION_ARG" ]] || die "Missing version argument (see --help)"

# ---------------------------------------------------------------------------
# Version files — all locations that contain a version string
# ---------------------------------------------------------------------------
VERSION_FILES=(
    "Cargo.toml"                                          # workspace version
)

# ---------------------------------------------------------------------------
# Read current version from workspace Cargo.toml
# ---------------------------------------------------------------------------
current_version() {
    grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/'
}

CURRENT="$(current_version)"
[[ -n "$CURRENT" ]] || die "Could not read current version from Cargo.toml"

# ---------------------------------------------------------------------------
# Resolve target version (explicit or semver bump)
# ---------------------------------------------------------------------------
bump_version() {
    local cur="$1" part="$2"
    IFS='.' read -r major minor patch <<< "$cur"
    case "$part" in
        major) echo "$((major + 1)).0.0" ;;
        minor) echo "${major}.$((minor + 1)).0" ;;
        patch) echo "${major}.${minor}.$((patch + 1))" ;;
    esac
}

case "$VERSION_ARG" in
    patch|minor|major)
        NEW_VERSION="$(bump_version "$CURRENT" "$VERSION_ARG")"
        ;;
    *)
        NEW_VERSION="${VERSION_ARG#v}"  # strip leading 'v' if present
        ;;
esac

TAG="v${NEW_VERSION}"

# Sanity check
[[ "$NEW_VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]] || \
    die "Invalid version: '$NEW_VERSION' (expected X.Y.Z)"

if [[ "$NEW_VERSION" == "$CURRENT" ]]; then
    warn "Version is already ${CURRENT} — skipping bump, proceeding to tag/push/deploy"
fi

# ---------------------------------------------------------------------------
# Pre-flight checks
# ---------------------------------------------------------------------------
info "Release: ${CURRENT} → ${BOLD}${NEW_VERSION}${RESET} (tag: ${TAG})"

# Ensure working tree is clean (aside from version bumps we're about to make)
if ! git diff --quiet HEAD 2>/dev/null; then
    warn "Working tree has uncommitted changes — they will be included in the release commit"
    echo ""
    git diff --stat
    echo ""
    if [[ "$DRY_RUN" == false ]]; then
        read -r -p "Continue? [y/N] " confirm
        [[ "$confirm" =~ ^[Yy]$ ]] || exit 0
    fi
fi

# Check deploy.sh exists
[[ -f "$SCRIPT_DIR/deploy.sh" ]] || die "scripts/deploy.sh not found"

# ---------------------------------------------------------------------------
# Bump versions in all files
# ---------------------------------------------------------------------------
info "Bumping version in ${#VERSION_FILES[@]} files..."

bump_file() {
    local file="$1" old="$2" new="$3"

    [[ -f "$file" ]] || { warn "Skipping missing file: $file"; return; }

    case "$file" in
        *.json)
            # JSON: "version": "X.Y.Z"
            sed -i "s/\"version\": \"${old}\"/\"version\": \"${new}\"/" "$file"
            ;;
        *.md)
            # YAML frontmatter: version: X.Y.Z
            sed -i "s/^version: ${old}/version: ${new}/" "$file"
            ;;
        *.toml)
            # TOML: version = "X.Y.Z" (first occurrence only for workspace Cargo.toml)
            sed -i "0,/version = \"${old}\"/s//version = \"${new}\"/" "$file"
            ;;
    esac

    # Verify the bump took effect
    if grep -q "$new" "$file"; then
        echo "  ✓ $file"
    else
        # Fallback: the file might have had an older version (not $CURRENT)
        # Try replacing any semver-looking version string in the version field
        case "$file" in
            *.json)
                sed -i "s/\"version\": \"[0-9]*\.[0-9]*\.[0-9]*\"/\"version\": \"${new}\"/" "$file"
                ;;
            *.md)
                sed -i "s/^version: [0-9]*\.[0-9]*\.[0-9]*/version: ${new}/" "$file"
                ;;
            *.toml)
                sed -i "0,/version = \"[0-9]*\.[0-9]*\.[0-9]*\"/s//version = \"${new}\"/" "$file"
                ;;
        esac
        if grep -q "$new" "$file"; then
            echo "  ✓ $file (was out of sync, now fixed)"
        else
            warn "  ✗ $file — could not bump version (check manually)"
        fi
    fi
}

if [[ "$DRY_RUN" == true ]]; then
    echo "  (dry run — no files modified)"
    for f in "${VERSION_FILES[@]}"; do
        echo "  → would bump: $f"
    done
else
    for f in "${VERSION_FILES[@]}"; do
        bump_file "$f" "$CURRENT" "$NEW_VERSION"
    done
fi

# ---------------------------------------------------------------------------
# Pre-commit gate
# ---------------------------------------------------------------------------
if [[ "$DRY_RUN" == false ]]; then
    info "Running pre-commit checks..."
    cargo fmt --all --check || die "cargo fmt check failed — run 'cargo fmt --all' first"
    cargo clippy --all-targets 2>&1 | tail -3
    cargo test 2>&1 | tail -5
    info "Pre-commit gate passed"
fi

# ---------------------------------------------------------------------------
# Commit
# ---------------------------------------------------------------------------
if [[ -z "$COMMIT_MSG" ]]; then
    COMMIT_MSG="release: bump to ${TAG}"
fi

if [[ "$DRY_RUN" == true ]]; then
    info "Would commit with message: ${COMMIT_MSG}"
    info "Would push to origin/master"
    info "Would run: bash scripts/deploy.sh ${TAG} ..."
    echo ""
    echo "Dry run complete. No changes made."
    exit 0
fi

info "Committing..."
git add "${VERSION_FILES[@]}"
# Also stage any other modified/new files (e.g. source code changes)
git add -u
git diff --cached --quiet && info "Nothing to commit (version already bumped)" || \
    git commit -m "$COMMIT_MSG" || die "Commit failed"

# ---------------------------------------------------------------------------
# Push
# ---------------------------------------------------------------------------
info "Pushing to origin/master..."
git push origin master

# ---------------------------------------------------------------------------
# Deploy (calls the existing deploy.sh)
# ---------------------------------------------------------------------------
DEPLOY_ARGS=("$TAG")
[[ -n "$TITLE" ]] && DEPLOY_ARGS+=(--title "$TITLE")
[[ -n "$NOTES" ]] && DEPLOY_ARGS+=(--notes "$NOTES")

info "Deploying via scripts/deploy.sh ${DEPLOY_ARGS[*]}..."
bash "$SCRIPT_DIR/deploy.sh" "${DEPLOY_ARGS[@]}"

echo ""
info "${BOLD}Release ${TAG} complete!${RESET}"
