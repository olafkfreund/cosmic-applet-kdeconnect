# cosmic-applet-kdeconnect - Claude Project Context

## Project Identity

**Name**: cosmic-applet-kdeconnect  
**Type**: Rust application + COSMIC Desktop applet  
**Purpose**: Native KDE Connect implementation for COSMIC Desktop Environment  
**Primary Developer**: Olaf (Cloud Architect & DevOps Leader)  
**Development Environment**: NixOS with COSMIC Desktop on Wayland  
**Status**: Foundation phase - protocol implementation in progress

## Project Overview

This is a ground-up implementation of the KDE Connect protocol in Rust, specifically designed for the COSMIC Desktop Environment. The project aims to provide seamless device synchronization between COSMIC Desktop and Android/iOS mobile devices.

### Key Components

1. **kdeconnect-protocol** (Library Crate)
   - Core protocol implementation
   - Device discovery via UDP broadcast and mDNS
   - TLS pairing and certificate management
   - Plugin architecture for extensibility
   - Transport abstraction (TCP, UDP, Bluetooth)

2. **cosmic-applet-kdeconnect** (Binary Crate)
   - COSMIC panel/dock applet
   - Quick device status view
   - Connection management UI
   - Notification indicators

3. **cosmic-kdeconnect** (Binary Crate)
   - Full desktop application
   - Comprehensive device management
   - Plugin configuration
   - Settings and preferences

4. **kdeconnect-daemon** (Binary Crate)
   - Background service
   - Maintains device connections
   - Handles plugin operations
   - System integration

## Technical Architecture

### Technology Stack

**Core Languages & Frameworks:**
- Rust (2021 edition)
- libcosmic (GUI toolkit based on iced)
- tokio (async runtime)

**Key Dependencies:**
```toml
libcosmic = { git = "https://github.com/pop-os/libcosmic" }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
rustls = "0.23"
zbus = "4"
mdns-sd = "0.11"
```

**System Integration:**
- zbus for DBus communication
- Wayland protocols for COSMIC integration
- MPRIS2 for media control
- Freedesktop notifications

### Protocol Specification

**KDE Connect Protocol v7/8:**
- JSON-based packet format
- TLS 1.3 encryption required
- UDP broadcast for discovery (ports 1714-1764)
- TCP for data transfer (ports 1714-1764)
- Certificate-based device pairing

**Packet Structure:**
```json
{
  "id": 1234567890,
  "type": "kdeconnect.packet.type",
  "body": {
    "key": "value"
  }
}
```

**Supported Plugins (Planned):**
- battery: Battery status reporting
- clipboard: Clipboard synchronization
- notification: Notification mirroring
- share: File and URL sharing
- mpris: Media player control
- ping: Connectivity testing
- findmyphone: Remote device location
- runcommand: Execute commands remotely
- sftp: File system access
- sms: SMS messaging

## Development Environment

### NixOS Configuration

The project uses a Nix flake for reproducible development:

**Key Features:**
- Rust toolchain (stable)
- COSMIC Desktop libraries
- libcosmic dependencies
- Development tools (rust-analyzer, clippy)
- Just command runner

**Firewall Requirements:**
```nix
networking.firewall = {
  allowedTCPPortRanges = [{ from = 1714; to = 1764; }];
  allowedUDPPortRanges = [{ from = 1714; to = 1764; }];
};
```

### Build System

Using **Just** as the command runner (justfile):
- `just build` - Build all components
- `just test` - Run all tests
- `just run-applet` - Run applet in development
- `just install` - Install to system
- `just fmt` - Format code
- `just lint` - Run clippy

## Development Phases

### Phase 1: Foundation (Current)
**Goals:**
- Set up project structure
- Implement core protocol library
- Device discovery mechanism
- TLS pairing implementation
- Basic packet handling

**Deliverables:**
- Working protocol library
- Device discovery and pairing
- Unit tests for core functionality

### Phase 2: Basic Applet
**Goals:**
- COSMIC applet implementation
- Device list UI
- Connection status display
- Basic file sharing

**Deliverables:**
- Functional panel applet
- File send capability
- Device status notifications

### Phase 3: Core Features
**Goals:**
- Notification synchronization
- Clipboard sharing
- Battery status display
- Media player control

**Deliverables:**
- Working notification sync
- Clipboard integration
- MPRIS2 integration

### Phase 4: Advanced Features
**Goals:**
- SMS messaging
- Remote input
- Run commands
- Bluetooth transport

**Deliverables:**
- Full application UI
- Complete plugin suite
- Bluetooth connectivity

### Phase 5: Integration & Polish
**Goals:**
- COSMIC Files integration
- COSMIC Notifications integration
- Performance optimization
- Documentation

**Deliverables:**
- Public release v1.0
- Comprehensive documentation
- Distribution packages

## Design Decisions

### Why Rust?
- Memory safety without garbage collection
- Excellent async/await support
- Strong type system prevents bugs
- COSMIC Desktop is Rust-native
- Growing ecosystem for system programming

### Why Not Reuse Existing Code?
- KDE Connect (C++/Qt): Heavy Qt dependency, not COSMIC-native
- GSConnect (JavaScript): GNOME Shell extension, not portable
- Valent (C/GTK): GTK-based, different architecture

**Our Approach:**
- Pure Rust implementation
- Native libcosmic integration
- Standalone processes (not extensions)
- Wayland-first design

### Architecture Patterns

**Protocol Library:**
- Async-first design
- Plugin trait for extensibility
- Transport abstraction
- Clear separation of concerns

**Applet Design:**
- Lightweight UI
- Minimal resource usage
- Fast startup time
- Responsive interactions

**Security Model:**
- TLS 1.3 mandatory
- Certificate pinning
- Secure key storage
- No plaintext transmission

