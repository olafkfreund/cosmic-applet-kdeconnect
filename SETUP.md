# cosmic-applet-kdeconnect Setup Files

This directory contains all the necessary files to set up the cosmic-applet-kdeconnect project.

## Files Included

### Documentation
- `README.md` - Comprehensive project overview and documentation
- `CONTRIBUTING.md` - Contribution guidelines
- `LICENSE` - GPL-3.0-or-later license
- `SETUP.md` - This file

### Build Configuration
- `Cargo.toml` - Rust workspace configuration
- `justfile` - Build commands and tasks
- `.gitignore` - Git ignore patterns

### NixOS Development Environment
- `flake.nix` - Nix flake for reproducible development (recommended)
- `shell.nix` - Alternative Nix shell configuration

### Claude AI Integration
- `.claude/claude.md` - Project context for Claude AI
- `.claude/skills/kdeconnect-protocol.md` - KDE Connect protocol implementation guide
- `.claude/skills/cosmic-applet-dev.md` - COSMIC applet development guide

### Setup Script
- `init-project.sh` - Automated project structure initialization

## Quick Start

### 1. Initialize Git Repository

```bash
# If you haven't created the repo yet
git init
git add .
git commit -m "Initial commit: Project structure and documentation"
```

### 2. Run Project Initialization

```bash
./init-project.sh
```

This script will:
- Create all necessary directories
- Generate Cargo.toml files for all crates
- Create initial source files
- Set up the basic project structure

### 3. Enter Development Environment

**Using Nix Flakes (recommended):**
```bash
nix develop
```

**Using nix-shell:**
```bash
nix-shell
```

**Without Nix:**
Install dependencies manually:
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install just
cargo install just

# Install system dependencies (Ubuntu/Debian)
sudo apt install libxkbcommon-dev libwayland-dev libdbus-1-dev \
                 libssl-dev libfontconfig-dev libfreetype-dev pkg-config
```

### 4. Build the Project

```bash
just build
```

### 5. Run Tests

```bash
just test
```

## Project Structure After Initialization

```
cosmic-applet-kdeconnect/
├── .claude/                          # Claude AI context
│   ├── claude.md
│   └── skills/
│       ├── kdeconnect-protocol.md
│       └── cosmic-applet-dev.md
├── kdeconnect-protocol/              # Core protocol library
│   ├── src/
│   │   ├── lib.rs
│   │   ├── error.rs
│   │   ├── discovery.rs
│   │   ├── pairing.rs
│   │   ├── packet.rs
│   │   ├── device.rs
│   │   ├── plugins/
│   │   └── transport/
│   ├── tests/
│   └── Cargo.toml
├── cosmic-applet-kdeconnect/         # Panel applet
│   ├── src/
│   │   └── main.rs
│   ├── data/
│   │   └── cosmic-applet-kdeconnect.desktop
│   └── Cargo.toml
├── cosmic-kdeconnect/                # Full application
│   ├── src/
│   │   └── main.rs
│   └── Cargo.toml
├── kdeconnect-daemon/                # Background daemon
│   ├── src/
│   │   └── main.rs
│   └── Cargo.toml
├── Cargo.toml                        # Workspace config
├── justfile                          # Build commands
├── flake.nix                         # Nix flake
├── shell.nix                         # Nix shell
├── README.md                         # Project docs
├── CONTRIBUTING.md                   # Contribution guide
├── LICENSE                           # GPL-3.0
└── .gitignore                        # Git ignore
```

## Development Workflow

### Building Components

```bash
# Build everything
just build

# Build specific component
just build-protocol
just build-applet
just build-app
just build-daemon

# Release build
just build-release
```

### Running Components

```bash
# Run applet
just run-applet

# Run full application
just run-app

# Run daemon
just run-daemon
```

### Code Quality

```bash
# Format code
just fmt

# Lint with clippy
just lint

# Run all checks
just check
```

### Testing

```bash
# All tests
just test

# With output
just test-verbose

# Specific tests
just test-protocol
just test-integration
```

## NixOS Configuration

### Firewall Setup

Add to your `configuration.nix`:

```nix
networking.firewall = {
  allowedTCPPortRanges = [
    { from = 1714; to = 1764; }
  ];
  allowedUDPPortRanges = [
    { from = 1714; to = 1764; }
  ];
};
```

### System Installation (Future)

Once the package is ready, you can add it to your system:

```nix
environment.systemPackages = with pkgs; [
  cosmic-applet-kdeconnect
];
```

## Using Claude AI

This project includes Claude-specific documentation:

1. **Project Context** (`.claude/claude.md`):
   - Comprehensive project overview
   - Technical architecture
   - Development phases
   - Design decisions

2. **KDE Connect Protocol Skill** (`.claude/skills/kdeconnect-protocol.md`):
   - Protocol implementation patterns
   - Packet structures
   - Plugin development
   - Testing strategies

3. **COSMIC Applet Development Skill** (`.claude/skills/cosmic-applet-dev.md`):
   - Applet architecture
   - UI components
   - Panel integration
   - Best practices

When using Claude AI for development:
- Reference these files for context
- Ask protocol-specific questions
- Get help with COSMIC integration
- Review code patterns and examples

## Common Tasks

### Add a New Plugin

```bash
just new-plugin battery
```

This creates a plugin template at:
`kdeconnect-protocol/src/plugins/battery.rs`

### Generate Documentation

```bash
just doc
```

Opens generated documentation in your browser.

### Check for Outdated Dependencies

```bash
just outdated
```

### Security Audit

```bash
just audit
```

### Create a Release

```bash
just release 0.1.0
```

## Troubleshooting

### Build Fails with Missing Libraries

Make sure you're in the Nix development environment:
```bash
nix develop
# or
nix-shell
```

### "command not found: just"

Install just:
```bash
cargo install just
```

Or use the Nix environment which includes it.

### Applet Doesn't Show in Panel

1. Check if the desktop entry is installed:
   ```bash
   ls /usr/share/applications/cosmic-applet-kdeconnect.desktop
   ```

2. Ensure NoDisplay=true is set in the desktop entry

3. Restart the COSMIC panel

### Network Discovery Not Working

1. Check firewall settings (ports 1714-1764)
2. Ensure devices are on the same network
3. Check logs: `RUST_LOG=debug just run-applet`

## Next Steps

1. **Implement Core Protocol**
   - Device discovery
   - TLS pairing
   - Basic packet handling

2. **Create Basic Applet**
   - Device list UI
   - Connection status
   - Basic interactions

3. **Implement Plugins**
   - Battery plugin
   - Notification plugin
   - File sharing
   - Clipboard sync

4. **Testing**
   - Unit tests
   - Integration tests
   - Manual testing with real devices

5. **Documentation**
   - API documentation
   - User guide
   - Troubleshooting guide

## Resources

- [KDE Connect Protocol](https://invent.kde.org/network/kdeconnect-kde)
- [Valent Protocol Reference](https://valent.andyholmes.ca/documentation/protocol.html)
- [libcosmic Documentation](https://pop-os.github.io/libcosmic-book/)
- [COSMIC Desktop](https://system76.com/cosmic)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)

## Support

- **Issues**: GitHub Issues
- **Discussions**: GitHub Discussions
- **Community**: Pop!_OS Mattermost

## License

This project is licensed under GPL-3.0-or-later.

---

Happy coding! 🚀
