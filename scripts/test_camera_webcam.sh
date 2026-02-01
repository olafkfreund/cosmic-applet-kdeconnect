#!/usr/bin/env bash
# test_camera_webcam.sh - Camera as Webcam testing script for COSMIC Connect
#
# This script helps test the Camera as Webcam feature (Issue #132) by:
# - Checking v4l2loopback module availability
# - Setting up virtual video devices
# - Providing testing instructions for Android integration
# - Testing video stream with various applications
#
# Usage: ./scripts/test_camera_webcam.sh [options]
#        ./scripts/test_camera_webcam.sh --setup
#        ./scripts/test_camera_webcam.sh --test
#        ./scripts/test_camera_webcam.sh --cleanup

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
NC='\033[0m' # No Color

# Default virtual video device
V4L2_DEVICE="${V4L2_DEVICE:-/dev/video10}"
V4L2_CARD_LABEL="COSMIC Connect Webcam"

# Test mode flags
SETUP_MODE=false
TEST_MODE=false
CLEANUP_MODE=false
INTERACTIVE_MODE=false

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -s|--setup)
            SETUP_MODE=true
            shift
            ;;
        -t|--test)
            TEST_MODE=true
            shift
            ;;
        -c|--cleanup)
            CLEANUP_MODE=true
            shift
            ;;
        -i|--interactive)
            INTERACTIVE_MODE=true
            shift
            ;;
        -d|--device)
            V4L2_DEVICE="$2"
            shift 2
            ;;
        -h|--help)
            cat << EOF
COSMIC Connect Camera as Webcam Testing Script

Usage: $0 [options]

Options:
  -s, --setup          Set up v4l2loopback module and virtual device
  -t, --test           Run camera webcam tests
  -c, --cleanup        Remove v4l2loopback module
  -i, --interactive    Run in interactive mode
  -d, --device PATH    Specify video device (default: /dev/video10)
  -h, --help           Show this help message

Examples:
  $0 --setup                      # Set up v4l2loopback
  $0 --test                       # Test camera streaming
  $0 --interactive                # Interactive testing menu
  $0 --setup --device /dev/video5 # Use custom device

Environment Variables:
  V4L2_DEVICE          Override default video device path

Requirements:
  - v4l2loopback kernel module
  - v4l2-ctl (v4l-utils package)
  - ffmpeg or gstreamer (for testing)
  - Android device with COSMIC Connect app

For more information, see:
  - docs/CAMERA_WEBCAM.md
  - Issue #132: Camera as Webcam plugin
EOF
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Banner
echo ""
echo "╔════════════════════════════════════════════════════════╗"
echo "║     COSMIC Connect Camera as Webcam Test Suite        ║"
echo "║     Testing Camera Plugin with V4L2 Loopback          ║"
echo "╚════════════════════════════════════════════════════════╝"
echo ""

# Function to check if running as root
check_root() {
    if [[ $EUID -eq 0 ]]; then
        echo -e "${YELLOW}⚠ Running as root is not recommended${NC}"
        echo -e "  Consider using sudo only for specific commands"
        echo ""
    fi
}

# Function to check if v4l2loopback module is loaded
check_v4l2loopback() {
    echo -e "${CYAN}📹 Checking v4l2loopback module...${NC}"

    if lsmod | grep -q v4l2loopback; then
        echo -e "   ${GREEN}✓ v4l2loopback module is loaded${NC}"
        return 0
    else
        echo -e "   ${YELLOW}⚠ v4l2loopback module is not loaded${NC}"
        return 1
    fi
}

# Function to check if v4l2loopback is available in the system
check_v4l2loopback_available() {
    echo -e "${CYAN}🔍 Checking v4l2loopback availability...${NC}"

    if modinfo v4l2loopback &> /dev/null; then
        echo -e "   ${GREEN}✓ v4l2loopback module is available${NC}"
        modinfo v4l2loopback | grep -E "^(filename|version|description):" | sed 's/^/   /'
        return 0
    else
        echo -e "   ${RED}✗ v4l2loopback module not found${NC}"
        echo -e "   ${YELLOW}→ On NixOS, ensure v4l2loopback is in your configuration${NC}"
        echo -e "   ${YELLOW}→ Add to configuration.nix:${NC}"
        echo -e "   ${BLUE}     boot.extraModulePackages = [ config.boot.kernelPackages.v4l2loopback ];${NC}"
        echo -e "   ${BLUE}     boot.kernelModules = [ \"v4l2loopback\" ];${NC}"
        return 1
    fi
}

