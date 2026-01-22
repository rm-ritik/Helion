# GitHub Actions Workflows

This directory contains automated workflows for the Helion project.

## Workflows

### 1. CI - Tests and Examples (`ci.yml`)

**Purpose:** Comprehensive testing of core library, Python bindings, and all examples.

**Triggers:**
- Every push to `main` branch
- Every pull request to `main` branch

**Jobs:**

1. **Core Library Tests**
   - Runs all Rust tests with `cargo test --all-features`
   - Validates documentation builds
   - Caches dependencies for faster builds

2. **Python Bindings & Examples**
   - Tests on Python 3.10, 3.11, and 3.12
   - Builds Python bindings with maturin
   - Runs all 5 Python examples
   - Validates README.md code example works

3. **Integration Check**
   - Final verification that all tests passed
   - Only runs if all previous jobs succeed

**Branch Protection:**

To prevent broken code from reaching `main`, enable branch protection:

1. Go to: [Repository Settings → Branches](https://github.com/rm-ritik/Helion/settings/branches)
2. Add rule for `main` branch
3. Enable:
   - ✅ Require status checks to pass before merging
   - ✅ Require branches to be up to date before merging
   - Select required checks:
     - `Core Library Tests`
     - `Python Bindings & Examples (3.10)`
     - `Python Bindings & Examples (3.11)`
     - `Python Bindings & Examples (3.12)`
     - `Integration Check`
   - ✅ Include administrators (optional but recommended)

**Result:** Pull requests cannot be merged if tests fail. Direct pushes to `main` are allowed but CI will show failure status.

### 2. Deploy Documentation (`docs.yml`)

**Purpose:** Automatically build and deploy Rust documentation to GitHub Pages.

**Triggers:**
- Every push to `main` branch
- Manual trigger via Actions UI

**Jobs:**
1. Builds Rust documentation with `cargo doc`
2. Deploys to GitHub Pages

See [documentation setup instructions](../../DOCS_SETUP.md).

## Preventing Broken Merges

The CI workflow is designed to catch issues before they reach `main`:

### For Pull Requests ✅
- All tests must pass before merge is allowed (with branch protection enabled)
- Developers get immediate feedback on their changes
- Main branch stays stable

### For Direct Pushes ⚠️
- Tests run after push (cannot be blocked)
- Workflow will fail and notify via GitHub
- Team should manually revert if needed: `git revert <commit-hash>`

**Best Practice:** Always use pull requests, even for small changes. Enable branch protection to enforce this.

## Local Testing

Before pushing, run tests locally:

```bash
# Core tests
cd core
cargo test --all-features

# Python examples
cd bindings/python
./build.sh
cd ../../examples/python
python scatter_basic.py
python scatter_million.py
python scatter_colors.py
python scatter_custom_ranges.py

# README example
python -c "
import helion
import numpy as np
x = np.random.rand(1_000_000)
y = np.random.rand(1_000_000)
plot = helion.scatter(x, y, color='#FF5733')
print('✅ Works!')
"
```

## Cache Strategy

The CI uses GitHub Actions cache to speed up builds:
- Cargo registry and git dependencies
- Compiled Rust artifacts
- Separate caches per Python version

This reduces build time from ~5 minutes to ~2 minutes on subsequent runs.

## Troubleshooting

### Python Example Fails in CI but Works Locally
- Check Python version (CI tests 3.10, 3.11, 3.12)
- Verify dependencies in `bindings/python/Cargo.toml`
- Check for hardcoded paths or environment assumptions

### Core Tests Fail
- Review test output in Actions tab
- Run locally: `cd core && cargo test --all-features --verbose`
- Check for platform-specific issues (CI runs on Ubuntu)

### Cache Issues
- Clear cache: Go to Actions → Caches → Delete old caches
- Cache keys are based on `Cargo.lock` hash
