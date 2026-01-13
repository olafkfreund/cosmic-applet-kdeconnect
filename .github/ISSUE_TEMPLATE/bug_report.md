---
name: Bug Report
about: Report a bug in cosmic-applet-kdeconnect
title: '[BUG] '
labels: 'bug'
assignees: ''
---

## Bug Description

**What happened?**
<!-- A clear and concise description of the bug -->

**What did you expect to happen?**
<!-- What you expected to happen instead -->

## Steps to Reproduce

1.
2.
3.
4.

**Reproducibility:**
- [ ] Always
- [ ] Sometimes (intermittent)
- [ ] Rarely

## Environment

**System Information:**
- OS: <!-- e.g., NixOS 24.11, Ubuntu 24.04 -->
- COSMIC Desktop Version: <!-- e.g., Alpha 4 -->
- cosmic-applet-kdeconnect Version: <!-- e.g., v0.1.0, git main -->
- Rust Version: <!-- output of `rustc --version` -->

**Device Information (if applicable):**
- Mobile Device: <!-- e.g., Android 14, iOS 17 -->
- KDE Connect Version: <!-- version on mobile device -->
- Network: <!-- WiFi, same subnet, etc. -->

## Logs and Output

**Relevant Logs:**
```
# Run with: RUST_LOG=debug cosmic-applet-kdeconnect
# Paste relevant log output here
```

**Error Messages:**
```
# Any error messages or stack traces
```

**Screenshots/Videos:**
<!-- If applicable, add screenshots or videos demonstrating the issue -->

## Impact

**Severity:**
- [ ] Critical - Application crashes or data loss
- [ ] High - Major functionality broken
- [ ] Medium - Feature doesn't work as expected
- [ ] Low - Minor issue or cosmetic

**Workaround:**
<!-- Is there a workaround? If so, describe it -->

## Additional Context

**Related Issues:**
<!-- Link to related issues if any -->

**Protocol Packets (if relevant):**
```json
// Paste relevant packet dumps if protocol issue
```

**System State:**
<!-- Any other relevant information about system state -->

---

## For Maintainers

**Acceptance Criteria for Fix:**
<!-- See ACCEPTANCE_CRITERIA.md -->

- [ ] Bug no longer reproducible
- [ ] Root cause identified and fixed
- [ ] Test case added to prevent regression
- [ ] Related functionality still works
- [ ] Fix doesn't introduce new issues
