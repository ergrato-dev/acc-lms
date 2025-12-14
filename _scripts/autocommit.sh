#!/bin/bash
# ============================================
# ACC LMS - Auto Commit Script
# Runs every 5 minutes via cron
# ============================================

set -e

# Configuration
REPO_DIR="/home/epti/Documents/epti-dev/acc-lms"
LOG_FILE="$REPO_DIR/_scripts/autocommit.log"
BRANCH="main"

# Colors for logging
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log() {
    echo -e "[$(date '+%Y-%m-%d %H:%M:%S')] $1" | tee -a "$LOG_FILE"
}

cd "$REPO_DIR" || exit 1

# Check if there are changes
if git diff --quiet && git diff --cached --quiet; then
    log "${YELLOW}No changes to commit${NC}"
    exit 0
fi

# Get list of changed files
CHANGED_FILES=$(git diff --name-only && git diff --cached --name-only)
CHANGED_COUNT=$(echo "$CHANGED_FILES" | wc -l)

# Detect commit type based on changed files
detect_commit_type() {
    local files="$1"
    
    if echo "$files" | grep -qE "^_docs/"; then
        echo "docs"
    elif echo "$files" | grep -qE "^fe/.*\.(tsx?|jsx?|css|scss)$"; then
        echo "feat"
    elif echo "$files" | grep -qE "^be/"; then
        echo "feat"
    elif echo "$files" | grep -qE "^db/migrations/"; then
        echo "feat"
    elif echo "$files" | grep -qE "^db/seeds/"; then
        echo "chore"
    elif echo "$files" | grep -qE "^_scripts/|^\.github/|Dockerfile|docker-compose"; then
        echo "ci"
    elif echo "$files" | grep -qE "\.gitignore|\.env\.example|README|LICENSE|CONTRIBUTING"; then
        echo "chore"
    elif echo "$files" | grep -qE "test|spec|\.test\.|\.spec\."; then
        echo "test"
    elif echo "$files" | grep -qE "fix|bug|patch"; then
        echo "fix"
    else
        echo "chore"
    fi
}

# Detect scope based on changed files
detect_scope() {
    local files="$1"
    
    if echo "$files" | grep -qE "^fe/"; then
        echo "fe"
    elif echo "$files" | grep -qE "^be/"; then
        echo "be"
    elif echo "$files" | grep -qE "^db/"; then
        echo "db"
    elif echo "$files" | grep -qE "^_docs/"; then
        echo "docs"
    elif echo "$files" | grep -qE "^_scripts/"; then
        echo "scripts"
    elif echo "$files" | grep -qE "^_assets/"; then
        echo "assets"
    else
        echo "root"
    fi
}

# Generate what/for/impact description
generate_description() {
    local files="$1"
    local type="$2"
    local scope="$3"
    
    # What: Brief description of changes
    local what=""
    case "$scope" in
        fe) what="update frontend components and styles" ;;
        be) what="update backend services and logic" ;;
        db) what="update database schemas and migrations" ;;
        docs) what="update project documentation" ;;
        scripts) what="update development scripts" ;;
        assets) what="update project assets" ;;
        root) what="update project configuration" ;;
    esac
    
    # For: Purpose of changes
    local for_desc="improve project structure and maintainability"
    
    # Impact: What this affects
    local impact=""
    case "$scope" in
        fe) impact="affects user interface and experience" ;;
        be) impact="affects API endpoints and business logic" ;;
        db) impact="affects data persistence and queries" ;;
        docs) impact="affects developer onboarding and reference" ;;
        scripts) impact="affects development workflow" ;;
        assets) impact="affects branding and visual identity" ;;
        root) impact="affects project setup and configuration" ;;
    esac
    
    echo -e "What: $what\nFor: $for_desc\nImpact: $impact"
}

# Build commit message
COMMIT_TYPE=$(detect_commit_type "$CHANGED_FILES")
SCOPE=$(detect_scope "$CHANGED_FILES")
DESCRIPTION=$(generate_description "$CHANGED_FILES" "$COMMIT_TYPE" "$SCOPE")

# Short summary for first line
SUMMARY="auto-update $CHANGED_COUNT file(s)"

# Full conventional commit message
COMMIT_MSG="$COMMIT_TYPE($SCOPE): $SUMMARY

$DESCRIPTION

Files changed: $CHANGED_COUNT
Timestamp: $(date '+%Y-%m-%d %H:%M:%S')
---
[autocommit]"

# Stage all changes
git add -A

# Commit with message
git commit -m "$COMMIT_MSG"

log "${GREEN}✓ Committed: $COMMIT_TYPE($SCOPE): $SUMMARY${NC}"

# Push to remote (SSH)
if git remote | grep -q "origin"; then
    git push origin "$BRANCH" 2>&1 | tee -a "$LOG_FILE"
    log "${GREEN}✓ Pushed to origin/$BRANCH${NC}"
else
    log "${YELLOW}⚠ No remote 'origin' configured. Skipping push.${NC}"
    log "${YELLOW}  Add remote with: git remote add origin git@github.com:USER/acc-lms.git${NC}"
fi

log "${GREEN}✓ Autocommit completed successfully${NC}"
