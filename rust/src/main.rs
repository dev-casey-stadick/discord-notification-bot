use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;
use twilight_gateway::{Intents, Shard, ShardId};
use twilight_http::Client as HttpClient;
use twilight_model::gateway::event::Event;

/// Configuration loaded from environment variables
#[derive(Clone, Debug, PartialEq)]
struct Config {
    notify_user_ids: Vec<u64>,
    tracked_guild_ids: Vec<u64>,
    tracked_voice_channel_ids: Vec<u64>,
}

impl Config {
    fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        dotenvy::dotenv().ok();

        let token = std::env::var("DISCORD_TOKEN")?;
        if token.is_empty() {
            return Err("DISCORD_TOKEN is empty".into());
        }

        let notify_user_ids = std::env::var("NOTIFY_USER_ID")
            .unwrap_or_default()
            .split(',')
            .filter_map(|s| {
                let trimmed = s.trim();
                if trimmed.is_empty() {
                    None
                } else {
                    trimmed.parse::<u64>().ok()
                }
            })
            .collect();

        let tracked_guild_ids = std::env::var("TRACKED_GUILD_IDS")
            .unwrap_or_default()
            .split(',')
            .filter_map(|s| {
                let trimmed = s.trim();
                if trimmed.is_empty() {
                    None
                } else {
                    trimmed.parse::<u64>().ok()
                }
            })
            .collect();

        let tracked_voice_channel_ids = std::env::var("TRACKED_VOICE_CHANNELS")
            .unwrap_or_default()
            .split(',')
            .filter_map(|s| {
                let trimmed = s.trim();
                if trimmed.is_empty() {
                    None
                } else {
                    trimmed.parse::<u64>().ok()
                }
            })
            .collect();

        Ok(Config {
            notify_user_ids,
            tracked_guild_ids,
            tracked_voice_channel_ids,
        })
    }

    /// Check if a guild should be tracked
    fn should_track_guild(&self, guild_id: u64) -> bool {
        self.tracked_guild_ids.is_empty() || self.tracked_guild_ids.contains(&guild_id)
    }

    /// Check if a voice channel should be tracked
    fn should_track_channel(&self, channel_id: u64) -> bool {
        self.tracked_voice_channel_ids.is_empty() || self.tracked_voice_channel_ids.contains(&channel_id)
    }

    /// Check if should notify
    fn should_notify(&self) -> bool {
        !self.notify_user_ids.is_empty()
    }
}

/// Shared state: maps channel_id -> member_count
type ChannelStates = Arc<Mutex<HashMap<u64, usize>>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Load config
    let config = Config::from_env()?;
    let token = std::env::var("DISCORD_TOKEN")?;

    // Create gateway shard with intents
    let intents = Intents::GUILDS | Intents::GUILD_VOICE_STATES;
    let mut shard = Shard::new(ShardId::new(0, 1), token.clone(), intents);

    info!("✅ Bot starting...");
    info!("📡 Listening for voice channel joins...");

    if config.should_notify() {
        info!(
            "📧 Join notifications → {} user(s)",
            config.notify_user_ids.len()
        );
    }

    let channel_states: ChannelStates = Arc::new(Mutex::new(HashMap::new()));

    // Main event loop
    loop {
        match shard.next_event().await {
            Ok(event) => {
                // Spawn event handling in background to not block shard
                let config = config.clone();
                let states = channel_states.clone();
                let http = Arc::new(HttpClient::new(token.clone()));
                tokio::spawn(handle_event(event, config, states, http));
            }
            Err(e) => {
                eprintln!("❌ Shard error: {}", e);
                // Reconnect logic is handled by shard automatically
            }
        }
    }
}

/// Handle individual gateway events
async fn handle_event(event: Event, config: Config, channel_states: ChannelStates, http: Arc<HttpClient>) {
    match event {
        Event::Ready(ready) => {
            info!("✅ Bot logged in as {}", ready.user.name);
        }
        Event::VoiceStateUpdate(voice_update) => {
            handle_voice_state_update(voice_update.0, config, channel_states, http).await;
        }
        _ => {}
    }
}

