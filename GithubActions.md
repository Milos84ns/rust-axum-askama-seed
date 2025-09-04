Here is a robust GitHub Actions workflow for your Rust project, following best practices for security, caching, cross-platform builds, and artifact distribution. Each step is documented for clarity.

Save this as `.github/workflows/ci-cd.yml`:

```yaml
name: CI/CD

on:
  push:
    branches: [main]
  release:
    types: [published]

jobs:
  build-test:
    name: Build & Test (${{ matrix.os }})
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    steps:
      # Step 1: Securely check out the repository
      - name: Checkout code
        uses: actions/checkout@v4

      # Step 2: Set up Rust toolchain (stable, locked to latest for reproducibility)
      - name: Set up Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
          profile: minimal

      # Step 3: Cache cargo registry and build artifacts for faster builds
      - name: Cache cargo registry
        uses: actions/cache@v4
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
          restore-keys: |
            ${{ runner.os }}-cargo-

      # Step 4: Build in release mode
      - name: Build (release)
        run: cargo build --release

      # Step 5: Run all tests (fail workflow if any test fails)
      - name: Run tests
        run: cargo test --all --release --locked

      # Step 6: Upload build artifacts for later jobs (only on release)
      - name: Upload release binary
        if: github.event_name == 'release'
        uses: actions/upload-artifact@v4
        with:
          name: ${{ runner.os }}-binary
          path: |
            target/release/${{ github.event.repository.name }}*
          if-no-files-found: error

  upload-release-assets:
    name: Upload Release Assets
    needs: build-test
    if: github.event_name == 'release'
    runs-on: ubuntu-latest
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    steps:
      # Step 1: Download the built binaries from previous job
      - name: Download binary artifact
        uses: actions/download-artifact@v4
        with:
          name: ${{ matrix.os }}-binary
          path: binaries/${{ matrix.os }}

      # Step 2: Upload binaries to GitHub Release as assets
      - name: Upload to GitHub Release
        uses: softprops/action-gh-release@v2
        with:
          files: binaries/${{ matrix.os }}/*
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

**Key points:**
- Triggers on push to `main` and on new releases.
- Builds and tests on Linux, Windows, and macOS.
- Uses caching for dependencies and build outputs.
- On release, uploads platform-specific binaries as assets to the GitHub Release.
- Uses secure, maintained actions and minimal permissions.
- Each step is clearly documented for maintainability.