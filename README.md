# Discord Channel Notifications

A lightweight Discord bot that listens for and logs voice channel join/leave events.

## Features

- 📍 Detects when users **join** voice channels
- 📍 Detects when users **leave** voice channels
- 📍 Detects when users **move** between channels
- 🔇 Detects mute/unmute status changes
- 🔊 Detects deafen/undeafen status changes
- No continuous polling needed - events are received in real-time

## Setup

### 1. Create a Discord Bot

See **[SETUP_TOKENS.md](SETUP_TOKENS.md)** for detailed step-by-step instructions including:
- Creating bot in Discord Developer Portal
- Getting your bot token
- Inviting bot to your server
- Setting permissions

**Quick version:**
1. Go to [Discord Developer Portal](https://discord.com/developers/applications)
2. Click "New Application"
3. Go to "Bot" section and click "Add Bot"
4. Copy your bot token
5. Go to "OAuth2" > "URL Generator"
6. Select scopes: `bot`
7. Select permissions: `View Channels`, `Connect`, `Speak`
8. Copy the generated URL and open it to invite the bot to your server

### 2. Install Dependencies

```bash
npm install
```

### 3. Create `.env` file

```bash
cp .env .env
# Edit .env and paste your DISCORD_TOKEN
```

See **[SETUP_TOKENS.md](SETUP_TOKENS.md)** for detailed `.env` configuration options.

### 4. Run the Bot

**Simple (Development):**
```bash
npm start
```

**Production with Auto-Restart (PM2):**
```bash
npm install -g pm2
npm run pm2:start

# View logs
npm run pm2:logs

# Stop/restart
npm run pm2:stop
npm run pm2:restart
```

You should see:
```
✅ Bot logged in as YourBot#0000
📡 Listening for voice channel events...
```

## 24/7 Deployment

The bot needs to run continuously.

**See [SETUP_TOKENS.md](SETUP_TOKENS.md) for complete deployment guides including token setup for each platform.**

### Deployment Options:

### Option 1: Docker Compose (Recommended) 🐳
Easiest and most reliable:
```bash
docker-compose up -d
docker-compose logs -f  # View logs
```
See [DOCKER.md](DOCKER.md) for detailed guide.

### Option 2: PM2 (Local Server)
Best for always-on computers or VPS:
```bash
npm install -g pm2
npm run pm2:start
pm2 startup  # Auto-start on reboot
pm2 save     # Save current process list
```

### Option 3: Cloud Hosting
Deploy to services like:
- **Railway**: Connect GitHub repo, set DISCORD_TOKEN secret, auto-deploys
- **Render.com**: Similar to Railway, free tier available
- **DigitalOcean App Platform**: $5-12/month
- **Fly.io**: $2.50/month with Docker
- **Heroku alternatives**: Replit, Railway, Render

## Notification Mechanisms

Choose how you want to be notified:

### 📢 **Channel Messages** (Recommended)
Post event notifications to a Discord channel (like #voice-logs):
```
.env: NOTIFICATION_CHANNEL_ID=123456789
```
Shows rich embeds with user, channel, and user count.

### 📧 **Direct Messages**
Send DM to a specific person:
```
.env: NOTIFY_USER_ID=987654321
```
Gets a DM: "🟢 **alice** joined **#general**"

### 💬 **Console Only** (Default)
Just logs to console - no notifications. Perfect for testing.

### 🔌 **SMS (Twilio)** - Advanced
Text message users - requires Twilio account (~$0.01-0.05/SMS).
Can be added to `notifications.js` if needed.

### 🔗 **Webhooks**
Post to external services (logging, analytics, etc).
See `notifications.js` for `notifyWebhook()` function.

## How It Works

The bot uses Discord.js's `voiceStateUpdate` event to detect voice channel changes:

- **oldState**: User's voice state before the change
- **newState**: User's voice state after the change

By comparing `oldState.channel` and `newState.channel`, we can detect:
- **Join**: `oldState.channel === null && newState.channel !== null`
- **Leave**: `oldState.channel !== null && newState.channel === null`
- **Move**: Both channels exist but are different

## Event Output Examples

### Console Output (Default Bot)
```
📍 [JOIN] alice joined #general
   Guild: My Server
   Channel Users: 3
```

### Channel Notifications (With NOTIFICATION_CHANNEL_ID)
Posts rich embeds to your designated channel:
```
📍 JOIN
User: alice
Channel: general
Users in Channel: 3
```

### DM Notifications (With NOTIFY_USER_ID)
```
🟢 **alice** joined **#general**
🔴 **bob** left **#general**
🔄 **charlie** moved to **#afk**
```

## Setup Notifications

### Option 1: Post to a Channel

1. Create a channel (e.g., #voice-logs) or use existing one
2. Right-click the channel → Copy Channel ID
3. Add to `.env`:
   ```
   NOTIFICATION_CHANNEL_ID=YOUR_CHANNEL_ID
   ```
4. Run `npm start` or `bot-with-notifications.js`

### Option 2: Get DMs

1. Right-click your Discord username → Copy User ID
2. Add to `.env`:
   ```
   NOTIFY_USER_ID=YOUR_USER_ID
   ```
3. Run with notifications enabled

### Option 3: Both
Enable both channel posts AND DMs in `.env`:
```
NOTIFICATION_CHANNEL_ID=123456789
NOTIFY_USER_ID=987654321
```

## Filtering by Voice Channels

You can track only **specific voice channels** instead of all channels:

1. Right-click the voice channel in Discord
2. Select "Copy Channel ID"
3. Add to `.env`:
   ```
   TRACKED_VOICE_CHANNELS=111111111,222222222,333333333
   ```

Now the bot will **only** process events in those channels and ignore all others.

**Examples:**
- `TRACKED_VOICE_CHANNELS=123456` - Only track #general voice
- `TRACKED_VOICE_CHANNELS=123,456,789` - Track 3 specific channels
- Leave blank or omit = track all channels (default)

## Using the Enhanced Bot

Use `bot-with-notifications.js` instead of `bot.js`:
```bash
# Edit package.json "start" script to use:
node bot-with-notifications.js

# Or run directly:
node bot-with-notifications.js
```

This includes:
- Notification mechanisms from `.env`
- Guild filtering (TRACKED_GUILD_IDS)
- **Voice channel filtering (TRACKED_VOICE_CHANNELS)** ← NEW!

## Next Steps

You can extend this bot to:
- Save events to a database
- Send notifications to other Discord channels
- Create analytics dashboards
- Trigger webhooks for external services
- Export event logs

## Resources

- [Discord.js Documentation](https://discord.js.org)
- [Discord Developer Documentation](https://discord.com/developers/docs)
- [VoiceState Documentation](https://discord.js.org/docs/packages/discord.js/main/VoiceState:Class)
