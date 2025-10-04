# Release Automation

This directory contains GitHub Actions workflows for automated package releases.

## release.yml

Automatically publishes packages to crates.io when version numbers change in `Cargo.toml` files.

### How it works

1. **Triggers**: The workflow runs on:
   - Push to `main` branch when `Cargo.toml` or `macros/Cargo.toml` changes
   - Manual workflow dispatch

2. **Version Detection**: 
   - Reads the current version from each `Cargo.toml`
   - Checks if that version exists on crates.io
   - If version doesn't exist, publishes the package

3. **Publishing Order**:
   - First publishes `grid-rs-macros` (dependency)
   - Waits 30 seconds for crates.io propagation
   - Then publishes `grid-rs` (main SDK)

4. **GitHub Releases**:
   - Creates a GitHub release for each published package
   - Tags are in format: `{package-name}-v{version}`

### Setup Required

Add the following secret to your GitHub repository:
- `CARGO_REGISTRY_TOKEN`: Your crates.io API token

To get a token:
1. Visit https://crates.io/settings/tokens
2. Generate a new token with publish permissions
3. Add it to GitHub repository secrets

### Usage

1. **Update version** in `Cargo.toml` or `macros/Cargo.toml`
2. **Commit and push** to `main` branch
3. **Workflow automatically runs** and publishes if version is new

### Manual Trigger

You can also manually trigger the workflow from the Actions tab in GitHub.