## Code Organization

### Module Structure

```
kdeconnect-protocol/src/
├── lib.rs              # Public API exports
├── discovery.rs        # Device discovery
├── pairing.rs          # TLS pairing logic
├── packet.rs           # Packet serialization
├── device.rs           # Device management
├── error.rs            # Error types
├── config.rs           # Configuration
├── transport/          # Transport layer
│   ├── mod.rs
│   ├── tcp.rs          # TCP transport
│   ├── udp.rs          # UDP broadcast
│   └── bluetooth.rs    # Bluetooth (future)
└── plugins/            # Plugin implementations
    ├── mod.rs
    ├── battery.rs
    ├── clipboard.rs
    ├── notification.rs
    ├── share.rs
    └── ...
```

### Coding Standards

**Style:**
- Use `rustfmt` with default settings
- Follow Rust API guidelines
- Document all public APIs
- Write unit tests for all modules

**Error Handling:**
- Use `Result<T, Error>` consistently
- Define custom error types
- Provide context with `anyhow` for applications
- Use `thiserror` for library errors

**Async Patterns:**
- Use `async-trait` for traits
- Prefer `tokio::spawn` for background tasks
- Use channels for cross-task communication
- Implement graceful shutdown

## Testing Strategy

### Unit Tests
- Test all protocol functions
- Mock network operations
- Test packet serialization
- Verify plugin behavior

### Integration Tests
- Test device discovery
- Test pairing flow
- Test plugin communication
- Test file transfers

### Manual Testing
- Test with real Android devices
- Test with KDE Connect desktop
- Verify firewall handling
- Check error recovery

## Documentation Requirements

### Code Documentation
- Rustdoc for all public APIs
- Examples in documentation
- Architecture documentation
- Protocol documentation

### User Documentation
- Installation guide
- User manual
- Troubleshooting guide
- FAQ

## Common Commands

### Development
```bash
# Enter dev shell
nix develop

# Build everything
just build

# Run tests
just test

# Run applet
just run-applet

# Check code
just check
```

### Debugging
```bash
# Enable debug logging
RUST_LOG=debug just run-applet

# Run with specific protocol logging
RUST_LOG=kdeconnect_protocol=trace just run-daemon

# Test discovery
just test-discovery
```

## References

### Official Documentation
- [KDE Connect Protocol](https://invent.kde.org/network/kdeconnect-kde)
- [Valent Protocol Docs](https://valent.andyholmes.ca/documentation/protocol.html)
- [libcosmic Book](https://pop-os.github.io/libcosmic-book/)
- [COSMIC UX Guidelines](https://system76.com/cosmic/ux)

### Implementation References
- KDE Connect Desktop: https://github.com/KDE/kdeconnect-kde
- GSConnect: https://github.com/GSConnect/gnome-shell-extension-gsconnect
- Valent: https://github.com/andyholmes/valent
- KDE Connect Android: https://invent.kde.org/network/kdeconnect-android

### Rust Resources
- [tokio documentation](https://tokio.rs)
- [zbus documentation](https://docs.rs/zbus)
- [rustls documentation](https://docs.rs/rustls)
- [libcosmic examples](https://github.com/pop-os/libcosmic/tree/master/examples)

## Project Goals

### Primary Goals
1. ✅ Native COSMIC Desktop integration
2. ✅ Rust-based implementation
3. ✅ Protocol compatibility with KDE Connect
4. ✅ Lightweight and efficient
5. ✅ Secure by default

### Non-Goals
- Supporting older protocol versions (< v7)
- Windows/macOS support (initially)
- Custom protocol extensions
- Cloud-based synchronization

## Contributing Guidelines

### Before Contributing
1. Read the code of conduct
2. Check existing issues
3. Discuss major changes first
4. Follow coding standards
5. Write tests

### Pull Request Process
1. Fork the repository
2. Create feature branch
3. Write code + tests
4. Run `just check`
5. Submit PR with description

## Known Challenges

### Technical Challenges
- TLS certificate management
- Efficient device discovery
- Bluetooth integration complexity
- Cross-platform compatibility

### Design Challenges
- Balancing features vs. simplicity
- UI/UX consistency with COSMIC
- Performance optimization
- Error recovery strategies

## Success Metrics

### Technical Metrics
- Test coverage > 80%
- Zero panics in production
- <50ms response time
- <10MB memory footprint (daemon)

### User Metrics
- Easy pairing process
- Reliable connections
- Fast file transfers
- Responsive UI

## Future Considerations

### Potential Enhancements
- WebRTC for direct connections
- End-to-end encryption beyond TLS
- Advanced filtering rules
- Plugin marketplace
- Cloud relay for remote connections

### Platform Expansion
- Windows support
- macOS support
- Other Linux desktop environments
- Mobile Linux (Plasma Mobile)

## Notes for Claude

### When Working on This Project

**Always:**
- Follow Rust best practices
- Use async/await patterns
- Write comprehensive tests
- Document public APIs
- Consider security implications

**Remember:**
- Olaf is experienced with DevOps and infrastructure
- NixOS is the primary development platform
- COSMIC Desktop is Wayland-only
- The protocol must remain compatible with KDE Connect

**Ask About:**
- UI/UX design decisions
- Feature prioritization
- Security considerations
- Performance requirements

### Code Generation Preferences

**Style:**
- Idiomatic Rust code
- Clear variable names
- Comprehensive error handling
- Well-structured modules

**Documentation:**
- Include rustdoc comments
- Provide usage examples
- Explain complex algorithms
- Reference protocol specs

**Testing:**
- Unit tests for functions
- Integration tests for features
- Property-based tests when appropriate
- Mock external dependencies

---

Last Updated: January 2026
Project Version: 0.1.0-alpha
Protocol Version: 7/8