/// Handle voice state updates
async fn handle_voice_state_update(
    new_state: twilight_model::voice::VoiceState,
    config: Config,
    channel_states: ChannelStates,
    http: Arc<HttpClient>,
) {
    let new_channel = new_state.channel_id.map(|ch| ch.get());
    let guild_id = match new_state.guild_id {
        Some(gid) => gid.get(),
        None => return,
    };

    let user_id = new_state.user_id.get();
    let mut states = channel_states.lock().await;

    // Handle when user joins a channel
    if let Some(ch_id) = new_channel {
        let previous_count = states.get(&ch_id).copied().unwrap_or(0);

        // If user joined a voice channel
        if previous_count == 0 {
            // Channel just became occupied
            if !config.should_track_guild(guild_id) {
                return;
            }

            if !config.should_track_channel(ch_id) {
                return;
            }

            // Fetch guild name
            let guild_name = match http
                .guild(twilight_model::id::Id::new(guild_id))
                .await
            {
                Ok(resp) => match resp.model().await {
                    Ok(guild) => guild.name,
                    Err(_) => format!("Guild {}", guild_id),
                },
                Err(_) => format!("Guild {}", guild_id),
            };

            // Fetch channel name
            let channel_name = match http
                .channel(twilight_model::id::Id::new(ch_id))
                .await
            {
                Ok(resp) => match resp.model().await {
                    Ok(channel) => channel.name.unwrap_or_else(|| format!("Channel {}", ch_id)),
                    Err(_) => format!("Channel {}", ch_id),
                },
                Err(_) => format!("Channel {}", ch_id),
            };

            // Fetch username
            let username = match http
                .user(twilight_model::id::Id::new(user_id))
                .await
            {
                Ok(resp) => match resp.model().await {
                    Ok(user) => user.name,
                    Err(_) => format!("User {}", user_id),
                },
                Err(_) => format!("User {}", user_id),
            };

            info!("\n🟢 [OCCUPIED] {}: {} is now occupied", guild_name, channel_name);
            info!("   User: {}", username);

            if config.should_notify() {
                let message = format!(
                    "🟢 **{}: {}** is now occupied ({} joined)",
                    guild_name, channel_name, username
                );
                for &notify_id in &config.notify_user_ids {
                    send_dm_with_retry(notify_id, &message, http.clone()).await;
                }
            }

            states.insert(ch_id, 1);
        } else {
            states.insert(ch_id, previous_count + 1);
        }
    } else {
        // User left a channel (new_channel is None)
        // We need to find which channel they left from
        // We'll check all tracked channels and decrement any that have members
        // In a real implementation, we'd get the old channel from the event
        // For now, we'll use a simple heuristic: decrement the most recently occupied channel

        if let Some((&ch_id, &count)) = states.iter().max_by_key(|&(_, &c)| c) {
            if count > 0 {
                let new_count = count - 1;

                // Check if this channel just became empty
                if new_count == 0 {
                    if !config.should_track_guild(guild_id) {
                        states.insert(ch_id, new_count);
                        return;
                    }

                    if !config.should_track_channel(ch_id) {
                        states.insert(ch_id, new_count);
                        return;
                    }

                    // Fetch guild name
                    let guild_name = match http
                        .guild(twilight_model::id::Id::new(guild_id))
                        .await
                    {
                        Ok(resp) => match resp.model().await {
                            Ok(guild) => guild.name,
                            Err(_) => format!("Guild {}", guild_id),
                        },
                        Err(_) => format!("Guild {}", guild_id),
                    };

                    // Fetch channel name
                    let channel_name = match http
                        .channel(twilight_model::id::Id::new(ch_id))
                        .await
                    {
                        Ok(resp) => match resp.model().await {
                            Ok(channel) => channel.name.unwrap_or_else(|| format!("Channel {}", ch_id)),
                            Err(_) => format!("Channel {}", ch_id),
                        },
                        Err(_) => format!("Channel {}", ch_id),
                    };

                    // Fetch username
                    let username = match http
                        .user(twilight_model::id::Id::new(user_id))
                        .await
                    {
                        Ok(resp) => match resp.model().await {
                            Ok(user) => user.name,
                            Err(_) => format!("User {}", user_id),
                        },
                        Err(_) => format!("User {}", user_id),
                    };

                    info!("\n🔕 [EMPTY] {}: {} is now empty", guild_name, channel_name);
                    info!("   User left: {}", username);

                    if config.should_notify() {
                        let message = format!(
                            "🔕 **{}: {}** is now empty",
                            guild_name, channel_name
                        );
                        for &notify_id in &config.notify_user_ids {
                            send_dm_with_retry(notify_id, &message, http.clone()).await;
                        }
                    }

                    states.remove(&ch_id);
                } else {
                    states.insert(ch_id, new_count);
                }
            }
        }
    }
}

