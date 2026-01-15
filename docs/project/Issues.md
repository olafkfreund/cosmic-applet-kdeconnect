# GitHub Issues for cosmic-applet-kdeconnect

This file contains all GitHub issues to create for tracking project development.

## Instructions

### Option 1: Using GitHub CLI (after authentication)
```bash
# Authenticate first
gh auth login

# Then run the script below
```

### Option 2: Create Manually
Copy each issue section and create it via the GitHub web interface at:
https://github.com/olafkfreund/cosmic-applet-kdeconnect/issues/new

---

## PHASE 1: FOUNDATION (Q1 2026)

### Issue #1: Implement Core Packet Structure

**Labels:** `protocol`, `foundation`, `good-first-issue`

**Description:**
Implement the core packet serialization/deserialization for the KDE Connect protocol.

**Details:**
The KDE Connect protocol uses JSON packets with a specific structure:
```json
{
  "id": timestamp_in_ms,
  "type": "kdeconnect.packet.type",
  "body": {}
}
```

**Tasks:**
- [ ] Define Packet struct with serde support
- [ ] Implement `to_bytes()` with newline terminator
- [ ] Implement `from_bytes()` for deserialization
- [ ] Add timestamp generation
- [ ] Create packet type constants
- [ ] Write unit tests for serialization/deserialization
- [ ] Add examples in documentation

**Reference:**
- See `kdeconnect-protocol.md` for packet structure details
- Protocol version: 7

**Files:**
- `kdeconnect-protocol/src/packet.rs`

---

### Issue #2: Implement UDP Device Discovery

**Labels:** `protocol`, `foundation`, `networking`

**Description:**
Implement UDP broadcast-based device discovery mechanism to find KDE Connect devices on the local network.

**Details:**
Device discovery uses UDP broadcast on ports 1714-1764 to exchange identity packets.

**Tasks:**
- [ ] Implement UDP socket binding and broadcast
- [ ] Create `broadcast_identity()` function
- [ ] Create `listen_for_devices()` function
- [ ] Parse incoming identity packets
- [ ] Extract device information (ID, name, type, capabilities)
- [ ] Implement channel-based device notification
- [ ] Add timeout and retry logic
- [ ] Write integration tests with mock devices
- [ ] Handle network errors gracefully

**Requirements:**
- Broadcast identity packet on all ports 1714-1764
- Listen on port 1716 for responses
- Use tokio for async operations
- Return device info via mpsc channel

**Reference:**
- See `kdeconnect-protocol.md` for discovery details
- Check firewall requirements in README

**Files:**
- `kdeconnect-protocol/src/discovery.rs`

---

### Issue #3: Implement TLS Pairing and Certificate Management

**Labels:** `protocol`, `foundation`, `security`

**Description:**
Implement secure TLS-based device pairing with certificate generation and validation.

**Details:**
KDE Connect requires TLS 1.3 for all device communication after discovery. Devices exchange certificates during pairing.

**Tasks:**
- [ ] Generate self-signed certificates using rcgen
- [ ] Implement certificate storage (filesystem)
- [ ] Create TLS connection setup (client and server)
- [ ] Implement certificate fingerprint validation
- [ ] Handle pairing request packets
- [ ] Handle pairing response packets
- [ ] Store paired device certificates
- [ ] Implement certificate pinning
- [ ] Add unpair functionality
- [ ] Write tests with mock certificates

**Security Considerations:**
- Use TLS 1.3 only
- Verify certificate fingerprints
- Secure key storage
- No plaintext communication

**Reference:**
- See `kdeconnect-protocol.md` for pairing flow
- Review rustls documentation

**Files:**
- `kdeconnect-protocol/src/pairing.rs`

---

### Issue #4: Implement Device State Management

**Labels:** `protocol`, `foundation`

**Description:**
Implement device state tracking and management for connected devices.

**Details:**
Track device information, connection status, capabilities, and manage device lifecycle.

**Tasks:**
- [ ] Define Device struct with all properties
- [ ] Track device ID, name, type
- [ ] Store device capabilities (incoming/outgoing)
- [ ] Manage connection state (discovered, paired, connected)
- [ ] Implement device persistence (save/load)
- [ ] Create DeviceManager for multiple devices
- [ ] Add event notifications (connected, disconnected)
- [ ] Implement device timeout/keepalive
- [ ] Write comprehensive unit tests

**Device Properties:**
- deviceId (unique identifier)
- deviceName
- deviceType (desktop, laptop, phone, tablet, tv)
- protocolVersion
- incomingCapabilities
- outgoingCapabilities
- certificate (when paired)
- connection status

