# Contributing to cosmic-applet-kdeconnect

Thank you for your interest in contributing to cosmic-applet-kdeconnect! This document provides guidelines and instructions for contributing.

## Code of Conduct

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). Please be respectful and constructive in all interactions.

## Getting Started

### Prerequisites

- Rust 1.70 or later
- NixOS (recommended) or Linux with development libraries
- COSMIC Desktop Environment (for testing)
- Git

### Development Setup

1. **Fork and clone the repository**
   ```bash
   git clone https://github.com/yourusername/cosmic-applet-kdeconnect.git
   cd cosmic-applet-kdeconnect
   ```

2. **Enter development environment**
   ```bash
   # NixOS users
   nix develop
   
   # Or with shell.nix
   nix-shell
   ```

3. **Build the project**
   ```bash
   just build
   ```

4. **Run tests**
   ```bash
   just test
   ```

## Acceptance Criteria

**Before you start developing, read:**
- [ACCEPTANCE_CRITERIA.md](ACCEPTANCE_CRITERIA.md) - Complete definition of done and quality standards

All contributions must meet these criteria to be merged. This ensures consistent quality across the project.

## Development Workflow

### 1. Create a Branch

Create a feature branch for your work:

```bash
git checkout -b feature/your-feature-name
```

Branch naming conventions:
- `feature/` - New features
- `fix/` - Bug fixes
- `docs/` - Documentation updates
- `refactor/` - Code refactoring
- `test/` - Test additions or fixes

### 2. Make Changes

- Write clear, idiomatic Rust code
- Follow the existing code style
- Add tests for new functionality
- Update documentation as needed
- Keep commits focused and atomic

### 3. Code Quality

Before submitting, ensure your code passes all checks:

```bash
# Run all checks
just check

# Or individually:
just fmt        # Format code
just lint       # Run clippy
just test       # Run tests
```

### 4. Commit Guidelines

Write clear commit messages following conventional commits:

```
type(scope): brief description

Longer description if needed, explaining what and why,
not how.

Fixes #123
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Formatting, missing semicolons, etc.
- `refactor`: Code restructuring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples:**
```
feat(protocol): implement battery plugin
fix(applet): correct popup positioning on left panel
docs(readme): add installation instructions
```

### 5. Submit Pull Request

1. Push your branch to your fork
2. Create a Pull Request on GitHub
3. Fill out the PR template completely
4. Link related issues
5. Wait for review

## What to Contribute

### Good First Issues

Look for issues labeled `good first issue` - these are well-defined tasks suitable for newcomers.

### Areas Needing Help

- **Protocol Implementation**: Core plugins (battery, clipboard, notification, etc.)
- **UI/UX**: Applet interface improvements
- **Documentation**: User guides, API documentation
- **Testing**: Unit tests, integration tests
- **Bug Fixes**: See issues labeled `bug`

### Feature Requests

Before implementing a new feature:
1. Check if an issue exists
2. If not, create an issue describing the feature
3. Wait for discussion and approval
4. Implement once approved

## Code Guidelines

### Rust Style

Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/):

- Use `rustfmt` for formatting (automatic with `just fmt`)
- Follow naming conventions (snake_case, CamelCase)
- Write documentation for public APIs
- Prefer explicit types in public APIs
- Use `Result` for operations that can fail

### Architecture Patterns

**Protocol Library:**
- Pure async/await
- Plugin-based architecture
- Clean separation of concerns
- Transport abstraction

**Applet:**
- Minimal resource usage
- Responsive UI
- Follow COSMIC design patterns
- Use libcosmic widgets

### Error Handling

```rust
// Use thiserror for library errors
#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("Connection failed: {0}")]
    Connection(#[from] std::io::Error),
    // ...
}

// Use anyhow for application errors
fn main() -> anyhow::Result<()> {
    // ...
}
```

### Async Patterns

```rust
// Use async-trait for traits
#[async_trait]
pub trait Plugin {
    async fn handle_packet(&mut self, packet: Packet) -> Result<(), Error>;
}

// Spawn background tasks properly
tokio::spawn(async move {
    // Task code
});
```

### Testing

Write tests for all new functionality:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // Test code
    }

    #[tokio::test]
    async fn test_async_feature() {
        // Async test code
    }
}
```

### Documentation

Document all public APIs:

```rust
/// Brief description of the function.
///
/// More detailed explanation if needed.
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
/// Description of errors that can occur
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

## Testing

### Running Tests

```bash
# All tests
just test

# Specific package
cargo test -p kdeconnect-protocol

# With output
just test-verbose

# Integration tests
just test-integration
```

### Writing Tests

- Unit tests in the same file as code
- Integration tests in `tests/` directory
- Mock external dependencies
- Test error cases

### Manual Testing

Test with real devices:
1. Install KDE Connect on Android/iOS
2. Ensure devices are on same network
3. Test all features you modified
4. Check error handling

## Documentation

### Code Documentation

- Use rustdoc comments (`///`)
- Include examples in documentation
- Document public APIs thoroughly
- Keep docs up to date

### User Documentation

Update relevant docs in:
- `README.md` - Project overview
- `.claude/claude.md` - Project context
- `.claude/skills/` - Development guides

## Review Process

### What Reviewers Look For

- Code quality and style
- Test coverage
- Documentation
- Performance implications
- Security considerations
- Compatibility with KDE Connect protocol

### Responding to Feedback

- Be open to suggestions
- Ask questions if unclear
- Make requested changes
- Mark conversations as resolved
- Be patient and respectful

### After Approval

- Squash commits if requested
- Ensure CI passes
- Wait for maintainer to merge

## Security

### Reporting Vulnerabilities

**Do not open public issues for security vulnerabilities.**

Email: security@yourproject.org

Include:
- Description of vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

### Security Considerations

When contributing, consider:
- Input validation
- TLS certificate verification
- Secure key storage
- Rate limiting
- Error message information disclosure

## Release Process

Releases are handled by maintainers:

1. Version bump in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Create git tag
4. Build release binaries
5. Publish to package repositories

## Communication

### Where to Ask Questions

- **GitHub Discussions**: General questions, ideas
- **GitHub Issues**: Bug reports, feature requests
- **Pull Requests**: Code-specific discussions
- **COSMIC Community**: [Mattermost](https://chat.pop-os.org/)

### Getting Help

If stuck:
1. Read existing documentation
2. Search closed issues/PRs
3. Ask in discussions
4. Join community chat

## Recognition

Contributors are recognized in:
- Git commit history
- Release notes
- Contributors file (coming soon)

## License

By contributing, you agree that your contributions will be licensed under the GPL-3.0-or-later license.

## Additional Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Async Book](https://rust-lang.github.io/async-book/)
- [KDE Connect Protocol](https://invent.kde.org/network/kdeconnect-kde)
- [libcosmic Book](https://pop-os.github.io/libcosmic-book/)
- [COSMIC Desktop](https://system76.com/cosmic)

## Questions?

Don't hesitate to ask! We're here to help.

Thank you for contributing to cosmic-applet-kdeconnect! 🚀
