#!/bin/bash
# ============================================
# ACC LMS - Setup Autocommit Cron
# Installs cron job for autocommit every 5 min
# ============================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
AUTOCOMMIT_SCRIPT="$SCRIPT_DIR/autocommit.sh"
CRON_JOB="*/5 * * * * $AUTOCOMMIT_SCRIPT >> $SCRIPT_DIR/autocommit.log 2>&1"

echo "🔧 Setting up autocommit cron job..."

# Check if script exists
if [ ! -f "$AUTOCOMMIT_SCRIPT" ]; then
    echo "❌ Error: autocommit.sh not found at $AUTOCOMMIT_SCRIPT"
    exit 1
fi

# Check if cron job already exists
if crontab -l 2>/dev/null | grep -q "autocommit.sh"; then
    echo "⚠️  Autocommit cron job already exists. Removing old entry..."
    crontab -l 2>/dev/null | grep -v "autocommit.sh" | crontab -
fi

# Add new cron job
(crontab -l 2>/dev/null; echo "$CRON_JOB") | crontab -

echo "✅ Cron job installed successfully!"
echo ""
echo "📋 Current crontab:"
crontab -l | grep "autocommit"
echo ""
echo "📁 Logs will be saved to: $SCRIPT_DIR/autocommit.log"
echo ""
echo "🔑 Make sure to:"
echo "   1. Configure SSH key for GitHub:"
echo "      ssh-keygen -t ed25519 -C \"your@email.com\""
echo "      cat ~/.ssh/id_ed25519.pub  # Add to GitHub"
echo ""
echo "   2. Add remote origin (SSH):"
echo "      git remote add origin git@github.com:YOUR_USER/acc-lms.git"
echo ""
echo "   3. Test SSH connection:"
echo "      ssh -T git@github.com"
echo ""
echo "🛑 To remove cron job:"
echo "   crontab -l | grep -v 'autocommit.sh' | crontab -"