**Files:**
- `kdeconnect-protocol/src/device.rs`

---

### Issue #5: Define Plugin Trait and Architecture

**Labels:** `protocol`, `architecture`

**Description:**
Define the plugin trait and architecture for extensible protocol features.

**Details:**
Plugins handle specific functionality like battery, clipboard, notifications, etc.

**Tasks:**
- [ ] Define async Plugin trait
- [ ] Add plugin lifecycle methods (init, shutdown)
- [ ] Implement `handle_packet()` method
- [ ] Add capability discovery
- [ ] Create plugin registry/manager
- [ ] Implement plugin loading
- [ ] Add plugin error handling
- [ ] Document plugin development process
- [ ] Create example plugin template

**Plugin Trait Methods:**
```rust
#[async_trait]
pub trait Plugin: Send + Sync {
    fn id(&self) -> &str;
    fn incoming_capabilities(&self) -> Vec<String>;
    fn outgoing_capabilities(&self) -> Vec<String>;
    async fn handle_packet(&mut self, packet: Packet) -> Result<()>;
    async fn init(&mut self) -> Result<()>;
    async fn shutdown(&mut self) -> Result<()>;
}
```

**Files:**
- `kdeconnect-protocol/src/plugins/mod.rs`

---

### Issue #6: Implement Error Handling and Logging

**Labels:** `infrastructure`, `quality`

**Description:**
Implement comprehensive error handling and structured logging throughout the codebase.

**Tasks:**
- [ ] Define error types using thiserror
- [ ] Create ProtocolError enum
- [ ] Add context to errors
- [ ] Implement error propagation
- [ ] Setup tracing/logging infrastructure
- [ ] Add debug, info, warn, error logs
- [ ] Log protocol events
- [ ] Log connection lifecycle
- [ ] Add performance tracing
- [ ] Document error handling patterns

**Error Types:**
- ConnectionError
- PairingError
- PluginError
- SerializationError
- CertificateError
- TimeoutError

**Files:**
- `kdeconnect-protocol/src/error.rs`

---

## PHASE 2: PLUGINS & FEATURES (Q2 2026)

### Issue #7: Implement Ping Plugin

**Labels:** `plugin`, `feature`, `good-first-issue`

**Description:**
Implement simple ping/pong functionality for connectivity testing.

**Details:**
The ping plugin allows devices to test connectivity and responsiveness.

**Tasks:**
- [ ] Implement PingPlugin struct
- [ ] Handle `kdeconnect.ping` packets
- [ ] Send ping response
- [ ] Add optional message support
- [ ] Emit notification on ping received
- [ ] Measure round-trip time
- [ ] Write unit tests
- [ ] Test bidirectional ping

**Packet Format:**
```json
{
  "type": "kdeconnect.ping",
  "body": {
    "message": "optional message"
  }
}
```

