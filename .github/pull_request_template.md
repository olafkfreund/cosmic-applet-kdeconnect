## Description

<!-- Provide a clear description of what this PR does -->

**What:**
<!-- What changes does this PR introduce? -->

**Why:**
<!-- Why are these changes needed? What problem do they solve? -->

**How:**
<!-- How were these changes implemented? Any technical details worth mentioning? -->

Fixes #<!-- issue number -->

## Type of Change

<!-- Check all that apply -->

- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update
- [ ] Code refactoring
- [ ] Performance improvement
- [ ] Test coverage improvement

## Acceptance Criteria Checklist

<!-- See ACCEPTANCE_CRITERIA.md for detailed requirements -->

### Code Quality ✅
- [ ] Code compiles without errors or warnings
- [ ] No `unwrap()` or `expect()` in production code
- [ ] All errors properly handled with `Result` or logged
- [ ] `cargo fmt` passes
- [ ] `cargo clippy` passes with zero warnings
- [ ] Follows Rust best practices and idioms

### Testing 🧪
- [ ] Unit tests added/updated and passing
- [ ] Integration tests added/updated (if applicable)
- [ ] Test coverage ≥70% for new code
- [ ] Manual testing completed
- [ ] Tested on COSMIC Desktop
- [ ] Edge cases tested

### Documentation 📚
- [ ] Public APIs have rustdoc comments
- [ ] Complex logic has inline comments
- [ ] README updated (if needed)
- [ ] CHANGELOG.md updated (if applicable)
- [ ] User-facing changes documented

### Protocol Compliance (if applicable) 🔌
- [ ] Follows KDE Connect protocol v7/8 specification
- [ ] Tested with official KDE Connect clients
- [ ] Packets include newline terminator
- [ ] TLS 1.3 used for encryption
- [ ] Certificate validation implemented

### UI/UX (if applicable) 🎨
- [ ] Uses COSMIC standard widgets
- [ ] Tested in dark theme
- [ ] Tested in light theme
- [ ] Loading states shown
- [ ] Empty states handled
- [ ] Error recovery provided
- [ ] Keyboard navigation works

### Security 🔒
- [ ] Input validation implemented
- [ ] No hardcoded secrets
- [ ] `cargo audit` passes
- [ ] No unsafe blocks (or justified)
- [ ] Sensitive data handled securely

### Performance ⚡
- [ ] No blocking operations on UI thread
- [ ] Memory usage reasonable
- [ ] No performance regressions
- [ ] Profiled if performance-critical

## Screenshots/Videos

<!-- For UI changes, include before/after screenshots or videos -->

**Before:**
<!-- Screenshot or description of old behavior -->

**After:**
<!-- Screenshot or description of new behavior -->

## Testing Performed

### Automated Tests
```bash
# Output of test run
cargo test
```

### Manual Testing
<!-- Describe manual testing steps and results -->
- [ ] Tested feature X with scenario Y
- [ ] Verified error handling with invalid input Z
- [ ] Checked performance with N items

### Compatibility Testing (if protocol change)
- [ ] Tested with KDE Connect Android
- [ ] Tested with KDE Connect Desktop
- [ ] Backward compatibility maintained

## Breaking Changes

<!-- If this PR includes breaking changes, describe them here -->

**What breaks:**
<!-- What functionality changes or breaks? -->

**Migration path:**
<!-- How should users/developers adapt to these changes? -->

## Additional Notes

<!-- Any additional information reviewers should know -->

## Checklist Before Requesting Review

- [ ] Self-reviewed the code
- [ ] Branch is up to date with main
- [ ] No merge conflicts
- [ ] Commits are logical and well-described
- [ ] CI/CD checks passing
- [ ] All checkboxes above completed

---

**By submitting this PR, I confirm that:**
- I have read and followed the [ACCEPTANCE_CRITERIA.md](../ACCEPTANCE_CRITERIA.md)
- I have read and followed the [CONTRIBUTING.md](../CONTRIBUTING.md)
- My code follows the project's coding standards
- I am ready for code review
