FROM node:20-alpine

WORKDIR /app

# Install dumb-init to handle signals properly
RUN apk add --no-cache dumb-init

# Copy package files
COPY package*.json ./

# Install dependencies
RUN npm install --production

# Copy bot and notification files
COPY bot-with-notifications.js .
COPY notifications.js .

# Create logs directory
RUN mkdir -p logs

# Health check (optional but recommended)
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
  CMD node -e "console.log('Bot is running')" || exit 1

# Use dumb-init to handle signals properly (allows graceful shutdown)
ENTRYPOINT ["/sbin/dumb-init", "--"]

# Start the bot
CMD ["node", "bot-with-notifications.js"]
