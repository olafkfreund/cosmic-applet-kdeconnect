//! MPRIS DBus Manager
//!
//! Manages integration with local MPRIS2 media players via DBus.
//! Discovers players, monitors their state, and provides control methods.

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use zbus::Connection;

/// MPRIS2 DBus interface names
pub const MPRIS_INTERFACE: &str = "org.mpris.MediaPlayer2";
pub const MPRIS_PLAYER_INTERFACE: &str = "org.mpris.MediaPlayer2.Player";
pub const MPRIS_BUS_PREFIX: &str = "org.mpris.MediaPlayer2.";

/// Playback status from MPRIS2
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackStatus {
    Playing,
    Paused,
    Stopped,
}

impl PlaybackStatus {
    pub fn from_str(s: &str) -> Self {
        match s {
            "Playing" => Self::Playing,
            "Paused" => Self::Paused,
            _ => Self::Stopped,
        }
    }

    pub fn is_playing(&self) -> bool {
        matches!(self, Self::Playing)
    }
}

/// Loop status from MPRIS2
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopStatus {
    None,
    Track,
    Playlist,
}

impl LoopStatus {
    pub fn from_str(s: &str) -> Self {
        match s {
            "Track" => Self::Track,
            "Playlist" => Self::Playlist,
            _ => Self::None,
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Track => "Track",
            Self::Playlist => "Playlist",
        }
    }
}

/// Media player metadata
#[derive(Debug, Clone, Default)]
pub struct PlayerMetadata {
    pub artist: Option<String>,
    pub title: Option<String>,
    pub album: Option<String>,
    pub album_art_url: Option<String>,
    pub length: i64, // microseconds
}

/// Player state from MPRIS2
#[derive(Debug, Clone)]
pub struct PlayerState {
    pub name: String,
    pub identity: String,
    pub playback_status: PlaybackStatus,
    pub position: i64, // microseconds
    pub volume: f64,   // 0.0 to 1.0
    pub loop_status: LoopStatus,
    pub shuffle: bool,
    pub can_play: bool,
    pub can_pause: bool,
    pub can_go_next: bool,
    pub can_go_previous: bool,
    pub can_seek: bool,
    pub metadata: PlayerMetadata,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            name: String::new(),
            identity: String::new(),
            playback_status: PlaybackStatus::Stopped,
            position: 0,
            volume: 1.0,
            loop_status: LoopStatus::None,
            shuffle: false,
            can_play: true,
            can_pause: true,
            can_go_next: true,
            can_go_previous: true,
            can_seek: true,
            metadata: PlayerMetadata::default(),
        }
    }
}

/// MPRIS DBus Manager
///
/// Manages discovery and control of MPRIS2 media players on the session bus.
pub struct MprisManager {
    connection: Connection,
    players: Arc<RwLock<HashMap<String, PlayerState>>>,
}

