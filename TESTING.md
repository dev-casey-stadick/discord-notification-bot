# Testing Guide

## Running Tests

### Unit Tests
```bash
cd rust
cargo test
```

**Output:**
```
running 8 tests
test tests::test_config_parsing_empty_env ... ok
test tests::test_config_should_track_guild_empty ... ok
test tests::test_config_should_track_guild_filtered ... ok
test tests::test_config_should_track_channel_empty ... ok
test tests::test_config_should_track_channel_filtered ... ok
test tests::test_config_should_notify ... ok
test tests::test_config_equality ... ok
test tests::test_channel_states_tracking ... ok

test result: ok. 8 passed
```

### Release Build Tests
```bash
cargo test --release
```

### Watch Mode (auto-run on changes)
```bash
cargo watch -x test
```
*(Requires `cargo-watch`: `cargo install cargo-watch`)*

---

## Test Coverage

### Config Tests ✅
- **Empty environment variables** - Handles missing config gracefully
- **Guild filtering** - Correctly filters tracked guilds (empty = all, filtered = specific)
- **Channel filtering** - Correctly filters tracked channels (empty = all, filtered = specific)
- **Notification settings** - Detects if notifications are enabled
- **Config equality** - Verifies config comparison works

### State Management Tests ✅
- **Channel state tracking** - Correctly tracks member counts by channel
- **Async mutex locks** - Verifies concurrent access to state

---

## Integration Testing (Manual)

Since the bot talks to Discord's API, integration tests require:
1. A real Discord token
2. A test Discord server
3. Test user accounts

### Local Testing Steps

1. **Set up test environment:**
   ```bash
   export DISCORD_TOKEN="your_test_token"
   export NOTIFY_USER_ID="your_user_id"
   export TRACKED_VOICE_CHANNELS="test_channel_id_1,test_channel_id_2"
   ```

2. **Run the bot:**
   ```bash
   cd rust
   cargo run --release
   ```

3. **Test scenarios:**
   - ✅ Join a tracked voice channel → Should send DM
   - ✅ Leave the voice channel → Should send "channel empty" DM
   - ✅ Multiple users joining/leaving → Should track counts correctly
   - ✅ Untracked channels → Should NOT send notifications
   - ✅ Network failures → Should retry with exponential backoff

---

## What Gets Tested

### Unit Tests (Automated)
- ✅ Configuration parsing
- ✅ Guild/channel filtering logic
- ✅ Notification enablement checks
- ✅ State mutation and tracking

### NOT Tested (Would need external Discord API)
- ❌ Voice state event handling (requires Discord gateway)
- ❌ DM sending (requires Discord HTTP API)
- ❌ Guild/channel name fetching (requires Discord API)
- ❌ User name fetching (requires Discord API)
- ❌ Network retry logic (complex to mock)

---

## Adding New Tests

### Example: Test a new filtering feature

```rust
#[test]
fn test_config_combined_filtering() {
    let config = Config {
        notify_user_ids: vec![123],
        tracked_guild_ids: vec![100, 200],      // Only these guilds
        tracked_voice_channel_ids: vec![1001],  // Only this channel
    };

    // Should allow this guild + channel combo
    assert!(config.should_track_guild(100));
    assert!(config.should_track_channel(1001));

    // Should reject wrong guild
    assert!(!config.should_track_guild(300));

    // Should reject wrong channel
    assert!(!config.should_track_channel(1002));
}
```

### Run specific test:
```bash
cargo test test_config_combined_filtering
```

---

## CI/CD Testing

GitHub Actions (`.github/workflows/build-release.yml`) runs:
1. ✅ Unit tests on every push
2. ✅ Build verification (Linux only)
3. ✅ Release builds for 5 platforms on tagged releases

### View test results:
1. Go to GitHub repo
2. Click "Actions" tab
3. Click the workflow run
4. Scroll to "Build & Release Rust Bot" section

---

## Debugging Failed Tests

### Run with verbose output:
```bash
cargo test -- --nocapture
```

### Run single test with output:
```bash
cargo test test_name -- --nocapture --test-threads=1
```

### Check test coverage (estimate):
```bash
# Currently ~60-70% coverage on testable logic
# (Discord API interaction can't be tested without mocks)
cargo tarpaulin  # Requires: cargo install cargo-tarpaulin
```

---

## Test Philosophy

**Unit Tests:** Focus on **logic that doesn't depend on external APIs**
- Config parsing ✅
- State management ✅
- Filtering logic ✅

**Integration Tests:** Would test **actual Discord API interaction**
- Requires Discord token & test server
- Slow (network calls)
- Fragile (API changes)
- Use sparingly

**Manual Testing:** **Real-world scenarios**
- Join/leave voice channels
- Network failures
- Multiple simultaneous events

---

## Best Practices

1. **Run tests before commits:**
   ```bash
   cargo test && cargo build --release
   ```

2. **Use meaningful test names:**
   ```rust
   #[test]
   fn test_config_should_track_guild_when_list_is_empty() { }
   ```

3. **Test edge cases:**
   - Empty config
   - Duplicate IDs
   - Invalid input formats

4. **Keep tests focused:**
   - One assertion per test (or related assertions)
   - Test one behavior at a time

---

## Summary

| Type | Command | Time | Coverage |
|------|---------|------|----------|
| Unit | `cargo test` | ~1s | 60-70% |
| Release | `cargo test --release` | ~3s | 60-70% |
| Manual | Run bot locally | varies | 100% |
| CI/CD | GitHub Actions | ~5min | 60-70% |
