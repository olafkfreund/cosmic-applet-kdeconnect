#!/bin/bash
# Script to create all GitHub issues for cosmic-applet-kdeconnect
# Run after: unset GITHUB_TOKEN && gh auth login

set -e

echo "Creating Phase 1 issues..."

gh issue create --title "Implement Core Packet Structure" \
  --label "protocol,foundation,good-first-issue" \
  --body "Implement the core packet serialization/deserialization for the KDE Connect protocol.

Tasks:
- Define Packet struct with serde support
- Implement to_bytes() with newline terminator
- Implement from_bytes() for deserialization
- Add timestamp generation
- Write unit tests

Files: kdeconnect-protocol/src/packet.rs"

gh issue create --title "Implement UDP Device Discovery" \
  --label "protocol,foundation,networking" \
  --body "Implement UDP broadcast-based device discovery mechanism.

Tasks:
- Implement UDP socket binding and broadcast
- Create broadcast_identity() function
- Create listen_for_devices() function
- Parse incoming identity packets
- Write integration tests

Files: kdeconnect-protocol/src/discovery.rs"

gh issue create --title "Implement TLS Pairing and Certificate Management" \
  --label "protocol,foundation,security" \
  --body "Implement secure TLS-based device pairing with certificate generation and validation.

Tasks:
- Generate self-signed certificates using rcgen
- Implement certificate storage
- Create TLS connection setup
- Handle pairing request/response packets
- Write tests with mock certificates

Files: kdeconnect-protocol/src/pairing.rs"

gh issue create --title "Implement Device State Management" \
  --label "protocol,foundation" \
  --body "Implement device state tracking and management for connected devices.

Tasks:
- Define Device struct with all properties
- Track device ID, name, type, capabilities
- Manage connection state
- Implement device persistence
- Create DeviceManager for multiple devices

Files: kdeconnect-protocol/src/device.rs"

gh issue create --title "Define Plugin Trait and Architecture" \
  --label "protocol,architecture" \
  --body "Define the plugin trait and architecture for extensible protocol features.

Tasks:
- Define async Plugin trait
- Add plugin lifecycle methods
- Implement handle_packet() method
- Create plugin registry/manager
- Document plugin development

Files: kdeconnect-protocol/src/plugins/mod.rs"

gh issue create --title "Implement Error Handling and Logging" \
  --label "infrastructure,quality" \
  --body "Implement comprehensive error handling and structured logging.

Tasks:
- Define error types using thiserror
- Create ProtocolError enum
- Setup tracing/logging infrastructure
- Add debug, info, warn, error logs
- Document error handling patterns

Files: kdeconnect-protocol/src/error.rs"

echo "Creating Phase 2 issues..."

gh issue create --title "Implement Ping Plugin" \
  --label "plugin,feature,good-first-issue" \
  --body "Implement simple ping/pong functionality for connectivity testing.

Tasks:
- Implement PingPlugin struct
- Handle kdeconnect.ping packets
- Send ping response
- Measure round-trip time
- Write unit tests

Depends on: Plugin trait
Files: kdeconnect-protocol/src/plugins/ping.rs"

gh issue create --title "Implement Battery Plugin" \
  --label "plugin,feature" \
  --body "Implement battery status reporting plugin.

Tasks:
- Implement BatteryPlugin struct
- Handle battery request packets
- Read battery status from system
- Implement periodic updates
- Write unit tests

Depends on: Plugin trait
Files: kdeconnect-protocol/src/plugins/battery.rs"

gh issue create --title "Implement Notification Sync Plugin" \
  --label "plugin,feature,notifications" \
  --body "Implement notification mirroring from mobile devices to desktop.

Tasks:
- Implement NotificationPlugin struct
- Parse notification data
- Integrate with freedesktop notifications (zbus)
- Show desktop notifications
- Write integration tests

Depends on: Plugin trait
Files: kdeconnect-protocol/src/plugins/notification.rs"

gh issue create --title "Implement Share/File Transfer Plugin" \
  --label "plugin,feature" \
  --body "Implement file and URL sharing between devices.

Tasks:
- Implement SharePlugin struct
- Handle share request packets
- Implement payload transfer
- Show transfer progress
- Add file picker integration

Depends on: Plugin trait
Files: kdeconnect-protocol/src/plugins/share.rs"

gh issue create --title "Implement Clipboard Sync Plugin" \
  --label "plugin,feature" \
  --body "Implement bidirectional clipboard synchronization.

Tasks:
- Implement ClipboardPlugin struct
- Monitor local clipboard changes
- Send/receive clipboard packets
- Prevent sync loops
- Test with clipboard managers

Depends on: Plugin trait
Files: kdeconnect-protocol/src/plugins/clipboard.rs"

echo "Creating UI issues..."

gh issue create --title "Enhance Applet UI with Device List" \
  --label "applet,ui,enhancement" \
  --body "Enhance the cosmic applet to display connected devices and their status.

Tasks:
- Add popup window for device list
- Display device name and status
- Show battery level
- Add pair/unpair actions
- Follow COSMIC design patterns

Files: cosmic-applet-kdeconnect/src/main.rs"

gh issue create --title "Implement Background Daemon Service" \
  --label "daemon,architecture" \
  --body "Implement the background daemon to maintain device connections.

Tasks:
- Implement daemon main loop
- Initialize protocol library
- Start device discovery
- Manage device connections
- Load and initialize plugins
- Add systemd service file

Files: kdeconnect-daemon/src/main.rs"

echo "Creating advanced feature issues..."

gh issue create --title "Implement MPRIS Media Control Plugin" \
  --label "plugin,feature,media" \
  --body "Implement media player control via MPRIS2 integration.

Tasks:
- Implement MprisPlugin struct
- Discover MPRIS2 media players
- Handle mpris request packets
- Implement play/pause/stop controls
- Send now-playing metadata

Depends on: Plugin trait
Files: kdeconnect-protocol/src/plugins/mpris.rs"

echo "Creating infrastructure issues..."

gh issue create --title "Setup CI/CD Pipeline" \
  --label "infrastructure,ci-cd" \
  --body "Set up automated testing and builds using GitHub Actions.

Tasks:
- Create GitHub Actions workflow
- Run cargo fmt, clippy, test
- Build release binaries
- Run cargo audit
- Setup dependabot

Files: .github/workflows/ci.yml"

gh issue create --title "Add Integration Tests" \
  --label "testing,quality" \
  --body "Create comprehensive integration tests for the protocol implementation.

Tasks:
- Setup integration test framework
- Create mock device implementation
- Test device discovery flow
- Test pairing flow end-to-end
- Test plugin communication

Files: kdeconnect-protocol/tests/integration_tests.rs"

gh issue create --title "Create User Documentation" \
  --label "documentation" \
  --body "Create comprehensive user documentation for installation and usage.

Tasks:
- Write installation guide for NixOS
- Write installation guide for other distros
- Create user manual with screenshots
- Document pairing process
- Create troubleshooting guide"

gh issue create --title "Create NixOS Package" \
  --label "nixos,packaging" \
  --body "Create a proper NixOS package for cosmic-applet-kdeconnect.

Tasks:
- Write package derivation
- Add systemd service configuration
- Create NixOS module with options
- Write package tests
- Submit to nixpkgs

Files: nix/package.nix, nix/module.nix"

echo ""
echo "✅ All 18 issues created successfully!"
echo ""
echo "View issues at: https://github.com/olafkfreund/cosmic-applet-kdeconnect/issues"
