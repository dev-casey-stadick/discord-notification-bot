// PM2 configuration for keeping bot running 24/7 with auto-restart
module.exports = {
  apps: [
    {
      name: 'discord-notifications',
      script: './bot.js',
      instances: 1,
      exec_mode: 'fork',
      env: {
        NODE_ENV: 'production'
      },
      // Auto-restart on crash
      autorestart: true,
      // Restart if memory exceeds 500MB
      max_memory_restart: '500M',
      // Log files
      out_file: 'logs/out.log',
      error_file: 'logs/error.log',
      // Ignore unnecessary restarts
      watch: false,
      // Graceful shutdown
      kill_timeout: 5000,
    }
  ]
};
