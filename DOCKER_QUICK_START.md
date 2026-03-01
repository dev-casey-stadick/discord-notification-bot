# Docker Quick Start

Get the bot running in Docker in 60 seconds.

## Prerequisites
- Docker installed: [docker.com/get-docker](https://www.docker.com/get-docker)
- Your Discord bot token ready

## Step 1: Create `.env` file

```bash
cp .env .env
```

Edit `.env` with your settings:
```env
DISCORD_TOKEN=your_bot_token_here
NOTIFICATION_CHANNEL_ID=123456789
NOTIFY_USER_ID=987654321
TRACKED_VOICE_CHANNELS=111111111,222222222
```

## Step 2: Start the bot

```bash
docker-compose up -d
```

Done! The bot is running.

## Step 3: Check it's working

```bash
docker-compose logs -f
```

You should see:
```
✅ Bot logged in as YourBot#0000
📡 Listening for voice channel events...
```

## Common Commands

```bash
# View logs
docker-compose logs -f

# Stop the bot
docker-compose down

# Restart the bot
docker-compose restart

# View status
docker-compose ps

# Rebuild (after code changes)
docker-compose up -d --build
```

## That's it!

Join a voice channel in Discord and you should see events logged immediately.

---

## Detailed Guide

See [DOCKER.md](DOCKER.md) for:
- Manual Docker commands
- Troubleshooting
- Production deployment
- Environment variables
- Persistence and volumes
