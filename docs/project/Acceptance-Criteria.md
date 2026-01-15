# Acceptance Criteria for cosmic-applet-kdeconnect

This document defines the acceptance criteria and quality standards for all code, features, and contributions to cosmic-applet-kdeconnect.

## Table of Contents

1. [Definition of Done](#definition-of-done)
2. [Code Quality Standards](#code-quality-standards)
3. [Feature Acceptance Criteria](#feature-acceptance-criteria)
4. [Protocol Implementation Criteria](#protocol-implementation-criteria)
5. [UI/UX Acceptance Criteria](#uiux-acceptance-criteria)
6. [Security Requirements](#security-requirements)
7. [Performance Benchmarks](#performance-benchmarks)
8. [Documentation Standards](#documentation-standards)
9. [Testing Requirements](#testing-requirements)
10. [Pull Request Checklist](#pull-request-checklist)

---

## Definition of Done

A feature, bug fix, or enhancement is considered **"Done"** when ALL of the following criteria are met:

### Code Completion
- [ ] Code is written and compiles without errors
- [ ] Code follows Rust best practices and idioms
- [ ] No `unwrap()` or `expect()` in production code
- [ ] All errors are properly handled with `Result` or logged
- [ ] Code passes `cargo fmt` formatting
- [ ] Code passes `cargo clippy` with zero warnings
- [ ] No compiler warnings on stable Rust

### Testing
- [ ] Unit tests written and passing (where applicable)
- [ ] Integration tests written and passing (where applicable)
- [ ] Test coverage ≥70% for new code
- [ ] Manual testing completed successfully
- [ ] Tested on COSMIC Desktop environment
- [ ] Edge cases and error scenarios tested

### Documentation
- [ ] Public APIs have rustdoc comments
- [ ] Complex logic has inline comments explaining "why"
- [ ] User-facing features documented in README or docs/
- [ ] CHANGELOG.md updated (if applicable)
- [ ] GitHub issue linked and updated

### Code Review
- [ ] Code reviewed by at least one other developer (or approved by maintainer)
- [ ] All review comments addressed
- [ ] No merge conflicts with main branch
- [ ] CI/CD pipeline passes all checks

### Integration
- [ ] Feature works with existing code
- [ ] No regressions in existing functionality
- [ ] Backward compatibility maintained (if applicable)
- [ ] Protocol compatibility verified (if protocol change)

---

## Code Quality Standards

All code must meet these standards before merging:

### Rust Code Standards

#### Error Handling
```rust
// ❌ REJECTED
let value = some_option.unwrap();
let result = operation().expect("Failed");

// ✅ ACCEPTED
let value = some_option.ok_or(Error::MissingValue)?;
match operation() {
    Ok(result) => result,
    Err(e) => {
        tracing::error!("Operation failed: {}", e);
        return Err(e);
    }
}
```

#### Async Operations
```rust
// ❌ REJECTED - Blocking operation in update
Message::LoadData => {
    let data = fetch_data(); // Blocks UI!
    self.data = data;
    Command::none()
}

// ✅ ACCEPTED - Async operation
Message::LoadData => {
    Command::perform(
        async { fetch_data().await },
        Message::DataLoaded
    )
}
```

#### Resource Management
```rust
// ✅ ACCEPTED - Proper cleanup
impl Drop for Connection {
    fn drop(&mut self) {
        if let Err(e) = self.close() {
            tracing::warn!("Failed to close connection: {}", e);
        }
    }
}
```

### libcosmic Standards

#### Widget Usage
```rust
// ❌ REJECTED - Custom widget when standard exists
fn custom_button() -> Element<Message> {
    container(text("Click me"))
        .on_press(Message::Clicked)
}

// ✅ ACCEPTED - Use COSMIC widgets
fn view_button() -> Element<Message> {
    button::standard("Click me")
        .on_press(Message::Clicked)
}
```

#### Theming
```rust
// ❌ REJECTED - Hardcoded colors
let text_color = Color::from_rgb(1.0, 1.0, 1.0);

// ✅ ACCEPTED - Theme-aware colors
text("Hello")
    .style(theme::Text::Color(
        self.core.system_theme().cosmic().accent_color().into()
    ))
```

### Naming Conventions
- **Variables/Functions:** `snake_case`
- **Types/Structs:** `PascalCase`
- **Constants:** `UPPER_SNAKE_CASE`
- **Modules:** `snake_case`

### File Organization
- Group related functionality in modules
- Keep files under 500 lines when possible
- Separate UI from business logic
- Place tests in same file as code or `tests/` directory

---

## Feature Acceptance Criteria

### For New Features

Every new feature must meet:

1. **Functional Requirements**
   - [ ] Feature works as described in specification
   - [ ] All user stories completed
   - [ ] Edge cases handled gracefully
   - [ ] Error messages are user-friendly

2. **Non-Functional Requirements**
   - [ ] Responsive UI (no freezing)
   - [ ] Acceptable performance (see benchmarks below)
   - [ ] Memory usage reasonable
   - [ ] No memory leaks detected

3. **Integration**
   - [ ] Works with existing features
   - [ ] Follows COSMIC design patterns
   - [ ] Consistent with app architecture
   - [ ] Plugin system integration (if applicable)

4. **User Experience**
   - [ ] Loading states shown
   - [ ] Empty states handled
   - [ ] Error recovery provided
   - [ ] Keyboard navigation works
   - [ ] Accessible to screen readers

5. **Testing**
   - [ ] Manual testing checklist completed
   - [ ] Tested with real devices (if protocol feature)
   - [ ] Tested in both dark and light themes
   - [ ] Tested at different panel sizes (if applet feature)

### For Bug Fixes

Bug fixes must meet:

1. **Fix Verification**
   - [ ] Bug no longer reproducible
   - [ ] Root cause identified and addressed
   - [ ] Fix doesn't introduce new bugs
   - [ ] Related bugs checked and fixed if found

2. **Testing**
   - [ ] Test case added to prevent regression
   - [ ] Manual verification completed
   - [ ] Related functionality still works

3. **Documentation**
   - [ ] Bug fix noted in commit message
   - [ ] GitHub issue updated and closed
   - [ ] User-facing changes documented

---

## Protocol Implementation Criteria

### Packet Handling

Every packet implementation must:

- [ ] Follow KDE Connect protocol v7/8 specification exactly
- [ ] Include newline terminator (`\n`) when sending
- [ ] Handle missing or invalid fields gracefully
- [ ] Log packet contents at `trace` level
- [ ] Validate packet structure before processing
- [ ] Use proper JSON serialization/deserialization
- [ ] Include timestamp in `id` field

**Example:**
```rust
// ✅ ACCEPTED
pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
    let mut json = serde_json::to_vec(self)?;
    json.push(b'\n'); // Protocol requires newline
    Ok(json)
}
```

### Device Discovery

Discovery implementation must:

- [ ] Broadcast on ports 1714-1764 (UDP)
- [ ] Listen on port 1716 for responses
- [ ] Handle network errors gracefully
- [ ] Timeout after reasonable period (5-10 seconds)
- [ ] Support concurrent discovery
- [ ] Parse identity packets correctly
- [ ] Extract all device capabilities

### TLS Pairing

Pairing implementation must:

- [ ] Use TLS 1.3 only
- [ ] Generate valid self-signed certificates
- [ ] Store certificates securely
- [ ] Verify certificate fingerprints
- [ ] Support pairing request/response flow
- [ ] Handle pairing rejection gracefully
- [ ] Clean up on unpair
- [ ] Never transmit in plaintext after pairing

### Plugin System

Every plugin must:

- [ ] Implement the `Plugin` trait completely
- [ ] Declare incoming/outgoing capabilities
- [ ] Handle packets asynchronously
- [ ] Implement proper init/shutdown
- [ ] Log errors without panicking
- [ ] Clean up resources on shutdown
- [ ] Follow plugin naming convention (`kdeconnect.*`)
- [ ] Be tested independently

**Plugin Template:**
```rust
#[async_trait]
impl Plugin for MyPlugin {
    fn id(&self) -> &str { "my_plugin" }

    fn incoming_capabilities(&self) -> Vec<String> {
        vec!["kdeconnect.my_plugin".to_string()]
    }

    fn outgoing_capabilities(&self) -> Vec<String> {
        vec!["kdeconnect.my_plugin.request".to_string()]
    }

    async fn handle_packet(&mut self, packet: Packet) -> Result<()> {
        tracing::debug!("Handling packet: {}", packet.packet_type);
        // Implementation
        Ok(())
    }

    async fn init(&mut self) -> Result<()> {
        tracing::info!("Initializing plugin");
        Ok(())
    }

    async fn shutdown(&mut self) -> Result<()> {
        tracing::info!("Shutting down plugin");
        Ok(())
    }
}
```

### Protocol Compatibility

All protocol changes must:

- [ ] Remain compatible with KDE Connect Android
- [ ] Remain compatible with KDE Connect Desktop
- [ ] Follow protocol version 7/8 specification
- [ ] Be tested with official KDE Connect clients
- [ ] Document any deviations (if unavoidable)

---

## UI/UX Acceptance Criteria

### COSMIC Applet Requirements

Applets must:

- [ ] Display only an icon in the panel (no text by default)
- [ ] Adapt to panel size (horizontal/vertical)
- [ ] Provide popup window for detailed UI
- [ ] Use COSMIC theming system
- [ ] Respect panel configuration
- [ ] Handle multiple monitors correctly
- [ ] Startup in <200ms
- [ ] Use <5MB memory when idle

### User Interface Standards

All UI must:

- [ ] Use COSMIC standard widgets
- [ ] Follow COSMIC spacing guidelines (8px, 12px, 16px, 24px)
- [ ] Show loading states for async operations
- [ ] Provide empty states with helpful messages
- [ ] Display user-friendly error messages
- [ ] Include error recovery options
- [ ] Support keyboard navigation
- [ ] Work in both dark and light themes
- [ ] Use symbolic icons (*-symbolic)

### Responsive Design

UI components must:

- [ ] Adapt to different window sizes
- [ ] Support minimum width/height
- [ ] Reflow content appropriately
- [ ] Show scroll indicators when needed
- [ ] Handle long text gracefully (truncate or wrap)

### Accessibility

All UI must:

- [ ] Support keyboard navigation
- [ ] Use semantic widget types
- [ ] Provide text alternatives for icons
- [ ] Support screen readers (where possible)
- [ ] Use adequate color contrast
- [ ] Respect user font size preferences

---

## Security Requirements

### TLS/Encryption

All encrypted communication must:

- [ ] Use TLS 1.3 (reject older versions)
- [ ] Validate certificates properly
- [ ] Never downgrade to plaintext
- [ ] Use secure random number generation
- [ ] Store keys securely (system keyring)
- [ ] Clear sensitive data from memory

### Input Validation

All inputs must:

- [ ] Validate packet structure before processing
- [ ] Sanitize file names and paths
- [ ] Check file sizes before transfer
- [ ] Validate URLs before opening
- [ ] Prevent path traversal attacks
- [ ] Limit resource consumption

### Code Security

Code must:

- [ ] No unsafe blocks (without justification)
- [ ] No SQL injection vulnerabilities
- [ ] No command injection vulnerabilities
- [ ] No cross-site scripting (XSS) in any UI
- [ ] No hardcoded secrets or credentials
- [ ] Pass `cargo audit` with zero vulnerabilities

---

## Performance Benchmarks

### Latency Requirements

- **Packet processing:** <10ms per packet
- **Device discovery:** <5 seconds to find devices
- **Pairing handshake:** <2 seconds total
- **UI response:** <100ms for user interactions
- **Applet startup:** <200ms

### Memory Requirements

- **Protocol library:** <2MB
- **Applet (idle):** <5MB
- **Applet (active):** <15MB
- **Daemon:** <20MB
- **Full app:** <50MB

### Throughput Requirements

- **File transfer:** ≥5MB/s over local network
- **Packet rate:** ≥100 packets/second
- **Concurrent connections:** ≥5 devices simultaneously

### Testing Performance

Performance tests must verify:

```bash
# Measure memory usage
just bench-memory

# Measure packet processing speed
just bench-packets

# Measure file transfer speed
just bench-transfer

# Profile with flamegraph
just profile
```

---

## Documentation Standards

### Code Documentation

All public APIs must have:

```rust
/// Brief one-line description.
///
/// Longer explanation of functionality, if needed.
///
/// # Arguments
///
/// * `param` - Description of parameter
///
/// # Returns
///
/// Description of return value
///
/// # Errors
///
/// Description of possible errors
///
/// # Examples
///
/// ```
/// use kdeconnect_protocol::Packet;
/// let packet = Packet::new("kdeconnect.ping", json!({}));
/// ```
pub fn function(param: Type) -> Result<Return, Error> {
    // Implementation
}
```

### User Documentation

User-facing features must include:

- [ ] Description in README.md
- [ ] Usage instructions in docs/
- [ ] Screenshots or examples
- [ ] Troubleshooting section
- [ ] FAQ entries (if common questions)

### Change Documentation

All changes must:

- [ ] Update CHANGELOG.md (for releases)
- [ ] Document breaking changes clearly
- [ ] Update relevant documentation files
- [ ] Include migration guide (if needed)

---

## Testing Requirements

### Unit Tests

Required for:

- [ ] All protocol functions
- [ ] All packet serialization/deserialization
- [ ] All plugin implementations
- [ ] Business logic and state management
- [ ] Error handling paths

**Example:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_serialization() {
        let packet = Packet::new("kdeconnect.ping", json!({}));
        let bytes = packet.to_bytes().unwrap();

        assert!(bytes.ends_with(b"\n"));

        let decoded = Packet::from_bytes(&bytes[..bytes.len()-1]).unwrap();
        assert_eq!(decoded.packet_type, "kdeconnect.ping");
    }

    #[tokio::test]
    async fn test_async_operation() {
        let result = async_function().await;
        assert!(result.is_ok());
    }
}
```

### Integration Tests

Required for:

- [ ] Device discovery flow
- [ ] Pairing flow end-to-end
- [ ] Plugin communication
- [ ] File transfer functionality
- [ ] Multi-device scenarios

**Location:** `tests/` directory

### Manual Testing Checklist

Before marking as done:

- [ ] Tested on COSMIC Desktop
- [ ] Tested with Android KDE Connect app
- [ ] Tested in dark theme
- [ ] Tested in light theme
- [ ] Tested with slow network
- [ ] Tested with network interruption
- [ ] Tested error scenarios
- [ ] Tested with multiple devices
- [ ] Tested applet in different panel positions
- [ ] Tested keyboard navigation

### Coverage Requirements

- **Minimum coverage:** 70% overall
- **Critical paths:** 90%+ coverage
- **Protocol code:** 80%+ coverage
- **Plugin code:** 75%+ coverage

```bash
# Check coverage
cargo tarpaulin --out Html
```

---

## Pull Request Checklist

Before submitting a PR, verify:

### Code Quality
- [ ] `cargo fmt` passes
- [ ] `cargo clippy` passes with zero warnings
- [ ] `cargo test` passes all tests
- [ ] `cargo build --release` succeeds
- [ ] No new compiler warnings

### Documentation
- [ ] Public APIs documented
- [ ] README updated (if needed)
- [ ] CHANGELOG updated (if needed)
- [ ] Commit messages are clear

### Testing
- [ ] New tests added
- [ ] All tests pass
- [ ] Manual testing completed
- [ ] Performance acceptable

### Git
- [ ] Branch is up to date with main
- [ ] No merge conflicts
- [ ] Commits are logical and atomic
- [ ] Commit messages follow convention

### PR Description
- [ ] Clear title describing change
- [ ] Description explains what and why
- [ ] Links to related issues
- [ ] Screenshots for UI changes
- [ ] Breaking changes highlighted

### Review
- [ ] Self-reviewed the code
- [ ] Requested reviewers
- [ ] CI/CD checks passing
- [ ] Ready for review

---

## Issue-Specific Acceptance Criteria

### For Protocol Issues (#1-6)

In addition to general criteria:

- [ ] Protocol specification followed exactly
- [ ] Tested with wireshark/tcpdump
- [ ] Tested with official KDE Connect clients
- [ ] Network errors handled gracefully
- [ ] Concurrent operations supported

### For Plugin Issues (#7-11, #14)

In addition to general criteria:

- [ ] Plugin trait fully implemented
- [ ] Capabilities declared correctly
- [ ] Plugin tested independently
- [ ] Tested with real mobile device
- [ ] Resource cleanup verified

### For UI Issues (#12)

In addition to general criteria:

- [ ] Follows COSMIC design patterns
- [ ] Works in all panel positions
- [ ] Responsive to window resize
- [ ] Tested in both themes
- [ ] Icons use symbolic variants

### For Daemon Issue (#13)

In addition to general criteria:

- [ ] Systemd service file provided
- [ ] Handles signals properly (SIGTERM, SIGHUP)
- [ ] DBus interface implemented
- [ ] Graceful startup and shutdown
- [ ] Logging to systemd journal

### For Infrastructure Issues (#15-18)

In addition to general criteria:

- [ ] CI/CD pipeline functional
- [ ] Documentation complete and accurate
- [ ] Package builds successfully
- [ ] Installation instructions tested

---

## Version Release Criteria

### For v0.1.0 (Phase 1 Complete)

Must have:
- [ ] All Phase 1 issues closed
- [ ] Core protocol working
- [ ] Device discovery functional
- [ ] Pairing working
- [ ] Plugin system ready
- [ ] Test coverage ≥70%
- [ ] Documentation complete
- [ ] No critical bugs

### For v1.0.0 (Public Release)

Must have:
- [ ] All core plugins working
- [ ] UI polished and tested
- [ ] Performance benchmarks met
- [ ] Security audit passed
- [ ] Documentation comprehensive
- [ ] Package in nixpkgs
- [ ] CI/CD pipeline operational
- [ ] User testing completed
- [ ] No known critical bugs
- [ ] Backward compatibility plan

---

## Continuous Improvement

These acceptance criteria are living documents. Update them:

- When patterns emerge from development
- When new requirements are discovered
- After retrospectives
- When technology changes
- When team learns better practices

**Last Updated:** January 2026
**Version:** 1.0.0
