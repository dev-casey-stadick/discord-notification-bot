# Releasing New Versions

## Automatic Builds & Releases

This project uses GitHub Actions to automatically build and release binaries for multiple platforms.

### How to Create a Release

1. **Tag a new version:**
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```

2. **GitHub Actions will automatically:**
   - ✅ Build for Linux x86-64
   - ✅ Build for Linux ARM64 (Raspberry Pi)
   - ✅ Build for macOS x86-64
   - ✅ Build for macOS ARM64 (Apple Silicon)
   - ✅ Build for Windows x86-64
   - ✅ Create a GitHub Release with all binaries attached
   - ✅ Add them to your releases page

3. **Download from Releases:**
   - Go to your repo's "Releases" tab
   - Download the binary for your platform
   - Extract and run!

### Version Naming

Use semantic versioning:
- `v1.0.0` - Major version
- `v1.0.1` - Patch version
- `v1.1.0` - Minor version

### Binary Names

After release, you'll have:
- `discord-notifications-rs-linux-x86_64.tar.gz` - Linux 64-bit
- `discord-notifications-rs-linux-arm64.tar.gz` - Linux ARM64
- `discord-notifications-rs-macos-x86_64.tar.gz` - macOS Intel
- `discord-notifications-rs-macos-arm64.tar.gz` - macOS Apple Silicon
- `discord-notifications-rs-windows-x86_64.zip` - Windows

### Deployment Example (Linux)

```bash
# Download
wget https://github.com/YOUR_USERNAME/discord-notifications/releases/download/v1.0.0/discord-notifications-rs-linux-x86_64.tar.gz

# Extract
tar xzf discord-notifications-rs-linux-x86_64.tar.gz

# Set environment
export DISCORD_TOKEN="your_token"
export NOTIFY_USER_ID="123456789"

# Run
./discord-notifications-rs
```

### Cost

**Completely Free! 🎉**

- GitHub Actions: ✅ Free (unlimited for public repos, 2000 min/month for private)
- Release downloads: ✅ Free (unlimited bandwidth)
- Storage: ✅ Free (per-repo releases don't count against storage)

No cost to you for building, hosting, or distributing binaries!

### Continuous Integration

- Every push to `main` triggers a quick Linux build verification
- Only tagged releases (`v*`) trigger full multi-platform builds and releases
- Artifacts are kept for 30 days for debugging

### Manual Testing

Before tagging a release, you can manually trigger a build:
1. Go to "Actions" tab
2. Click "Build & Release Rust Bot"
3. Click "Run workflow"
4. Select your branch
5. It will build and upload artifacts (no release created)
