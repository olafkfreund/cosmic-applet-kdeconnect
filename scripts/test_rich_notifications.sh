#!/usr/bin/env bash
#
# test_rich_notifications.sh - Manual Test Script for Rich Notifications Feature
#
# =============================================================================
# USAGE
# =============================================================================
#
# This script tests the Rich Notifications feature of COSMIC Connect by sending
# various notification types through the desktop notification system.
#
# Prerequisites:
#   - libnotify (provides notify-send)
#   - gdbus (part of glib2, usually pre-installed)
#   - A notification daemon running (COSMIC, dunst, mako, etc.)
#   - COSMIC Connect daemon running with notifications plugin enabled
#
# Usage:
#   ./test_rich_notifications.sh [test_number]
#
# Examples:
#   ./test_rich_notifications.sh       # Run all tests interactively
#   ./test_rich_notifications.sh 1     # Run only test 1 (basic notification)
#   ./test_rich_notifications.sh 3     # Run only test 3 (image hint)
#
# Expected Results:
#   Each test sends a notification that should:
#   1. Appear on the local desktop
#   2. Be captured by COSMIC Connect daemon
#   3. Be forwarded to connected devices (if any)
#
# =============================================================================

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
DELAY_BETWEEN_TESTS=3  # Seconds to wait between tests

# -----------------------------------------------------------------------------
# Helper Functions
# -----------------------------------------------------------------------------

print_header() {
    echo -e "\n${BLUE}=============================================================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}=============================================================================${NC}\n"
}

print_test() {
    local test_num=$1
    local test_name=$2
    echo -e "${GREEN}[TEST $test_num]${NC} $test_name"
}

print_info() {
    echo -e "${YELLOW}INFO:${NC} $1"
}

print_expected() {
    echo -e "${YELLOW}EXPECTED:${NC} $1"
}

print_command() {
    echo -e "${BLUE}COMMAND:${NC} $1"
}

wait_for_user() {
    if [[ -z "${BATCH_MODE:-}" ]]; then
        echo ""
        read -rp "Press Enter to continue to next test..."
    else
        sleep "$DELAY_BETWEEN_TESTS"
    fi
}

