# Git Hooks for cosmic-applet-kdeconnect

This directory contains Git hooks that automatically check and format code before commits.

## Installation

### Automatic (Recommended)
```bash
just install-hooks
```

Or as part of initial setup:
```bash
just setup
```

### Manual
```bash
cp hooks/pre-commit .git/hooks/pre-commit
cp hooks/commit-msg .git/hooks/commit-msg
chmod +x .git/hooks/pre-commit
chmod +x .git/hooks/commit-msg
```

## Available Hooks

### pre-commit

Runs before each commit to ensure code quality:

1. **Code Formatting** (`cargo fmt`)
   - Automatically formats code to project standards
   - Auto-stages formatted files

2. **Linting** (`cargo clippy`)
   - Checks for common mistakes and anti-patterns
   - Enforces warnings-as-errors

3. **Compilation Check** (`cargo check`)
   - Verifies code compiles successfully
   - Checks all targets and features

4. **Tests** (`cargo test`)
   - Runs all tests to prevent regressions
   - Can be skipped with `SKIP_TESTS=1 git commit`

5. **Debugging Code Detection**
   - Warns about `println!`, `dbg!`, `eprintln!`
   - Warns about `todo!()`, `unimplemented!()`

6. **Error Handling Check**
   - Warns about `unwrap()` and `expect()` in new code
   - Encourages proper error handling

**Example Output:**
```
🔍 Running pre-commit checks...

📝 Checking code formatting...
✓ Code formatting is correct

🔎 Running clippy linter...
✓ No clippy warnings

🔨 Checking if code compiles...
✓ Code compiles successfully

🧪 Running tests...
✓ All tests passed

🐛 Checking for debugging code...
✓ No debugging code found

⚠️  Checking for unwrap/expect...
✓ No unwrap/expect in new code

✅ All pre-commit checks passed!
   Proceeding with commit...
```

### commit-msg

Enforces conventional commit message format:

**Format:**
```
type(scope): subject

body (optional)

footer (optional)
```

**Valid Types:**
- `feat` - New feature
- `fix` - Bug fix
- `docs` - Documentation changes
- `style` - Formatting, missing semicolons, etc
- `refactor` - Code restructuring
- `perf` - Performance improvement
- `test` - Adding tests
- `build` - Build system changes
- `ci` - CI/CD changes
- `chore` - Maintenance tasks

**Examples:**
```bash
git commit -m "feat(protocol): implement device discovery"
git commit -m "fix(applet): correct popup positioning"
git commit -m "docs(readme): add installation instructions"
git commit -m "refactor(daemon): restructure plugin loading"
git commit -m "test(protocol): add packet serialization tests"
```

**Invalid Examples:**
```bash
git commit -m "Added new feature"           # ❌ No type
git commit -m "fix: fixed the bug"          # ❌ No scope
git commit -m "update: updating code"       # ❌ Invalid type
git commit -m "feat(protocol) add feature"  # ❌ Missing colon
```

## Bypassing Hooks

While **not recommended**, you can bypass hooks when necessary:

```bash
# Bypass all hooks
git commit --no-verify

# Skip just tests (hooks will still run other checks)
SKIP_TESTS=1 git commit
```

**When to bypass:**
- Emergency hotfixes (but fix properly after!)
- Work-in-progress commits on feature branches
- Rebasing/squashing commits

**Never bypass for:**
- Commits to main branch
- Pull requests
- Release commits

## Testing Hooks

Test hooks without committing:

```bash
# Test both hooks
just test-hooks

# Test pre-commit only
bash hooks/pre-commit

# Test commit-msg only
echo "feat(test): test message" > /tmp/msg
bash hooks/commit-msg /tmp/msg
rm /tmp/msg
```

## Environment Variables

### SKIP_TESTS
Skip running tests in pre-commit hook:
```bash
SKIP_TESTS=1 git commit -m "feat(protocol): add packet type"
```

### RUST_LOG
Control log level during hook execution:
```bash
RUST_LOG=debug git commit
```

## Uninstalling Hooks

```bash
just uninstall-hooks
```

Or manually:
```bash
rm .git/hooks/pre-commit
rm .git/hooks/commit-msg
```

## Performance

### Pre-commit Hook Timing

Typical execution times:
- **Code formatting**: <1s
- **Clippy linting**: 5-15s (cached: <2s)
- **Compilation check**: 10-30s (cached: <5s)
- **Tests**: 5-60s depending on test count

**Total**: ~20-60s for first run, ~10-20s with cache

### Speeding Up Hooks

1. **Use sccache** for compilation caching:
   ```bash
   cargo install sccache
   export RUSTC_WRAPPER=sccache
   ```

2. **Skip tests** when appropriate:
   ```bash
   SKIP_TESTS=1 git commit
   ```

3. **Use cargo check** instead of full build:
   - Already done by default in hooks

4. **Keep dependencies up to date**:
   ```bash
   cargo update
   ```

## Troubleshooting

### Hook not executing

**Problem**: Hook doesn't run at all

**Solution**:
```bash
# Reinstall hooks
just install-hooks

# Verify permissions
ls -la .git/hooks/pre-commit
ls -la .git/hooks/commit-msg

# Should show -rwxr-xr-x (executable)
```

### Hook fails immediately

**Problem**: Hook exits with error right away

**Solution**:
```bash
# Check bash is available
which bash

# Test hook manually
bash hooks/pre-commit

# Check for syntax errors
bash -n hooks/pre-commit
```

### Clippy or rustfmt not found

**Problem**: Hook can't find cargo tools

**Solution**:
```bash
# Install components
rustup component add rustfmt clippy

# Or run setup
just setup
```

### Tests fail in hook but pass manually

**Problem**: Tests pass with `cargo test` but fail in hook

**Solution**:
- Check for environment differences
- Ensure database/network resources are available
- Check file permissions

### Formatting keeps changing

**Problem**: Hook keeps reformatting the same files

**Solution**:
```bash
# Format manually and commit
cargo fmt --all
git add .
git commit
```

## Best Practices

1. **Run checks before committing**:
   ```bash
   just check  # Runs fmt + lint + test
   ```

2. **Keep commits small**:
   - Hooks run faster on fewer changed files
   - Easier to review

3. **Write good commit messages**:
   - Follow conventional commits format
   - Explain "why" not "what"
   - Reference issues

4. **Don't bypass hooks**:
   - Fix issues instead of bypassing
   - Hooks prevent bugs from reaching main

5. **Update hooks regularly**:
   ```bash
   git pull  # Gets hook updates
   just install-hooks  # Reinstalls
   ```

## Contributing to Hooks

When modifying hooks:

1. **Test thoroughly**:
   ```bash
   bash hooks/pre-commit
   bash hooks/commit-msg /tmp/test-msg
   ```

2. **Document changes**:
   - Update this README
   - Update justfile comments

3. **Keep fast**:
   - Use caching where possible
   - Provide skip options for slow checks

4. **Provide clear output**:
   - Use colors for readability
   - Show progress indicators
   - Give actionable error messages

## Related Documentation

- [ACCEPTANCE_CRITERIA.md](../ACCEPTANCE_CRITERIA.md) - Quality standards enforced by hooks
- [CONTRIBUTING.md](../CONTRIBUTING.md) - Development workflow
- [justfile](../justfile) - Build commands including hook management

---

**Note**: These hooks are designed to catch issues early and maintain code quality. They enforce the standards defined in ACCEPTANCE_CRITERIA.md.
