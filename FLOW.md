# Discord Voice Events - How It Works

## High-Level Flow

```
Discord Server
      ↓
   Someone joins/leaves/moves voice channel
      ↓
Discord sends voiceStateUpdate event to bot
      ↓
Bot receives event (oldState, newState)
      ↓
Bot checks filters:
   ├─ Correct guild? (if TRACKED_GUILD_IDS set)
   ├─ Correct voice channel? (if TRACKED_VOICE_CHANNELS set)
      ↓
Bot detects event type:
   ├─ JOIN: oldChannel=null, newChannel=something
   ├─ LEAVE: oldChannel=something, newChannel=null
   ├─ MOVE: oldChannel≠newChannel
   ├─ MUTE/UNMUTE: same channel, mute flag changed
      ↓
Bot sends notifications:
   ├─ Post to Discord channel (if NOTIFICATION_CHANNEL_ID set)
   ├─ DM to user (if NOTIFY_USER_ID set)
   └─ Log to console
```

## Code Flow (Simplified)

```javascript
// 1. Bot starts and waits
client.login(TOKEN)
console.log("Listening for voice channel events...")

// 2. Event happens in Discord
// User joins #general voice channel

// 3. Bot receives event
client.on(Events.VoiceStateUpdate, (oldState, newState) => {

  // 4. Extract data
  const member = newState.member              // "alice"
  const newChannel = newState.channel         // "general"

  // 5. Apply filters
  if (TRACKED_VOICE_CHANNELS && !includesChannel) return  // Skip if wrong channel

  // 6. Detect type
  if (!oldState.channel && newState.channel) {  // ← This is a JOIN
    console.log("alice joined general")

    // 7. Send notifications
    notifyChannelEmbed(...)  // Post to #voice-logs
    notifyUserDM(...)        // DM user
  }
})
```

## Example: With Filtering

### Setup
```
.env:
NOTIFICATION_CHANNEL_ID=999999999
NOTIFY_USER_ID=888888888
TRACKED_VOICE_CHANNELS=111111111,222222222
```

This means:
- Only track voice channels with IDs 111111111 and 222222222
- Ignore events in all other channels
- When events happen in tracked channels, post to channel 999999999 and DM user 888888888

### What Happens

```
Scenario 1: Someone joins #general (ID: 111111111)
  ✅ IN TRACKED CHANNELS → Process event → Post embed + DM

Scenario 2: Someone joins #afk (ID: 333333333)
  ❌ NOT IN TRACKED CHANNELS → Skip event, do nothing

Scenario 3: Someone joins #gaming (ID: 222222222)
  ✅ IN TRACKED CHANNELS → Process event → Post embed + DM
```

## Configuration Combinations

| Config | Behavior |
|--------|----------|
| No filters | Track all voice channels in all servers |
| `TRACKED_GUILD_IDS=X` | Track all voice channels only in server X |
| `TRACKED_VOICE_CHANNELS=A,B` | Track only voice channels A and B in all servers |
| Both above | Track only channels A,B in server X |
| No notification settings | Just log to console (no Discord posts/DMs) |

## Getting IDs

### Discord Channel ID
1. Right-click the voice channel in Discord
2. Select "Copy Channel ID"
3. Paste into `.env`

Example:
```
TRACKED_VOICE_CHANNELS=912345678901234567
```

### How to Tell if It's Working

Run the bot and check console:
```
✅ Bot logged in as MyBot#0000
📡 Listening for voice channel events...
📢 Event notifications → Channel 999999999
📧 DM notifications → User 888888888

📍 [JOIN] alice joined #general
   Users in channel: 3
```

If you join a tracked channel, you should see the event logged immediately!