impl MprisManager {
    /// Create a new MPRIS manager
    pub async fn new() -> Result<Self> {
        let connection = Connection::session()
            .await
            .context("Failed to connect to session bus")?;

        Ok(Self {
            connection,
            players: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Discover all MPRIS2 players on the session bus
    pub async fn discover_players(&self) -> Result<Vec<String>> {
        let dbus_proxy = zbus::fdo::DBusProxy::new(&self.connection)
            .await
            .context("Failed to create DBus proxy")?;

        let names = dbus_proxy
            .list_names()
            .await
            .context("Failed to list DBus names")?;

        let mut players = Vec::new();
        for name in names {
            if name.starts_with(MPRIS_BUS_PREFIX) {
                let player_name = name.strip_prefix(MPRIS_BUS_PREFIX).unwrap().to_string();
                debug!("Discovered MPRIS player: {}", player_name);
                players.push(player_name);
            }
        }

        info!("Discovered {} MPRIS players", players.len());
        Ok(players)
    }

    /// Get list of active players
    pub async fn get_player_list(&self) -> Vec<String> {
        self.players.read().await.keys().cloned().collect()
    }

    /// Get player state
    pub async fn get_player_state(&self, player: &str) -> Option<PlayerState> {
        self.players.read().await.get(player).cloned()
    }

    /// Query player state from DBus (not yet implemented - requires zbus property access)
    pub async fn query_player_state(&self, _player: &str) -> Result<PlayerState> {
        // TODO: Implement DBus property queries for:
        // - org.mpris.MediaPlayer2.Player.PlaybackStatus
        // - org.mpris.MediaPlayer2.Player.Position
        // - org.mpris.MediaPlayer2.Player.Volume
        // - org.mpris.MediaPlayer2.Player.LoopStatus
        // - org.mpris.MediaPlayer2.Player.Shuffle
        // - org.mpris.MediaPlayer2.Player.Metadata
        // - org.mpris.MediaPlayer2.Player.CanPlay
        // - org.mpris.MediaPlayer2.Player.CanPause
        // - org.mpris.MediaPlayer2.Player.CanGoNext
        // - org.mpris.MediaPlayer2.Player.CanGoPrevious
        // - org.mpris.MediaPlayer2.Player.CanSeek
        // - org.mpris.MediaPlayer2.Identity

        warn!("query_player_state not yet implemented - returning default state");
        Ok(PlayerState::default())
    }

    /// Call a playback control method (not yet implemented)
    pub async fn call_player_method(&self, _player: &str, _method: &str) -> Result<()> {
        // TODO: Implement DBus method calls for:
        // - org.mpris.MediaPlayer2.Player.Play()
        // - org.mpris.MediaPlayer2.Player.Pause()
        // - org.mpris.MediaPlayer2.Player.PlayPause()
        // - org.mpris.MediaPlayer2.Player.Stop()
        // - org.mpris.MediaPlayer2.Player.Next()
        // - org.mpris.MediaPlayer2.Player.Previous()
        // - org.mpris.MediaPlayer2.Player.Seek(offset)
        // - org.mpris.MediaPlayer2.Player.SetPosition(trackid, position)

        warn!("call_player_method not yet implemented");
        Ok(())
    }

    /// Set a player property (not yet implemented)
    pub async fn set_player_property(
        &self,
        _player: &str,
        _property: &str,
        _value: zbus::zvariant::Value<'_>,
    ) -> Result<()> {
        // TODO: Implement DBus property setters for:
        // - org.mpris.MediaPlayer2.Player.Volume
        // - org.mpris.MediaPlayer2.Player.LoopStatus
        // - org.mpris.MediaPlayer2.Player.Shuffle

        warn!("set_player_property not yet implemented");
        Ok(())
    }

    /// Subscribe to PropertiesChanged signals (not yet implemented)
    pub async fn subscribe_to_changes(&self, _player: &str) -> Result<()> {
        // TODO: Implement PropertiesChanged signal subscription
        // Need to use zbus proxy and signal subscription APIs
        // Match rule for org.freedesktop.DBus.Properties.PropertiesChanged
        // on org.mpris.MediaPlayer2.Player interface
        //
        // Example approach:
        // 1. Create proxy for player's org.mpris.MediaPlayer2.Player interface
        // 2. Use proxy.receive_properties_changed() to get signal stream
        // 3. Spawn background task to monitor stream and update state

        warn!("subscribe_to_changes not yet implemented");
        Ok(())
    }

    /// Start monitoring a player (not yet fully implemented)
    pub async fn start_monitoring(&self, player: String) -> Result<()> {
        info!("Starting MPRIS monitoring for player: {}", player);

        // Query initial state
        let state = self.query_player_state(&player).await?;

        // Store state
        self.players.write().await.insert(player.clone(), state);

        // TODO: Subscribe to PropertiesChanged signals for this player
        // and update state in background task

        Ok(())
    }

    /// Stop monitoring a player
    pub async fn stop_monitoring(&self, player: &str) {
        info!("Stopping MPRIS monitoring for player: {}", player);
        self.players.write().await.remove(player);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_playback_status() {
        assert_eq!(PlaybackStatus::from_str("Playing"), PlaybackStatus::Playing);
        assert_eq!(PlaybackStatus::from_str("Paused"), PlaybackStatus::Paused);
        assert_eq!(PlaybackStatus::from_str("Stopped"), PlaybackStatus::Stopped);
        assert!(PlaybackStatus::Playing.is_playing());
        assert!(!PlaybackStatus::Paused.is_playing());
    }

    #[tokio::test]
    async fn test_loop_status() {
        assert_eq!(LoopStatus::from_str("None"), LoopStatus::None);
        assert_eq!(LoopStatus::from_str("Track"), LoopStatus::Track);
        assert_eq!(LoopStatus::from_str("Playlist"), LoopStatus::Playlist);
        assert_eq!(LoopStatus::None.to_string(), "None");
        assert_eq!(LoopStatus::Track.to_string(), "Track");
    }

    // Integration tests require DBus session bus
    // Skipping for now as they would fail in CI
}
