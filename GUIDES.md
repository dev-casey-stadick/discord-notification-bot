# Documentation Guides

Complete documentation for the Discord Notifications bot.

## 📚 Main Guides

### **[README.md](README.md)** - Start Here
- Features overview
- Quick setup instructions
- Notification mechanisms
- Filtering options
- Basic usage

### **[SETUP_TOKENS.md](SETUP_TOKENS.md)** ⭐ TOKEN & DEPLOYMENT SETUP
Complete guide for:
- Creating Discord bot in Developer Portal
- Getting your bot token
- Configuring tokens for different platforms
- **Railway deployment** (Recommended, free)
- Render, DigitalOcean setup
- Get Channel/User IDs
- Troubleshooting token issues

**👉 Start here if deploying to production!**

### **[DOCKER.md](DOCKER.md)** - Docker Deployment
- Docker image setup
- Docker Compose guide
- Environment variables in Docker
- Production deployment
- Security notes

### **[DOCKER_QUICK_START.md](DOCKER_QUICK_START.md)** - Docker 60-Second Setup
Quick reference for:
- Running with docker-compose
- Common Docker commands
- Checking logs

### **[FLOW.md](FLOW.md)** - How It Works (Technical)
Visual diagrams and explanations of:
- Complete event flow
- Code flow walkthrough
- Filtering logic
- Configuration combinations
- How to tell if it's working

---

## 🚀 Quick Navigation by Task

### I want to get started quickly
→ [README.md](README.md) (Setup section)

### I want to deploy to production (Railway, etc)
→ [SETUP_TOKENS.md](SETUP_TOKENS.md)

### I want to use Docker
→ [DOCKER_QUICK_START.md](DOCKER_QUICK_START.md) (quick) or [DOCKER.md](DOCKER.md) (detailed)

### I need to understand how the bot works
→ [FLOW.md](FLOW.md)

### I'm having issues
→ [SETUP_TOKENS.md](SETUP_TOKENS.md) (Troubleshooting section)

### I want to configure notifications and filtering
→ [README.md](README.md) (Notification Mechanisms & Filtering sections)

---

## 📋 Configuration Checklist

Before deploying, you need:

- [ ] **Discord Bot Token** - From Developer Portal
  - See: [SETUP_TOKENS.md](SETUP_TOKENS.md) - Create Discord Bot section

- [ ] **Channel ID** (Optional) - For notifications
  - See: [SETUP_TOKENS.md](SETUP_TOKENS.md) - Get Channel/User IDs section

- [ ] **User ID** (Optional) - For DM notifications
  - See: [SETUP_TOKENS.md](SETUP_TOKENS.md) - Get Channel/User IDs section

- [ ] **Voice Channel IDs** (Optional) - For filtering
  - See: [SETUP_TOKENS.md](SETUP_TOKENS.md) - Get Channel/User IDs section

- [ ] **Hosting Platform** - Choose where to run
  - See: [SETUP_TOKENS.md](SETUP_TOKENS.md) - Railway Deployment / Other Platforms

---

## 📖 File Structure

```
discord-notifications/
├── README.md                 ← Overview & quick start
├── SETUP_TOKENS.md          ← Token setup & deployment guides ⭐
├── DOCKER.md                ← Docker deployment (detailed)
├── DOCKER_QUICK_START.md    ← Docker quick reference
├── FLOW.md                  ← How it works (technical)
├── GUIDES.md                ← This file
│
├── bot.js                   ← Basic bot (console only)
├── bot-with-notifications.js ← Enhanced bot (recommended)
├── notifications.js         ← Notification functions
│
├── Dockerfile               ← Docker image config
├── docker-compose.yml       ← Docker Compose config
├── .dockerignore           ← Docker build excludes
│
├── package.json            ← Dependencies
├── .env.example            ← Config template
├── .gitignore              ← Git excludes
└── ecosystem.config.js     ← PM2 config
```

---

## 🔑 Key Concepts

### **Environment Variables (.env)**
Configuration values stored locally (not in code):
- `DISCORD_TOKEN` - Your bot's secret token
- `NOTIFICATION_CHANNEL_ID` - Where to post events
- `NOTIFY_USER_ID` - Who to DM
- `TRACKED_VOICE_CHANNELS` - Which channels to monitor

See: [SETUP_TOKENS.md](SETUP_TOKENS.md) - Local Setup section

### **Hosting/Deployment**
Where your bot runs 24/7:
- **Railway** - Easiest (free tier)
- **Docker** - Most flexible
- **DigitalOcean** - Most control
- **Your PC** - For testing

See: [SETUP_TOKENS.md](SETUP_TOKENS.md) - Railway / Other Platforms sections

### **Filtering**
What events to listen to:
- **Guild/Server IDs** - Monitor only specific servers
- **Voice Channel IDs** - Monitor only specific voice channels

See: [README.md](README.md) - Filtering section

### **Notifications**
How you're notified:
- **Discord Channel** - Post to #voice-logs
- **Discord DM** - Direct message user
- **Console** - Just log locally

See: [README.md](README.md) - Notification Mechanisms section

---

## ❓ Frequently Asked Questions

**Q: Where do I get my bot token?**
A: [SETUP_TOKENS.md](SETUP_TOKENS.md) - Get Your Bot Token section

**Q: How do I deploy the bot?**
A: [SETUP_TOKENS.md](SETUP_TOKENS.md) - Choose Railway, Docker, or other platform

**Q: What's the difference between NOTIFICATION_CHANNEL_ID and TRACKED_VOICE_CHANNELS?**
A: [SETUP_TOKENS.md](SETUP_TOKENS.md) or [README.md](README.md) - Notification vs Filtering sections

**Q: How do I get channel/user IDs?**
A: [SETUP_TOKENS.md](SETUP_TOKENS.md) - Get Channel/User IDs section

**Q: My bot won't start, what's wrong?**
A: [SETUP_TOKENS.md](SETUP_TOKENS.md) - Troubleshooting section

**Q: How does the bot work internally?**
A: [FLOW.md](FLOW.md)

---

## 🆘 Support

- Check [SETUP_TOKENS.md](SETUP_TOKENS.md) **Troubleshooting** section first
- Review [FLOW.md](FLOW.md) to understand how it works
- Check console/logs for error messages
- Verify all IDs are correct (right-click to copy in Discord)
- Make sure bot is in your server with correct permissions
- Make sure DISCORD_TOKEN is set and valid

---

## 📝 Document Status

| Document | Topic | Difficulty |
|----------|-------|------------|
| README.md | Overview, quick start | Beginner |
| SETUP_TOKENS.md | Tokens, deployment | Beginner-Intermediate |
| DOCKER.md | Docker, containers | Intermediate |
| DOCKER_QUICK_START.md | Docker reference | Beginner |
| FLOW.md | Technical details | Advanced |
| GUIDES.md | This navigation file | Reference |

---

## 🚀 Next Steps

1. Read [README.md](README.md) for overview
2. Follow [SETUP_TOKENS.md](SETUP_TOKENS.md) to create bot and deploy
3. Check your bot is working in Discord
4. Reference other guides as needed

Good luck! 🎉