# Function to set up v4l2loopback
setup_v4l2loopback() {
    echo -e "${CYAN}⚙️  Setting up v4l2loopback...${NC}"

    # Check if already loaded
    if check_v4l2loopback; then
        echo -e "   ${YELLOW}ℹ Module already loaded, reloading with correct parameters...${NC}"
        sudo modprobe -r v4l2loopback || true
    fi

    # Extract device number from path (e.g., /dev/video10 -> 10)
    DEVICE_NUM="${V4L2_DEVICE##*/video}"

    echo -e "   ${BLUE}→ Loading v4l2loopback module...${NC}"
    echo -e "     Device: ${V4L2_DEVICE}"
    echo -e "     Label: ${V4L2_CARD_LABEL}"

    if sudo modprobe v4l2loopback \
        video_nr="${DEVICE_NUM}" \
        card_label="${V4L2_CARD_LABEL}" \
        exclusive_caps=1 \
        max_buffers=2; then
        echo -e "   ${GREEN}✓ v4l2loopback module loaded successfully${NC}"
        sleep 1

        # Verify device exists
        if [[ -e "${V4L2_DEVICE}" ]]; then
            echo -e "   ${GREEN}✓ Virtual camera device created: ${V4L2_DEVICE}${NC}"
            return 0
        else
            echo -e "   ${RED}✗ Device ${V4L2_DEVICE} was not created${NC}"
            return 1
        fi
    else
        echo -e "   ${RED}✗ Failed to load v4l2loopback module${NC}"
        return 1
    fi
}

# Function to list all video devices
list_video_devices() {
    echo -e "${CYAN}📋 Video Devices:${NC}"
    echo ""

    if command -v v4l2-ctl &> /dev/null; then
        v4l2-ctl --list-devices | while IFS= read -r line; do
            if [[ -z "$line" ]]; then
                echo ""
            elif [[ "$line" =~ ^[[:space:]] ]]; then
                # Device path (indented)
                device_path=$(echo "$line" | xargs)
                if [[ "$device_path" == "$V4L2_DEVICE" ]]; then
                    echo -e "   ${GREEN}→ ${device_path} (COSMIC Connect)${NC}"
                else
                    echo "   → ${device_path}"
                fi
            else
                # Device name
                echo -e "${BLUE}${line}${NC}"
            fi
        done
        echo ""
    else
        echo -e "${YELLOW}⚠ v4l2-ctl not found${NC}"
        echo -e "  Install v4l-utils package to list devices"
        echo ""
        echo "Available devices:"
        ls -la /dev/video* 2>/dev/null || echo "No video devices found"
        echo ""
    fi
}

# Function to show device information
show_device_info() {
    local device="${1:-$V4L2_DEVICE}"

    echo -e "${CYAN}ℹ️  Device Information: ${device}${NC}"
    echo ""

    if [[ ! -e "$device" ]]; then
        echo -e "${RED}✗ Device does not exist: ${device}${NC}"
        return 1
    fi

    if command -v v4l2-ctl &> /dev/null; then
        echo -e "${BLUE}Capabilities:${NC}"
        v4l2-ctl -d "$device" --all | grep -A 20 "Driver Info" | sed 's/^/  /'
        echo ""

        echo -e "${BLUE}Supported Formats:${NC}"
        v4l2-ctl -d "$device" --list-formats-ext | head -30 | sed 's/^/  /'
        echo ""
    else
        echo -e "${YELLOW}⚠ v4l2-ctl not found, cannot show detailed info${NC}"
        ls -l "$device"
        echo ""
    fi
}

