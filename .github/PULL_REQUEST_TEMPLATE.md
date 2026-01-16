## ⚠️ Pre-Commit Checks Completed

Before submitting this PR, confirm you ran **BOTH** required checks:

- [ ] **@cosmic-code-reviewer /pre-commit-check** - COSMIC Desktop code review
- [ ] **@code-simplifier** - Code simplification and quality

> **Required for all PRs**. These checks catch hard-coded values, unsafe error handling, and COSMIC pattern violations.
>
> Exception: Skip only for trivial changes (typo fixes, comments only).

---

## Description

<!-- Provide a clear description of the changes -->

## Type of Change

- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update
- [ ] Code refactoring
- [ ] Performance improvement

## Changes Made

<!-- List the specific changes in this PR -->

-
-
-

## Testing

### Manual Testing

<!-- Describe how you tested these changes -->

- [ ] Tested with real Android/iOS device
- [ ] Tested in light theme
- [ ] Tested in dark theme
- [ ] Tested all affected plugins
- [ ] Verified no regressions

### Test Configuration

- **Device**: (e.g., Google Pixel 7, KDE Connect app version X.Y)
- **OS**: (e.g., NixOS 24.05, Ubuntu 24.04)
- **COSMIC Version**: (e.g., COSMIC Alpha 2)

### Automated Tests

- [ ] All existing tests pass (`just test`)
- [ ] Added new tests for new functionality
- [ ] Tests cover both success and error paths

## Code Quality

- [ ] Code follows project style guidelines
- [ ] Linter passes (`just lint`)
- [ ] No compiler warnings
- [ ] No hard-coded values (colors, dimensions, radii)
- [ ] No `.unwrap()` or `.expect()` calls
- [ ] Proper error handling with logging

## Documentation

- [ ] Updated relevant documentation
- [ ] Added doc comments to public APIs
- [ ] Updated CHANGELOG (if applicable)
- [ ] Added usage examples (if needed)

## Screenshots/Videos

<!-- For UI changes, include screenshots of both light and dark themes -->
<!-- For complex interactions, include screen recordings -->

### Light Theme
<!-- Screenshot here -->

### Dark Theme
<!-- Screenshot here -->

## Related Issues

<!-- Link related issues using #issue_number -->

Fixes #
Closes #
Related to #

## Additional Context

<!-- Add any other context about the PR here -->

## Checklist

- [ ] My code follows the project's code style guidelines
- [ ] I have performed a self-review of my code
- [ ] I have commented my code, particularly in hard-to-understand areas
- [ ] I have made corresponding changes to the documentation
- [ ] My changes generate no new warnings
- [ ] I have added tests that prove my fix is effective or that my feature works
- [ ] New and existing unit tests pass locally with my changes
- [ ] Any dependent changes have been merged and published

## For Reviewers

<!-- Help reviewers by highlighting specific areas that need attention -->

**Focus Areas:**
-
-

**Known Limitations:**
-

---

**By submitting this PR, I confirm that:**
- I have run the mandatory pre-commit checks
- My code adheres to COSMIC Desktop best practices
- I have tested my changes thoroughly
