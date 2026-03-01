# Running Discord Bot in Docker

This guide shows how to run the Discord notifications bot in a Docker container.

## Quick Start (Recommended)

### 1. Create `.env` file

```bash
cp .env .env
# Edit .env with your settings:
# DISCORD_TOKEN=your_bot_token
# NOTIFICATION_CHANNEL_ID=...
# etc.
```

### 2. Run with Docker Compose

```bash
docker-compose up -d
```

That's it! The bot will start and run in the background.

### 3. Check if it's running

```bash
# View logs
docker-compose logs -f

# Stop the bot
docker-compose down

# Restart the bot
docker-compose restart
```

---

## Manual Docker Build & Run

### Build the image

```bash
docker build -t discord-notifications-bot .
```

### Run the container

```bash
docker run -d \
  --name discord-bot \
  --restart unless-stopped \
  -e DISCORD_TOKEN=your_token \
  -e NOTIFICATION_CHANNEL_ID=123456789 \
  -e NOTIFY_USER_ID=987654321 \
  -e TRACKED_VOICE_CHANNELS=111111111,222222222 \
  -v $(pwd)/logs:/app/logs \
  discord-notifications-bot
```

### Useful Commands

```bash
# View logs
docker logs -f discord-bot

# Stop the bot
docker stop discord-bot

# Restart the bot
docker restart discord-bot

# Remove the container
docker rm discord-bot

# View container info
docker ps
docker inspect discord-bot
```

---

## Docker Compose (Recommended)

Use `docker-compose.yml` for easier management. It:
- Builds the image automatically
- Reads from `.env` file
- Auto-restarts on crash
- Preserves logs
- Easy start/stop/restart

### Commands

```bash
# Start the bot (build if needed)
docker-compose up -d

# View logs in real-time
docker-compose logs -f

# Stop the bot
docker-compose down

# Restart the bot
docker-compose restart

# Rebuild the image
docker-compose up -d --build

# View status
docker-compose ps
```

---

## Environment Variables

Set these in `.env` or pass with `-e` flag:

```bash
DISCORD_TOKEN=your_bot_token
NOTIFICATION_CHANNEL_ID=channel_id      # Optional
NOTIFY_USER_ID=user_id                  # Optional
TRACKED_GUILD_IDS=guild1,guild2         # Optional
TRACKED_VOICE_CHANNELS=channel1,channel2 # Optional
```

---

## Logs

### View logs
```bash
# Docker Compose
docker-compose logs -f discord-bot

# Manual Docker
docker logs -f discord-bot

# Last 50 lines
docker logs --tail 50 discord-bot
```

### Persistent logs
Logs are saved to `./logs/` directory (mounted volume in docker-compose).

---

## Troubleshooting

### Bot won't start
```bash
# Check logs
docker-compose logs discord-bot

# Rebuild image
docker-compose up -d --build

# Check if token is set
docker-compose config | grep DISCORD_TOKEN
```

### Can't send notifications
- Check that channel IDs are correct
- Make sure bot has permissions in the channel
- Verify user ID is correct

### Container exits immediately
```bash
# View error
docker logs discord-bot

# Common issues:
# - Missing DISCORD_TOKEN in .env
# - Token is invalid
# - Network issues
```

### Remove everything and start fresh
```bash
# Stop container
docker-compose down

# Remove image
docker rmi discord-notifications-bot

# Clean up
docker system prune -a

# Start fresh
docker-compose up -d --build
```

---

## Production Deployment

### Using Docker on a VPS

1. **SSH into your VPS**
   ```bash
   ssh user@your-vps-ip
   ```

2. **Clone/upload the project**
   ```bash
   git clone <your-repo> discord-notifications
   cd discord-notifications
   ```

3. **Create `.env` with your tokens**
   ```bash
   nano .env
   # Add your settings, then Ctrl+X to save
   ```

4. **Start the bot**
   ```bash
   docker-compose up -d
   ```

5. **Check it's running**
   ```bash
   docker-compose ps
   docker-compose logs
   ```

### Keep bot running on reboot
Docker Compose with `restart: unless-stopped` will auto-restart the bot when the container/VPS reboots.

### Monitor the bot
```bash
# Check status
docker-compose ps

# View logs
docker-compose logs -f --tail 100

# Alert on errors (optional)
docker-compose logs | grep -i error
```

---

## Docker Networking

If running multiple services, use docker-compose networks:

```yaml
# In docker-compose.yml
services:
  discord-bot:
    networks:
      - bot-network

networks:
  bot-network:
    driver: bridge
```

---

## Size & Performance

- **Image size**: ~150-200MB (Node.js Alpine)
- **Memory usage**: ~50-100MB
- **CPU usage**: Minimal (event-driven, not polling)

To reduce image size further, use multi-stage builds or Alpine Linux distroless images.

---

## Security Notes

- ✅ Never commit `.env` to git
- ✅ Use strong bot tokens
- ✅ Rotate tokens if exposed
- ✅ Don't share container logs with sensitive info
- ✅ Use `--restart unless-stopped` not `always` to prevent restart loops

---

## Examples

### Example 1: Simple setup
```bash
docker-compose up -d
# Just needs DISCORD_TOKEN in .env
```

### Example 2: Monitor specific channels
```env
DISCORD_TOKEN=your_token
NOTIFICATION_CHANNEL_ID=999999999
TRACKED_VOICE_CHANNELS=111111111,222222222
```

### Example 3: Multiple environments
```bash
# Development
docker-compose -f docker-compose.yml -f docker-compose.dev.yml up

# Production
docker-compose -f docker-compose.yml -f docker-compose.prod.yml up
```