**Dependencies:**
- Depends on: Plugin trait (#5)

**Files:**
- `kdeconnect-protocol/src/plugins/ping.rs`

---

### Issue #8: Implement Battery Plugin

**Labels:** `plugin`, `feature`

**Description:**
Implement battery status reporting plugin.

**Details:**
The battery plugin reports device battery level and charging status.

**Tasks:**
- [ ] Implement BatteryPlugin struct
- [ ] Handle `kdeconnect.battery.request` packets
- [ ] Send `kdeconnect.battery` response packets
- [ ] Read battery status from system (via sysfs/upower)
- [ ] Implement periodic status updates
- [ ] Add threshold event detection
- [ ] Handle charging state changes
- [ ] Write unit tests with mocked battery data
- [ ] Test with real Android device

**Packet Format:**
```json
{
  "type": "kdeconnect.battery",
  "body": {
    "currentCharge": 85,
    "isCharging": true,
    "thresholdEvent": 0
  }
}
```

**Dependencies:**
- Depends on: Plugin trait (#5)

**Files:**
- `kdeconnect-protocol/src/plugins/battery.rs`

---

### Issue #9: Implement Notification Sync Plugin

**Labels:** `plugin`, `feature`, `notifications`

**Description:**
Implement notification mirroring from mobile devices to desktop.

**Details:**
Mirror Android/iOS notifications to COSMIC desktop notifications.

**Tasks:**
- [ ] Implement NotificationPlugin struct
- [ ] Handle `kdeconnect.notification` packets
- [ ] Parse notification data (id, title, text, app)
- [ ] Integrate with freedesktop notifications (zbus)
- [ ] Show desktop notifications
- [ ] Handle notification dismissal
- [ ] Support notification actions
- [ ] Add notification history
- [ ] Filter notifications by app
- [ ] Write integration tests with notification daemon

**Packet Format:**
```json
{
  "type": "kdeconnect.notification",
  "body": {
    "id": "notification_id",
    "appName": "App Name",
    "title": "Notification Title",
    "text": "Notification body",
    "isClearable": true,
    "time": "1736784000000"
  }
}
```

**Dependencies:**
- Depends on: Plugin trait (#5)
- Requires: zbus integration

**Files:**
- `kdeconnect-protocol/src/plugins/notification.rs`

---

### Issue #10: Implement Share/File Transfer Plugin

**Labels:** `plugin`, `feature`, `file-transfer`

**Description:**
Implement file and URL sharing between devices.

**Details:**
The share plugin handles sending/receiving files and URLs.

**Tasks:**
- [ ] Implement SharePlugin struct
- [ ] Handle `kdeconnect.share.request` packets
- [ ] Parse file metadata (filename, size)
- [ ] Implement payload transfer (separate TCP connection)
- [ ] Handle incoming file transfers
- [ ] Show transfer progress
- [ ] Save files to downloads directory
- [ ] Handle URL sharing
- [ ] Add transfer cancellation
- [ ] Implement file picker integration
- [ ] Write integration tests

**Packet Format:**
```json
{
  "type": "kdeconnect.share.request",
  "body": {
    "filename": "document.pdf",
    "text": "optional text",
    "url": "optional URL"
  },
  "payloadSize": 1024000,
  "payloadTransferInfo": {
    "port": 1739
  }
}
```

**Dependencies:**
- Depends on: Plugin trait (#5)

**Files:**
- `kdeconnect-protocol/src/plugins/share.rs`

---

### Issue #11: Implement Clipboard Sync Plugin

**Labels:** `plugin`, `feature`

**Description:**
Implement bidirectional clipboard synchronization between devices.

**Details:**
Keep clipboard content synchronized across devices.

**Tasks:**
- [ ] Implement ClipboardPlugin struct
- [ ] Monitor local clipboard changes
- [ ] Send `kdeconnect.clipboard` packets on change
- [ ] Receive and update local clipboard
- [ ] Handle clipboard connect/disconnect
- [ ] Support text content
- [ ] Add clipboard history (optional)
- [ ] Prevent sync loops
- [ ] Implement debouncing
- [ ] Test with various clipboard managers

**Packet Format:**
```json
{
  "type": "kdeconnect.clipboard",
  "body": {
    "content": "clipboard text"
  }
}
```

**Dependencies:**
- Depends on: Plugin trait (#5)
- Requires: clipboard access library

**Files:**
- `kdeconnect-protocol/src/plugins/clipboard.rs`

---

## PHASE 3: APPLET & UI (Q2 2026)

### Issue #12: Enhance Applet UI with Device List

**Labels:** `applet`, `ui`, `enhancement`

**Description:**
Enhance the cosmic applet to display connected devices and their status.

**Current State:**
The applet shows only a phone icon button.

**Tasks:**
- [ ] Add popup window for device list
- [ ] Display device name and status
- [ ] Show connection indicator (connected/disconnected)
- [ ] Add device battery level display
- [ ] Implement device selection
- [ ] Add "Pair Device" action
- [ ] Add "Unpair Device" action
- [ ] Show loading/discovery state
- [ ] Handle empty state (no devices)
- [ ] Follow COSMIC design patterns

**UI Components Needed:**
- Scrollable device list
- Device item with icon, name, status
- Battery indicator
- Action buttons (pair, send file, etc.)
- Settings button

**Reference:**
- See `cosmic-applet-dev.md` for UI patterns

**Files:**
- `cosmic-applet-kdeconnect/src/main.rs`

---

### Issue #13: Implement Background Daemon Service

**Labels:** `daemon`, `architecture`

**Description:**
Implement the background daemon to maintain device connections.

**Details:**
The daemon runs in the background, manages connections, and handles plugin operations.

**Tasks:**
- [ ] Implement daemon main loop
- [ ] Initialize protocol library
- [ ] Start device discovery
- [ ] Manage device connections
- [ ] Load and initialize plugins
- [ ] Handle graceful shutdown
- [ ] Add systemd service file
- [ ] Implement DBus interface for control
- [ ] Add logging configuration
- [ ] Handle process signals (SIGTERM, SIGHUP)
- [ ] Write integration tests

**Architecture:**
- Single daemon process
- Async event loop (tokio)
- Plugin system
- DBus interface for apps
- Persistent device storage

**Files:**
- `kdeconnect-daemon/src/main.rs`

---

## PHASE 4: ADVANCED FEATURES (Q3 2026)

### Issue #14: Implement MPRIS Media Control Plugin

**Labels:** `plugin`, `feature`, `media`

**Description:**
Implement media player control via MPRIS2 integration.

**Details:**
Control desktop media players from mobile devices using MPRIS2 DBus interface.

**Tasks:**
- [ ] Implement MprisPlugin struct
- [ ] Discover MPRIS2-compatible media players
- [ ] Handle `kdeconnect.mpris.request` packets
- [ ] Send `kdeconnect.mpris` status updates
- [ ] Implement play/pause/stop controls
- [ ] Implement next/previous track
- [ ] Support volume control
- [ ] Support seek operations
- [ ] Send now-playing metadata
- [ ] Handle multiple players
- [ ] Test with various media players (VLC, Firefox, etc.)

**Packet Format:**
```json
{
  "type": "kdeconnect.mpris",
  "body": {
    "player": "player_name",
    "isPlaying": true,
    "title": "Song Title",
    "artist": "Artist Name",
    "album": "Album Name",
    "length": 240000,
    "pos": 60000,
    "volume": 75
  }
}
```

**Dependencies:**
- Depends on: Plugin trait (#5)
- Requires: zbus with MPRIS2 support

**Files:**
- `kdeconnect-protocol/src/plugins/mpris.rs`

---

## INFRASTRUCTURE & QUALITY

### Issue #15: Setup CI/CD Pipeline

**Labels:** `infrastructure`, `ci-cd`

**Description:**
Set up automated testing and builds using GitHub Actions.

**Tasks:**
- [ ] Create GitHub Actions workflow
- [ ] Run cargo fmt check
- [ ] Run cargo clippy
- [ ] Run cargo test
- [ ] Test on multiple Rust versions (stable, beta)
- [ ] Build release binaries
- [ ] Run cargo audit for security
- [ ] Generate and publish documentation
- [ ] Add status badges to README
- [ ] Setup dependabot for dependency updates

**Workflow Triggers:**
- Push to main branch
- Pull requests
- Scheduled (weekly security audit)

**Files:**
- `.github/workflows/ci.yml`
- `.github/workflows/release.yml`
- `.github/dependabot.yml`

---

### Issue #16: Add Integration Tests

**Labels:** `testing`, `quality`

**Description:**
Create comprehensive integration tests for the protocol implementation.

**Details:**
Test real-world scenarios with multiple devices and plugins.

**Tasks:**
- [ ] Setup integration test framework
- [ ] Create mock device implementation
- [ ] Test device discovery flow
- [ ] Test pairing flow end-to-end
- [ ] Test packet exchange
- [ ] Test plugin communication
- [ ] Test error handling and recovery
- [ ] Test network disconnection scenarios
- [ ] Test certificate validation
- [ ] Add performance benchmarks

**Test Scenarios:**
- Device A discovers device B
- Devices pair successfully
- Pairing rejection handling
- Plugin packet exchange
- Connection timeout
- Network disruption recovery
- Multiple simultaneous devices

**Files:**
- `kdeconnect-protocol/tests/integration_tests.rs`

---

### Issue #17: Create User Documentation

**Labels:** `documentation`

**Description:**
Create comprehensive user documentation for installation and usage.

**Tasks:**
- [ ] Write installation guide for NixOS
- [ ] Write installation guide for other distributions
- [ ] Create user manual with screenshots
- [ ] Document pairing process
- [ ] Document each plugin feature
- [ ] Create troubleshooting guide
- [ ] Write FAQ
- [ ] Add architecture diagrams
- [ ] Document firewall requirements
- [ ] Create video tutorials (optional)

**Documentation Structure:**
```
docs/
├── installation/
│   ├── nixos.md
│   ├── ubuntu.md
│   └── arch.md
├── user-guide/
│   ├── getting-started.md
│   ├── pairing.md
│   ├── features.md
│   └── settings.md
├── troubleshooting.md
└── faq.md
```

---

### Issue #18: Create NixOS Package

**Labels:** `nixos`, `packaging`

**Description:**
Create a proper NixOS package for cosmic-applet-kdeconnect.

**Tasks:**
- [ ] Write package derivation
- [ ] Define dependencies
- [ ] Add systemd service configuration
- [ ] Add firewall rules to module
- [ ] Create NixOS module with options
- [ ] Write package tests
- [ ] Submit to nixpkgs (when stable)
- [ ] Add to COSMIC module
- [ ] Document NixOS-specific configuration

**NixOS Module Options:**
```nix
services.cosmic-kdeconnect = {
  enable = mkEnableOption "KDE Connect";
  openFirewall = mkOption { default = true; };
  package = mkPackageOption pkgs "cosmic-applet-kdeconnect" {};
};
```

**Files:**
- `nix/package.nix`
- `nix/module.nix`

---

## MILESTONES

### Milestone: Phase 1 - Foundation Complete

**Target:** Q1 2026

**Required Issues:**
- #1 - Core Packet Structure
- #2 - UDP Device Discovery
- #3 - TLS Pairing
- #4 - Device State Management
- #5 - Plugin Architecture
- #6 - Error Handling

**Success Criteria:**
- ✅ Can discover devices on local network
- ✅ Can pair with Android/iOS KDE Connect app
- ✅ Can send/receive basic packets
- ✅ Code is well-documented
- ✅ Tests have good coverage (>70%)

---

### Milestone: Phase 2 - Basic Functionality

**Target:** Q2 2026

**Required Issues:**
- #7 - Ping Plugin
- #8 - Battery Plugin
- #9 - Notification Sync
- #10 - File Sharing
- #12 - Enhanced Applet UI
- #13 - Background Daemon

**Success Criteria:**
- ✅ Applet shows device list in panel
- ✅ Can send/receive files
- ✅ Notifications appear on desktop
- ✅ Battery status visible
- ✅ Daemon runs in background

---

### Milestone: Phase 3 - Advanced Features

**Target:** Q3 2026

**Required Issues:**
- #11 - Clipboard Sync
- #14 - MPRIS Media Control

**Success Criteria:**
- ✅ Clipboard syncs automatically
- ✅ Can control media players from phone
- ✅ All core features stable

---

### Milestone: Phase 4 - Polish & Release

**Target:** Q4 2026

**Required Issues:**
- #15 - CI/CD Pipeline
- #16 - Integration Tests
- #17 - User Documentation
- #18 - NixOS Package

**Success Criteria:**
- ✅ Automated builds and tests
- ✅ Comprehensive documentation
- ✅ Package available in nixpkgs
- ✅ Ready for public release v1.0

---

## Script to Create All Issues

Save this as `create-issues.sh`:

```bash
#!/bin/bash

# Authenticate first if needed
# gh auth login

# Phase 1
gh issue create --title "Implement Core Packet Structure" --label "protocol,foundation,good-first-issue" --body-file issue-01.md
gh issue create --title "Implement UDP Device Discovery" --label "protocol,foundation,networking" --body-file issue-02.md
gh issue create --title "Implement TLS Pairing and Certificate Management" --label "protocol,foundation,security" --body-file issue-03.md
gh issue create --title "Implement Device State Management" --label "protocol,foundation" --body-file issue-04.md
gh issue create --title "Define Plugin Trait and Architecture" --label "protocol,architecture" --body-file issue-05.md
gh issue create --title "Implement Error Handling and Logging" --label "infrastructure,quality" --body-file issue-06.md

# Phase 2
gh issue create --title "Implement Ping Plugin" --label "plugin,feature,good-first-issue" --body-file issue-07.md
gh issue create --title "Implement Battery Plugin" --label "plugin,feature" --body-file issue-08.md
gh issue create --title "Implement Notification Sync Plugin" --label "plugin,feature,notifications" --body-file issue-09.md
gh issue create --title "Implement Share/File Transfer Plugin" --label "plugin,feature,file-transfer" --body-file issue-10.md
gh issue create --title "Implement Clipboard Sync Plugin" --label "plugin,feature" --body-file issue-11.md

# Phase 3
gh issue create --title "Enhance Applet UI with Device List" --label "applet,ui,enhancement" --body-file issue-12.md
gh issue create --title "Implement Background Daemon Service" --label "daemon,architecture" --body-file issue-13.md

# Phase 4
gh issue create --title "Implement MPRIS Media Control Plugin" --label "plugin,feature,media" --body-file issue-14.md

# Infrastructure
gh issue create --title "Setup CI/CD Pipeline" --label "infrastructure,ci-cd" --body-file issue-15.md
gh issue create --title "Add Integration Tests" --label "testing,quality" --body-file issue-16.md
gh issue create --title "Create User Documentation" --label "documentation" --body-file issue-17.md
gh issue create --title "Create NixOS Package" --label "nixos,packaging" --body-file issue-18.md

echo "All issues created successfully!"
```
