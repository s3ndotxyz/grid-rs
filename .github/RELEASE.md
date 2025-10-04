# Automated Package Release Guide

This repository now has automated CI/CD for publishing Rust packages to crates.io.

## Overview

The workflow automatically detects version changes in `Cargo.toml` files and publishes new versions to crates.io when they differ from what's currently published.

## Setup Instructions

### 1. Get a Crates.io API Token

1. Log in to [crates.io](https://crates.io/)
2. Go to [Account Settings > API Tokens](https://crates.io/settings/tokens)
3. Click "New Token"
4. Give it a descriptive name (e.g., "grid-rs-ci")
5. Grant "publish-update" scope
6. Copy the generated token

### 2. Add Token to GitHub Secrets

1. Go to your GitHub repository
2. Navigate to **Settings** → **Secrets and variables** → **Actions**
3. Click **New repository secret**
4. Name: `CARGO_REGISTRY_TOKEN`
5. Value: Paste your crates.io token
6. Click **Add secret**

## How to Release a New Version

### For grid-rs-macros

1. Edit `macros/Cargo.toml`
2. Update the version number:
   ```toml
   [package]
   version = "0.1.1"  # Change this
   ```
3. Commit and push to `main`
4. The workflow automatically publishes if the version is new

### For grid-rs

1. Edit `Cargo.toml`
2. Update the version number:
   ```toml
   [package]
   version = "0.1.2"  # Change this
   ```
3. **Important**: If grid-rs depends on a new version of grid-rs-macros, update the dependency too:
   ```toml
   [dependencies]
   grid-rs-macros = "0.1.1"  # Update to match
   ```
4. Commit and push to `main`
5. The workflow automatically publishes if the version is new

## Workflow Behavior

- **Triggers**: Runs on push to `main` when `Cargo.toml` or `macros/Cargo.toml` changes
- **Version Check**: Compares local version with crates.io
- **Publishing Order**: 
  1. Publishes `grid-rs-macros` first (if needed)
  2. Waits 30 seconds for crates.io index update
  3. Publishes `grid-rs` (if needed)
- **GitHub Releases**: Creates a GitHub release with tag for each published package

## Tags Format

- Macros: `grid-rs-macros-v0.1.0`
- SDK: `grid-rs-v0.1.1`

## Manual Workflow Trigger

You can manually run the workflow from the GitHub Actions tab:
1. Go to **Actions** → **Package Release**
2. Click **Run workflow**
3. Select the `main` branch
4. Click **Run workflow**

## Troubleshooting

### "Package version already published"
- The version in `Cargo.toml` already exists on crates.io
- Increment the version number to publish

### "Permission denied" or "Authentication failed"
- Check that `CARGO_REGISTRY_TOKEN` secret is set correctly
- Verify the token has publish permissions on crates.io
- Ensure you're a publisher for the package on crates.io

### "Dependency version not found"
- If grid-rs depends on a newer grid-rs-macros version
- The workflow publishes macros first and waits 30 seconds
- If still failing, try manual workflow trigger after a few minutes

## Best Practices

1. **Update versions together**: When changing macros, update both:
   - `macros/Cargo.toml` version
   - `Cargo.toml` dependency on grid-rs-macros

2. **Follow semver**: Use semantic versioning:
   - MAJOR version for incompatible API changes
   - MINOR version for new backward-compatible functionality
   - PATCH version for backward-compatible bug fixes

3. **Test before release**: Build and test locally before pushing version changes

4. **One version at a time**: Avoid changing multiple versions in one commit to make rollback easier