# Function to test video stream with ffmpeg
test_with_ffmpeg() {
    echo -e "${CYAN}🎥 Testing with ffmpeg...${NC}"

    if ! command -v ffmpeg &> /dev/null; then
        echo -e "${YELLOW}⚠ ffmpeg not installed, skipping test${NC}"
        return 1
    fi

    echo -e "   ${BLUE}→ Attempting to read from ${V4L2_DEVICE}...${NC}"
    echo -e "   ${YELLOW}ℹ This will fail if no stream is active - that's expected${NC}"
    echo ""

    # Try to read 5 frames with a timeout
    timeout 5 ffmpeg -f v4l2 -i "$V4L2_DEVICE" -frames:v 5 -f null - 2>&1 | \
        grep -E "(Stream|Video:|fps,|error)" || true

    echo ""
    echo -e "   ${GREEN}✓ ffmpeg test complete${NC}"
    echo -e "   ${YELLOW}→ If you see 'No such device' or errors, start streaming from Android${NC}"
}

# Function to test video stream with gstreamer
test_with_gstreamer() {
    echo -e "${CYAN}🎬 Testing with GStreamer...${NC}"

    if ! command -v gst-launch-1.0 &> /dev/null; then
        echo -e "${YELLOW}⚠ GStreamer not installed, skipping test${NC}"
        return 1
    fi

    echo -e "   ${BLUE}→ Attempting to display video from ${V4L2_DEVICE}...${NC}"
    echo -e "   ${YELLOW}ℹ Press Ctrl+C to stop${NC}"
    echo ""

    # Try to display video for 10 seconds
    timeout 10 gst-launch-1.0 v4l2src device="$V4L2_DEVICE" ! \
        videoconvert ! autovideosink 2>&1 | \
        grep -E "(Setting pipeline|caps)" || true

    echo ""
    echo -e "   ${GREEN}✓ GStreamer test complete${NC}"
}

# Function to test with applications
test_with_applications() {
    echo -e "${CYAN}🖥️  Testing with Applications${NC}"
    echo ""

    echo "Suggested applications to test camera stream:"
    echo ""
    echo -e "${BLUE}1. VLC Media Player${NC}"
    echo "   vlc v4l2://${V4L2_DEVICE}"
    echo ""
    echo -e "${BLUE}2. MPV${NC}"
    echo "   mpv av://v4l2:${V4L2_DEVICE}"
    echo ""
    echo -e "${BLUE}3. Cheese (GNOME)${NC}"
    echo "   cheese"
    echo "   (Select 'COSMIC Connect Webcam' from device list)"
    echo ""
    echo -e "${BLUE}4. Zoom / Teams / Google Meet${NC}"
    echo "   Select 'COSMIC Connect Webcam' in video settings"
    echo ""
    echo -e "${BLUE}5. OBS Studio${NC}"
    echo "   Add Video Capture Device source"
    echo "   Device: ${V4L2_DEVICE}"
    echo ""
}

# Function to show Android setup instructions
show_android_instructions() {
    echo -e "${MAGENTA}📱 Android Device Setup Instructions${NC}"
    echo ""
    echo "1. Install COSMIC Connect Android app on your phone"
    echo ""
    echo "2. Pair your phone with COSMIC Desktop"
    echo "   → Open the app and accept the pairing request"
    echo ""
    echo "3. Enable Camera plugin in the app settings"
    echo "   → Go to Settings → Plugins → Enable 'Camera'"
    echo ""
    echo "4. Start camera streaming from the app"
    echo "   → Tap 'Start Camera' button"
    echo "   → Select camera (front/back)"
    echo "   → Choose resolution (720p recommended)"
    echo "   → Choose quality (Medium recommended)"
    echo ""
    echo "5. Verify streaming on desktop"
    echo "   → Run: ffmpeg -f v4l2 -i ${V4L2_DEVICE} -frames:v 5 -f null -"
    echo "   → Or use any video application listed above"
    echo ""
    echo -e "${YELLOW}Troubleshooting:${NC}"
    echo "  - Ensure phone and desktop are on the same network"
    echo "  - Check firewall allows ports 1716-1764"
    echo "  - Verify cosmic-connect-daemon is running"
    echo "  - Check logs: journalctl --user -u cosmic-connect-daemon -f"
    echo ""
}