check_dependencies() {
    local missing=()
    
    if ! command -v notify-send &> /dev/null; then
        missing+=("notify-send (libnotify)")
    fi
    
    if ! command -v gdbus &> /dev/null; then
        missing+=("gdbus (glib2)")
    fi
    
    if [[ ${#missing[@]} -gt 0 ]]; then
        echo -e "${RED}ERROR: Missing dependencies:${NC}"
        for dep in "${missing[@]}"; do
            echo "  - $dep"
        done
        echo ""
        echo "Install on NixOS: nix-shell -p libnotify glib"
        echo "Install on Ubuntu: sudo apt install libnotify-bin"
        echo "Install on Fedora: sudo dnf install libnotify"
        exit 1
    fi
}

# -----------------------------------------------------------------------------
# Test Cases
# -----------------------------------------------------------------------------

test_1_basic_notification() {
    print_test 1 "Basic Notification"
    echo ""
    print_info "Sends a simple notification with title and body text."
    print_info "This is the most common notification type."
    print_expected "A notification popup with 'Test Title' and 'This is a basic notification body'"
    echo ""
    
    print_command 'notify-send "Test Title" "This is a basic notification body"'
    notify-send "Test Title" "This is a basic notification body"
    
    echo -e "${GREEN}Notification sent!${NC}"
}

test_2_notification_with_icon() {
    print_test 2 "Notification with Icon"
    echo ""
    print_info "Sends a notification with an application icon."
    print_info "Icons can be specified by name (from icon theme) or file path."
    print_expected "A notification with Firefox icon, title 'Firefox', body 'Page loaded successfully'"
    echo ""
    
    # Try common icons that should exist on most systems
    local icons=("firefox" "web-browser" "applications-internet" "dialog-information")
    local icon_to_use="dialog-information"  # fallback
    
    for icon in "${icons[@]}"; do
        if [[ -f "/usr/share/icons/hicolor/48x48/apps/${icon}.png" ]] || \
           [[ -f "/usr/share/icons/Adwaita/48x48/apps/${icon}.png" ]] || \
           [[ -n "$(find /usr/share/icons -name "${icon}.*" 2>/dev/null | head -1)" ]]; then
            icon_to_use="$icon"
            break
        fi
    done
    
    print_command "notify-send -i $icon_to_use \"Application\" \"Page loaded successfully\""
    notify-send -i "$icon_to_use" "Application" "Page loaded successfully"
    
    echo -e "${GREEN}Notification with icon sent!${NC}"
}

test_3_notification_with_image_hint() {
    print_test 3 "Notification with Image Hint (via gdbus)"
    echo ""
    print_info "Sends a notification with an image using the 'image-path' hint."
    print_info "This uses D-Bus directly to set the image hint properly."
    print_info "The image-path hint is used for inline images in notifications."
    print_expected "A notification with an embedded image (if image exists)"
    echo ""
    
    # Find a suitable image for testing
    local test_image=""
    local search_paths=(
        "/usr/share/icons/hicolor/256x256/apps"
        "/usr/share/icons/hicolor/128x128/apps"
        "/usr/share/icons/hicolor/48x48/apps"
        "/usr/share/pixmaps"
        "$HOME/.local/share/icons"
    )
    
    for path in "${search_paths[@]}"; do
        if [[ -d "$path" ]]; then
            test_image=$(find "$path" -name "*.png" 2>/dev/null | head -1)
            if [[ -n "$test_image" ]]; then
                break
            fi
        fi
    done
    
    if [[ -z "$test_image" ]]; then
        print_info "No suitable image found for testing. Creating a test image..."
        test_image="/tmp/cosmic-connect-test-image.png"
        # Create a simple colored square using ImageMagick if available, otherwise skip
        if command -v convert &> /dev/null; then
            convert -size 64x64 xc:blue "$test_image"
        else
            echo -e "${YELLOW}WARNING: No test image available and ImageMagick not installed.${NC}"
            echo "Skipping image-path hint test."
            return
        fi
    fi
    
    print_info "Using test image: $test_image"
    
    # Send notification via gdbus with image-path hint
    # The hint format is: {'image-path': <'path/to/image'>}
    print_command "gdbus call --session --dest org.freedesktop.Notifications ..."
    
    gdbus call --session \
        --dest org.freedesktop.Notifications \
        --object-path /org/freedesktop/Notifications \
        --method org.freedesktop.Notifications.Notify \
        "COSMIC Connect Test" \
        0 \
        "" \
        "Image Notification" \
        "This notification includes an image via image-path hint" \
        "[]" \
        "{'image-path': <'$test_image'>}" \
        5000
    
    echo -e "${GREEN}Notification with image hint sent!${NC}"
}

test_4_notification_with_markup() {
    print_test 4 "Notification with HTML/Pango Formatting"
    echo ""
    print_info "Sends a notification with Pango markup for rich text formatting."
    print_info "Supported tags: <b>bold</b>, <i>italic</i>, <u>underline</u>, <a>links</a>"
    print_info "Note: Not all notification daemons support markup."
    print_expected "A notification with bold, italic, and underlined text"
    echo ""
    
    local body="<b>Bold text</b>, <i>italic text</i>, and <u>underlined text</u>.
    
Visit <a href=\"https://github.com\">GitHub</a> for more info."
    
    print_command 'notify-send "Rich Text Notification" "<b>Bold</b>, <i>italic</i>..."'
    notify-send "Rich Text Notification" "$body"
    
    echo -e "${GREEN}Notification with markup sent!${NC}"
    print_info "If formatting doesn't appear, your notification daemon may not support markup."
}

test_5_notification_urgency_levels() {
    print_test 5 "Notification Urgency Levels"
    echo ""
    print_info "Sends notifications with different urgency levels."
    print_info "Urgency levels: low, normal, critical"
    print_info "Critical notifications typically don't auto-dismiss and may have different styling."
    print_expected "Three notifications with different urgency levels"
    echo ""
    
    # Low urgency
    print_info "Sending LOW urgency notification..."
    print_command 'notify-send -u low "Low Urgency" "This is a low priority message"'
    notify-send -u low "Low Urgency" "This is a low priority message - may auto-dismiss quickly"
    sleep 1
    
    # Normal urgency
    print_info "Sending NORMAL urgency notification..."
    print_command 'notify-send -u normal "Normal Urgency" "This is a normal priority message"'
    notify-send -u normal "Normal Urgency" "This is a normal priority message - standard behavior"
    sleep 1
    
    # Critical urgency
    print_info "Sending CRITICAL urgency notification..."
    print_command 'notify-send -u critical "Critical Urgency" "This is a critical priority message"'
    notify-send -u critical "Critical Urgency" "This is a critical priority message - requires attention!"
    
    echo -e "${GREEN}All urgency level notifications sent!${NC}"
    print_info "Critical notifications may require manual dismissal."
}

test_6_notification_with_actions() {
    print_test 6 "Notification with Actions (Bonus)"
    echo ""
    print_info "Sends a notification with clickable action buttons."
    print_info "Actions are defined as pairs: action_key, action_label"
    print_expected "A notification with 'Open' and 'Dismiss' buttons"
    echo ""
    
    print_command "gdbus call ... with actions ['open', 'Open', 'dismiss', 'Dismiss']"
    
    gdbus call --session \
        --dest org.freedesktop.Notifications \
        --object-path /org/freedesktop/Notifications \
        --method org.freedesktop.Notifications.Notify \
        "COSMIC Connect Test" \
        0 \
        "dialog-question" \
        "Action Notification" \
        "This notification has action buttons. Click one!" \
        "['open', 'Open', 'dismiss', 'Dismiss']" \
        "{}" \
        10000
    
    echo -e "${GREEN}Notification with actions sent!${NC}"
    print_info "Click an action button to see the response (if supported by your daemon)."
}

test_7_notification_with_timeout() {
    print_test 7 "Notification with Custom Timeout (Bonus)"
    echo ""
    print_info "Sends notifications with different display durations."
    print_info "Timeout is specified in milliseconds. 0 means never expire."
    print_expected "A notification that stays for 10 seconds"
    echo ""
    
    print_command 'notify-send -t 10000 "Long Notification" "This stays for 10 seconds"'
    notify-send -t 10000 "Long Notification" "This notification stays visible for 10 seconds"
    
    echo -e "${GREEN}Notification with custom timeout sent!${NC}"
}

# -----------------------------------------------------------------------------
# Main Script
# -----------------------------------------------------------------------------

main() {
    print_header "COSMIC Connect - Rich Notifications Test Suite"
    
    echo "This script tests various notification features for COSMIC Connect."
    echo "Make sure the COSMIC Connect daemon is running to capture notifications."
    echo ""
    
    check_dependencies
    
    # Check if a specific test was requested
    if [[ $# -gt 0 ]]; then
        case "$1" in
            1) test_1_basic_notification ;;
            2) test_2_notification_with_icon ;;
            3) test_3_notification_with_image_hint ;;
            4) test_4_notification_with_markup ;;
            5) test_5_notification_urgency_levels ;;
            6) test_6_notification_with_actions ;;
            7) test_7_notification_with_timeout ;;
            all)
                BATCH_MODE=1
                test_1_basic_notification
                sleep "$DELAY_BETWEEN_TESTS"
                test_2_notification_with_icon
                sleep "$DELAY_BETWEEN_TESTS"
                test_3_notification_with_image_hint
                sleep "$DELAY_BETWEEN_TESTS"
                test_4_notification_with_markup
                sleep "$DELAY_BETWEEN_TESTS"
                test_5_notification_urgency_levels
                sleep "$DELAY_BETWEEN_TESTS"
                test_6_notification_with_actions
                sleep "$DELAY_BETWEEN_TESTS"
                test_7_notification_with_timeout
                ;;
            *)
                echo "Unknown test: $1"
                echo "Valid options: 1, 2, 3, 4, 5, 6, 7, all"
                exit 1
                ;;
        esac
    else
        # Interactive mode - run all tests with prompts
        echo "Running all tests interactively. Press Enter after each test."
        echo ""
        
        test_1_basic_notification
        wait_for_user
        
        test_2_notification_with_icon
        wait_for_user
        
        test_3_notification_with_image_hint
        wait_for_user
        
        test_4_notification_with_markup
        wait_for_user
        
        test_5_notification_urgency_levels
        wait_for_user
        
        test_6_notification_with_actions
        wait_for_user
        
        test_7_notification_with_timeout
    fi
    
    print_header "Test Suite Complete"
    echo "Check your connected devices to verify notifications were forwarded."
    echo ""
    echo "Troubleshooting tips:"
    echo "  - Verify COSMIC Connect daemon is running: systemctl --user status cosmic-connect"
    echo "  - Check daemon logs: journalctl --user -u cosmic-connect -f"
    echo "  - Ensure notifications plugin is enabled in configuration"
    echo ""
}

main "$@"