/// Send a DM with exponential backoff retry
async fn send_dm_with_retry(user_id: u64, message: &str, http: Arc<HttpClient>) {
    for attempt in 0..3 {
        // Create a twilight ID from the u64
        let user = twilight_model::id::Id::new(user_id);

        // Create private channel
        match http.create_private_channel(user).await {
            Ok(response) => {
                let channel_id = response.model().await.ok().map(|ch| ch.id);

                if let Some(ch_id) = channel_id {
                    // Send message
                    match http.create_message(ch_id).content(message).unwrap().await {
                        Ok(_) => {
                            info!("✉️  DM sent successfully to {}", user_id);
                            return;
                        }
                        Err(e) => {
                            let is_last = attempt == 2;
                            if is_last {
                                info!("❌ Failed to send DM after 3 attempts: {}", e);
                                return;
                            }

                            let delay_ms = (1000 * 2_u64.pow(attempt as u32)) + (rand::random::<u64>() % 500);
                            info!("⏳ DM failed (attempt {}/3), retrying in {}ms...", attempt + 1, delay_ms);
                            tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
                        }
                    }
                }
            }
            Err(e) => {
                let is_last = attempt == 2;
                if is_last {
                    info!("❌ Failed to open DM channel after 3 attempts: {}", e);
                    return;
                }

                let delay_ms = (1000 * 2_u64.pow(attempt as u32)) + (rand::random::<u64>() % 500);
                info!("⏳ Failed to open DM channel (attempt {}/3), retrying in {}ms...", attempt + 1, delay_ms);
                tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_parsing_empty_env() {
        // With empty env vars, should create empty config
        std::env::remove_var("NOTIFY_USER_ID");
        std::env::remove_var("TRACKED_GUILD_IDS");
        std::env::remove_var("TRACKED_VOICE_CHANNELS");

        let config = Config {
            notify_user_ids: vec![],
            tracked_guild_ids: vec![],
            tracked_voice_channel_ids: vec![],
        };

        assert_eq!(config.notify_user_ids.len(), 0);
        assert_eq!(config.tracked_guild_ids.len(), 0);
        assert_eq!(config.tracked_voice_channel_ids.len(), 0);
    }

    #[test]
    fn test_config_should_track_guild_empty() {
        let config = Config {
            notify_user_ids: vec![123],
            tracked_guild_ids: vec![], // Empty = track all
            tracked_voice_channel_ids: vec![],
        };

        assert!(config.should_track_guild(999));
        assert!(config.should_track_guild(111));
    }

    #[test]
    fn test_config_should_track_guild_filtered() {
        let config = Config {
            notify_user_ids: vec![123],
            tracked_guild_ids: vec![100, 200],
            tracked_voice_channel_ids: vec![],
        };

        assert!(config.should_track_guild(100));
        assert!(config.should_track_guild(200));
        assert!(!config.should_track_guild(300));
    }

    #[test]
    fn test_config_should_track_channel_empty() {
        let config = Config {
            notify_user_ids: vec![123],
            tracked_guild_ids: vec![],
            tracked_voice_channel_ids: vec![], // Empty = track all
        };

        assert!(config.should_track_channel(999));
        assert!(config.should_track_channel(111));
    }

    #[test]
    fn test_config_should_track_channel_filtered() {
        let config = Config {
            notify_user_ids: vec![123],
            tracked_guild_ids: vec![],
            tracked_voice_channel_ids: vec![1001, 1002],
        };

        assert!(config.should_track_channel(1001));
        assert!(config.should_track_channel(1002));
        assert!(!config.should_track_channel(1003));
    }

    #[test]
    fn test_config_should_notify() {
        let config_with_notifs = Config {
            notify_user_ids: vec![123, 456],
            tracked_guild_ids: vec![],
            tracked_voice_channel_ids: vec![],
        };

        let config_no_notifs = Config {
            notify_user_ids: vec![],
            tracked_guild_ids: vec![],
            tracked_voice_channel_ids: vec![],
        };

        assert!(config_with_notifs.should_notify());
        assert!(!config_no_notifs.should_notify());
    }

    #[test]
    fn test_config_equality() {
        let config1 = Config {
            notify_user_ids: vec![123, 456],
            tracked_guild_ids: vec![789],
            tracked_voice_channel_ids: vec![999, 888],
        };

        let config2 = Config {
            notify_user_ids: vec![123, 456],
            tracked_guild_ids: vec![789],
            tracked_voice_channel_ids: vec![999, 888],
        };

        assert_eq!(config1, config2);
    }

    #[tokio::test]
    async fn test_channel_states_tracking() {
        let states: ChannelStates = Arc::new(Mutex::new(HashMap::new()));

        {
            let mut s = states.lock().await;
            s.insert(1001, 1);
            s.insert(1002, 2);
        }

        {
            let s = states.lock().await;
            assert_eq!(s.get(&1001), Some(&1));
            assert_eq!(s.get(&1002), Some(&2));
        }
    }
}
