#!/usr/bin/env bash
set -e

# Colors for Termux terminal feedback
GREEN='\033[0;325m' # approximate bioluminescent green/emerald
CYAN='\033[0;36m'
RESET='\033[0m'

echo -e "${CYAN}=== Ejected Media Release Tagging Utility ===${RESET}"

# Ensure working directory is clean
if [[ -n $(git status -s) ]]; then
    echo "Working directory has uncommitted changes. Stashing or committing first..."
    git status
    read -p "Do you want to auto-commit all changes with a generic message? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        git add .
        git commit -m "chore: prepare release snapshot"
    else
        echo "Aborting. Please commit or stash changes manually."
        exit 1
    fi
fi

# Fetch latest tags
git fetch --tags

# Get current latest tag or default to v0.0.0
LATEST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "v0.0.0")
echo -e "Current latest tag: ${GREEN}$LATEST_TAG${RESET}"

# Strip leading 'v' and split into major, minor, patch
VERSION=${LATEST_TAG#v}
IFS='.' read -r MAJOR MINOR PATCH <<< "$VERSION"

# Default increment is patch
INC_TYPE=${1:-patch}

case "$INC_TYPE" in
    major)
        MAJOR=$((MAJOR + 1))
        MINOR=0
        PATCH=0
        ;;
    minor)
        MINOR=$((MINOR + 1))
        PATCH=0
        ;;
    patch)
        PATCH=$((PATCH + 1))
        ;;
    *)
        echo "Invalid increment type. Use: major, minor, or patch"
        exit 1
        ;;
esac

NEW_TAG="v$MAJOR.$MINOR.$PATCH"
echo -e "New calculated release tag: ${GREEN}$NEW_TAG${RESET}"

read -p "Proceed with creating and pushing tag $NEW_TAG? (y/n) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Cancelled."
    exit 0
fi

# Create annotated tag and push to remote
git tag -a "$NEW_TAG" -m "Release $NEW_TAG via Termux CLI"
git push origin main
git push origin "$NEW_TAG"

echo -e "${GREEN}Successfully pushed tag $NEW_TAG! GitHub Actions release workflow triggered.${RESET}"
