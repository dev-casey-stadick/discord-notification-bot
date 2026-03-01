/**
 * Discord bot that notifies on voice channel joins with exponential backoff retry
 */

import { Client, GatewayIntentBits, Events } from 'discord.js';
import 'dotenv/config';

const client = new Client({
  intents: [
    GatewayIntentBits.Guilds,
    GatewayIntentBits.GuildVoiceStates,
    GatewayIntentBits.DirectMessages,
  ],
});

// Configuration
const CONFIG = {
  notifyUserIds: process.env.NOTIFY_USER_ID?.split(',').map(id => id.trim()).filter(Boolean) || [],
  trackedGuildIds: process.env.TRACKED_GUILD_IDS?.split(',').filter(Boolean) || [],
  trackedVoiceChannelIds: process.env.TRACKED_VOICE_CHANNELS?.split(',').filter(Boolean) || [],
};

// Track channel states: { channelId: memberCount }
const channelStates = new Map();

// Exponential backoff retry with jitter
async function sendDMWithRetry(userId, message, maxRetries = 3) {
  for (let attempt = 0; attempt < maxRetries; attempt++) {
    try {
      const user = await client.users.fetch(userId);
      await user.send(message);
      console.log(`✉️  DM sent successfully to ${user.username}`);
      return true;
    } catch (error) {
      const isLastAttempt = attempt === maxRetries - 1;

      if (isLastAttempt) {
        console.error(`❌ Failed to send DM after ${maxRetries} attempts:`, error.message);
        return false;
      }

      // Exponential backoff: 1s, 2s, 4s, etc. + random jitter (0-500ms)
      const delayMs = (1000 * Math.pow(2, attempt)) + Math.random() * 500;
      const delaySecs = (delayMs / 1000).toFixed(2);

      console.log(`⏳ DM failed (attempt ${attempt + 1}/${maxRetries}), retrying in ${delaySecs}s...`);
      await new Promise(resolve => setTimeout(resolve, delayMs));
    }
  }
}

client.once(Events.ClientReady, (readyClient) => {
  console.log(`✅ Bot logged in as ${readyClient.user.tag}`);
  console.log(`📡 Listening for voice channel joins...`);

  if (CONFIG.notifyUserIds.length > 0) {
    console.log(`📧 Join notifications → ${CONFIG.notifyUserIds.length} user(s)`);
  }
});

client.on(Events.VoiceStateUpdate, async (oldState, newState) => {
  const member = newState.member;
  const oldChannel = oldState.channel;
  const newChannel = newState.channel;
  const guild = newState.guild;

  if (!member) return;

  // Process old channel (user left it)
  if (oldChannel) {
    const oldCount = oldChannel.members.size;
    const previousCount = channelStates.get(oldChannel.id) || 0;

    // Channel is now empty (transitioned from occupied to empty)
    if (oldCount === 0 && previousCount > 0) {
      console.log(`\n🔕 [EMPTY] #${oldChannel.name} is now empty`);

      if (CONFIG.notifyUserIds.length > 0) {
        const message = `🔕 **#${oldChannel.name}** is now empty`;
        for (const userId of CONFIG.notifyUserIds) {
          await sendDMWithRetry(userId, message);
        }
      }
    }

    // Update state
    if (oldCount === 0) {
      channelStates.delete(oldChannel.id);
    } else {
      channelStates.set(oldChannel.id, oldCount);
    }
  }

  // Process new channel (user joined it)
  if (newChannel) {
    // Skip if tracking specific guilds
    if (CONFIG.trackedGuildIds.length > 0 &&
        !CONFIG.trackedGuildIds.includes(guild.id)) {
      return;
    }

    // Skip if tracking specific voice channels
    if (CONFIG.trackedVoiceChannelIds.length > 0 &&
        !CONFIG.trackedVoiceChannelIds.includes(newChannel.id)) {
      return;
    }

    const newCount = newChannel.members.size;
    const previousCount = channelStates.get(newChannel.id) || 0;

    // Channel just became occupied (transitioned from empty to occupied)
    if (newCount > 0 && previousCount === 0) {
      console.log(`\n🟢 [OCCUPIED] #${newChannel.name} is now occupied`);
      console.log(`   First person: ${member.user.username}`);
      console.log(`   Users in channel: ${newCount}`);

      if (CONFIG.notifyUserIds.length > 0) {
        const message = `🟢 **#${newChannel.name}** is now occupied (${member.user.username} joined)`;
        for (const userId of CONFIG.notifyUserIds) {
          await sendDMWithRetry(userId, message);
        }
      }
    }

    // Update state
    channelStates.set(newChannel.id, newCount);
  }
});

client.on('error', (error) => {
  console.error('❌ Client error:', error);
});

process.on('unhandledRejection', (reason) => {
  console.error('❌ Unhandled rejection:', reason);
});

const TOKEN = process.env.DISCORD_TOKEN;
if (!TOKEN) {
  console.error('❌ DISCORD_TOKEN environment variable is not set!');
  process.exit(1);
}

client.login(TOKEN);
