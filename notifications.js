/**
 * Notification mechanisms for voice channel events
 * Easily extensible for different notification types
 */

/**
 * Send a DM to a specific user
 * @param {Client} client - Discord client
 * @param {string} userId - Discord user ID
 * @param {string} message - Message to send
 */
export async function notifyUserDM(client, userId, message) {
  try {
    const user = await client.users.fetch(userId);
    await user.send(message);
    console.log(`✉️  DM sent to ${user.username}`);
  } catch (error) {
    console.error(`❌ Failed to DM user ${userId}:`, error.message);
  }
}

/**
 * Post event to a notification channel
 * @param {Guild} guild - Discord guild
 * @param {string} channelId - Channel ID to post to
 * @param {Object} event - Event data
 * @param {string} event.type - 'join' | 'leave' | 'move'
 * @param {string} event.username - Username
 * @param {string} event.channelName - Voice channel name
 * @param {number} event.userCount - Users in channel
 */
export async function notifyChannelEmbed(guild, channelId, event) {
  try {
    const channel = await guild.channels.fetch(channelId);

    const colors = {
      join: 0x00ff00,    // Green
      leave: 0xff0000,   // Red
      move: 0xffaa00,    // Orange
    };

    const emoji = {
      join: '📍',
      leave: '👋',
      move: '➡️',
    };

    const embed = {
      color: colors[event.type] || 0x808080,
      title: `${emoji[event.type]} ${event.type.toUpperCase()}`,
      fields: [
        {
          name: 'User',
          value: event.username,
          inline: true,
        },
        {
          name: 'Channel',
          value: event.channelName,
          inline: true,
        },
      ],
      timestamp: new Date(),
    };

    if (event.userCount !== undefined) {
      embed.fields.push({
        name: 'Users in Channel',
        value: event.userCount.toString(),
        inline: true,
      });
    }

    await channel.send({ embeds: [embed] });
    console.log(`📤 Embed posted to #${channel.name}`);
  } catch (error) {
    console.error(`❌ Failed to post to channel ${channelId}:`, error.message);
  }
}

/**
 * Send a simple text message to a channel
 * @param {Guild} guild - Discord guild
 * @param {string} channelId - Channel ID
 * @param {string} message - Message text
 */
export async function notifyChannelText(guild, channelId, message) {
  try {
    const channel = await guild.channels.fetch(channelId);
    await channel.send(message);
    console.log(`💬 Message sent to #${channel.name}`);
  } catch (error) {
    console.error(`❌ Failed to message channel ${channelId}:`, error.message);
  }
}

/**
 * Mention a user/role and notify
 * @param {Guild} guild
 * @param {string} channelId
 * @param {string} mentionId - User or role ID
 * @param {string} message - Message to send
 */
export async function notifyWithMention(guild, channelId, mentionId, message) {
  try {
    const channel = await guild.channels.fetch(channelId);
    await channel.send(`<@${mentionId}> ${message}`);
    console.log(`🔔 Mention notification sent to #${channel.name}`);
  } catch (error) {
    console.error(`❌ Failed to send mention notification:`, error.message);
  }
}

/**
 * Send webhook notification (for external services)
 * @param {string} webhookUrl - Discord webhook URL
 * @param {Object} payload - Webhook payload
 */
export async function notifyWebhook(webhookUrl, payload) {
  try {
    const response = await fetch(webhookUrl, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(payload),
    });

    if (!response.ok) {
      throw new Error(`HTTP ${response.status}`);
    }
    console.log(`🔗 Webhook notification sent`);
  } catch (error) {
    console.error(`❌ Webhook notification failed:`, error.message);
  }
}