# Function to cleanup v4l2loopback
cleanup_v4l2loopback() {
    echo -e "${CYAN}🧹 Cleaning up v4l2loopback...${NC}"

    if check_v4l2loopback; then
        echo -e "   ${BLUE}→ Removing v4l2loopback module...${NC}"
        if sudo modprobe -r v4l2loopback; then
            echo -e "   ${GREEN}✓ v4l2loopback module removed${NC}"
        else
            echo -e "   ${YELLOW}⚠ Failed to remove module (may be in use)${NC}"
        fi
    else
        echo -e "   ${YELLOW}ℹ Module not loaded, nothing to clean up${NC}"
    fi
}

# Function to run comprehensive tests
run_tests() {
    echo -e "${BLUE}Running Camera Webcam Tests...${NC}"
    echo ""

    # Check prerequisites
    check_v4l2loopback_available || return 1
    check_v4l2loopback || echo -e "${YELLOW}⚠ Module not loaded - run with --setup first${NC}"
    echo ""

    # List devices
    list_video_devices

    # Show device info if exists
    if [[ -e "$V4L2_DEVICE" ]]; then
        show_device_info "$V4L2_DEVICE"
    fi

    # Test with various tools
    test_with_ffmpeg
    echo ""

    # Show application testing instructions
    test_with_applications

    # Show Android instructions
    echo ""
    show_android_instructions
}

# Interactive menu
show_interactive_menu() {
    while true; do
        clear
        echo "╔════════════════════════════════════════════════════════╗"
        echo "║   COSMIC Connect Camera Webcam Testing Menu           ║"
        echo "╠════════════════════════════════════════════════════════╣"
        echo -e "║ Device: ${V4L2_DEVICE}                     ║"
        echo "╚════════════════════════════════════════════════════════╝"
        echo ""
        echo "Select an action:"
        echo ""
        echo "  1) ⚙️  Setup v4l2loopback module"
        echo "  2) 📋 List all video devices"
        echo "  3) ℹ️  Show device information"
        echo "  4) 🎥 Test with ffmpeg"
        echo "  5) 🎬 Test with GStreamer"
        echo "  6) 🖥️  Show application testing instructions"
        echo "  7) 📱 Show Android setup instructions"
        echo "  8) 🧪 Run all tests"
        echo "  9) 🧹 Cleanup (remove v4l2loopback)"
        echo "  0) 🚪 Exit"
        echo ""
        echo -n "Choice: "
        read -r choice

        echo ""
        case $choice in
            1)
                setup_v4l2loopback
                ;;
            2)
                list_video_devices
                ;;
            3)
                show_device_info
                ;;
            4)
                test_with_ffmpeg
                ;;
            5)
                test_with_gstreamer
                ;;
            6)
                test_with_applications
                ;;
            7)
                show_android_instructions
                ;;
            8)
                run_tests
                ;;
            9)
                cleanup_v4l2loopback
                ;;
            0)
                echo "Exiting..."
                exit 0
                ;;
            *)
                echo -e "${RED}Invalid choice${NC}"
                ;;
        esac

        echo ""
        echo -n "Press Enter to continue..."
        read -r
    done
}

# Main execution flow
check_root

# Handle specific modes
if [ "$SETUP_MODE" = true ]; then
    check_v4l2loopback_available || exit 1
    setup_v4l2loopback
    echo ""
    list_video_devices
    exit 0
fi

if [ "$CLEANUP_MODE" = true ]; then
    cleanup_v4l2loopback
    exit 0
fi

if [ "$INTERACTIVE_MODE" = true ]; then
    show_interactive_menu
    exit 0
fi

if [ "$TEST_MODE" = true ]; then
    run_tests
    exit 0
fi

# Default: Run basic checks and show instructions
check_v4l2loopback_available

if check_v4l2loopback; then
    echo ""
    list_video_devices
    echo ""
    echo -e "${GREEN}✓ v4l2loopback is ready${NC}"
    echo ""
    echo "Next steps:"
    echo "  1. Run with --test to perform comprehensive testing"
    echo "  2. Run with --interactive for step-by-step testing"
    echo "  3. See --help for all options"
else
    echo ""
    echo -e "${YELLOW}→ Run with --setup to configure v4l2loopback${NC}"
    echo -e "${YELLOW}→ Run with --help to see all options${NC}"
fi

echo ""
