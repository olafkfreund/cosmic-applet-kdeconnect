# Pre-Commit Reminder

## ⚠️ REQUIRED: Run These Checks Before EVERY Commit

### Quick Reference

```bash
# Step 1: COSMIC Code Review
@cosmic-code-reviewer /pre-commit-check

# Step 2: Code Simplification
@code-simplifier review the changes we made
```

### Why These Checks?

**@cosmic-code-reviewer** catches:
- ❌ Hard-coded colors, dimensions, radii
- ❌ `.unwrap()` and `.expect()` calls
- ❌ Missing error handling
- ❌ Theme integration issues
- ❌ COSMIC Desktop pattern violations
- ❌ Accessibility issues

**@code-simplifier** ensures:
- ✅ Clean, idiomatic Rust code
- ✅ Removal of redundant patterns
- ✅ Consistent code style
- ✅ Improved maintainability
- ✅ Better performance

### Integration

Add to your workflow:

```bash
# Before git commit
@cosmic-code-reviewer /pre-commit-check
@code-simplifier review the changes we made
git add .
git commit -m "feat: your changes"
```

### Exception

Skip only for trivial changes:
- Typo fixes
- Comment-only changes
- Documentation updates (no code)

**For all code changes: Both checks are MANDATORY.**

---

See [CONTRIBUTING.md](../CONTRIBUTING.md) for complete guidelines.
