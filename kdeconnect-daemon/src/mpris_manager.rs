//! MPRIS DBus Manager
//!
//! Manages integration with local MPRIS2 media players via DBus.
//! Discovers players, monitors their state, and provides control methods.

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use zbus::zvariant::OwnedValue;
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

    pub fn as_str(&self) -> &'static str {
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

    /// Standard MPRIS object path
    const MPRIS_OBJECT_PATH: &'static str = "/org/mpris/MediaPlayer2";

    /// Get the DBus bus name for a player
    fn player_bus_name(player: &str) -> String {
        format!("{}{}", MPRIS_BUS_PREFIX, player)
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

    /// Query player state from DBus
    pub async fn query_player_state(&self, player: &str) -> Result<PlayerState> {
        let bus_name = Self::player_bus_name(player);

        let player_proxy = zbus::Proxy::new(
            &self.connection,
            bus_name.as_str(),
            Self::MPRIS_OBJECT_PATH,
            MPRIS_PLAYER_INTERFACE,
        )
        .await
        .context("Failed to create player proxy")?;

        let mpris_proxy = zbus::Proxy::new(
            &self.connection,
            bus_name.as_str(),
            Self::MPRIS_OBJECT_PATH,
            MPRIS_INTERFACE,
        )
        .await
        .context("Failed to create MPRIS proxy")?;

        // Query string properties with defaults
        let playback_status: String = player_proxy
            .get_property("PlaybackStatus")
            .await
            .unwrap_or_else(|_| "Stopped".to_string());
        let loop_status: String = player_proxy
            .get_property("LoopStatus")
            .await
            .unwrap_or_else(|_| "None".to_string());
        let identity: String = mpris_proxy
            .get_property("Identity")
            .await
            .unwrap_or_else(|_| player.to_string());

        // Query numeric and boolean properties with defaults
        let position: i64 = player_proxy.get_property("Position").await.unwrap_or(0);
        let volume: f64 = player_proxy.get_property("Volume").await.unwrap_or(1.0);
        let shuffle: bool = player_proxy.get_property("Shuffle").await.unwrap_or(false);
        let can_play: bool = player_proxy.get_property("CanPlay").await.unwrap_or(true);
        let can_pause: bool = player_proxy.get_property("CanPause").await.unwrap_or(true);
        let can_go_next: bool = player_proxy.get_property("CanGoNext").await.unwrap_or(true);
        let can_go_previous: bool = player_proxy.get_property("CanGoPrevious").await.unwrap_or(true);
        let can_seek: bool = player_proxy.get_property("CanSeek").await.unwrap_or(true);

        let metadata = self.query_metadata(&player_proxy).await?;

        Ok(PlayerState {
            name: player.to_string(),
            identity,
            playback_status: PlaybackStatus::from_str(&playback_status),
            position,
            volume,
            loop_status: LoopStatus::from_str(&loop_status),
            shuffle,
            can_play,
            can_pause,
            can_go_next,
            can_go_previous,
            can_seek,
            metadata,
        })
    }

    /// Query metadata from player
    async fn query_metadata(&self, player_proxy: &zbus::Proxy<'_>) -> Result<PlayerMetadata> {
        let metadata_dict: HashMap<String, OwnedValue> = player_proxy
            .get_property("Metadata")
            .await
            .unwrap_or_default();

        // Helper to extract string fields from metadata
        let get_string = |key: &str| -> Option<String> {
            metadata_dict
                .get(key)
                .and_then(|v| <&str>::try_from(v).ok())
                .map(String::from)
        };

        Ok(PlayerMetadata {
            // TODO: Handle artist as array of strings (some players return arrays)
            artist: get_string("xesam:artist"),
            title: get_string("xesam:title"),
            album: get_string("xesam:album"),
            album_art_url: get_string("mpris:artUrl"),
            length: metadata_dict
                .get("mpris:length")
                .and_then(|v| i64::try_from(v).ok())
                .unwrap_or(0),
        })
    }

    /// Call a playback control method
    pub async fn call_player_method(&self, player: &str, method: &str) -> Result<()> {
        const VALID_METHODS: &[&str] = &["Play", "Pause", "PlayPause", "Stop", "Next", "Previous"];

        if !VALID_METHODS.contains(&method) {
            return Err(anyhow::anyhow!("Unknown method: {}", method));
        }

        let bus_name = Self::player_bus_name(player);
        let player_proxy = zbus::Proxy::new(
            &self.connection,
            bus_name.as_str(),
            Self::MPRIS_OBJECT_PATH,
            MPRIS_PLAYER_INTERFACE,
        )
        .await
        .context("Failed to create player proxy")?;

        player_proxy
            .call_method(method, &())
            .await
            .with_context(|| format!("Failed to call {}", method))?;

        debug!("Called {} on player {}", method, player);
        Ok(())
    }

    /// Seek relative to current position
    pub async fn seek(&self, player: &str, offset_microseconds: i64) -> Result<()> {
        let bus_name = Self::player_bus_name(player);
        let player_proxy = zbus::Proxy::new(
            &self.connection,
            bus_name.as_str(),
            Self::MPRIS_OBJECT_PATH,
            MPRIS_PLAYER_INTERFACE,
        )
        .await
        .context("Failed to create player proxy")?;

        player_proxy
            .call_method("Seek", &(offset_microseconds,))
            .await
            .context("Failed to call Seek")?;

        debug!("Seeked {} microseconds on player {}", offset_microseconds, player);
        Ok(())
    }

    /// Set absolute position
    pub async fn set_position(&self, player: &str, track_id: &str, position_microseconds: i64) -> Result<()> {
        use zbus::zvariant::ObjectPath;

        let bus_name = Self::player_bus_name(player);
        let player_proxy = zbus::Proxy::new(
            &self.connection,
            bus_name.as_str(),
            Self::MPRIS_OBJECT_PATH,
            MPRIS_PLAYER_INTERFACE,
        )
        .await
        .context("Failed to create player proxy")?;

        let track_path = ObjectPath::try_from(track_id)?;
        player_proxy
            .call_method("SetPosition", &(track_path, position_microseconds))
            .await
            .context("Failed to call SetPosition")?;

        debug!("Set position to {} on player {}", position_microseconds, player);
        Ok(())
    }

    /// Open URI
    pub async fn open_uri(&self, player: &str, uri: &str) -> Result<()> {
        let bus_name = Self::player_bus_name(player);
        let player_proxy = zbus::Proxy::new(
            &self.connection,
            bus_name.as_str(),
            Self::MPRIS_OBJECT_PATH,
            MPRIS_PLAYER_INTERFACE,
        )
        .await
        .context("Failed to create player proxy")?;

        player_proxy
            .call_method("OpenUri", &(uri,))
            .await
            .context("Failed to call OpenUri")?;

        debug!("Opened URI {} on player {}", uri, player);
        Ok(())
    }

    /// Set volume (0.0 to 1.0+)
    pub async fn set_volume(&self, player: &str, volume: f64) -> Result<()> {
        let bus_name = Self::player_bus_name(player);
        let player_proxy = zbus::Proxy::new(
            &self.connection,
            bus_name.as_str(),
            Self::MPRIS_OBJECT_PATH,
            MPRIS_PLAYER_INTERFACE,
        )
        .await
        .context("Failed to create player proxy")?;

        player_proxy
            .set_property("Volume", volume)
            .await
            .context("Failed to set Volume")?;

        debug!("Set volume to {} on player {}", volume, player);
        Ok(())
    }

    /// Set loop status
    pub async fn set_loop_status(&self, player: &str, loop_status: LoopStatus) -> Result<()> {
        let bus_name = Self::player_bus_name(player);
        let player_proxy = zbus::Proxy::new(
            &self.connection,
            bus_name.as_str(),
            Self::MPRIS_OBJECT_PATH,
            MPRIS_PLAYER_INTERFACE,
        )
        .await
        .context("Failed to create player proxy")?;

        player_proxy
            .set_property("LoopStatus", loop_status.as_str())
            .await
            .context("Failed to set LoopStatus")?;

        debug!("Set loop status to {} on player {}", loop_status.as_str(), player);
        Ok(())
    }

    /// Set shuffle
    pub async fn set_shuffle(&self, player: &str, shuffle: bool) -> Result<()> {
        let bus_name = Self::player_bus_name(player);
        let player_proxy = zbus::Proxy::new(
            &self.connection,
            bus_name.as_str(),
            Self::MPRIS_OBJECT_PATH,
            MPRIS_PLAYER_INTERFACE,
        )
        .await
        .context("Failed to create player proxy")?;

        player_proxy
            .set_property("Shuffle", shuffle)
            .await
            .context("Failed to set Shuffle")?;

        debug!("Set shuffle to {} on player {}", shuffle, player);
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
        assert_eq!(LoopStatus::None.as_str(), "None");
        assert_eq!(LoopStatus::Track.as_str(), "Track");
    }

    // Integration tests require DBus session bus
    // Skipping for now as they would fail in CI
}
