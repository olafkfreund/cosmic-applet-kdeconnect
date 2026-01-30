//! System Volume Plugin
//!
//! Allows remote control of system volume and audio sinks using PipeWire/WirePlumber.
//!
//! ## Protocol
//!
//! **Packet Types**:
//! - `cconnect.systemvolume.request` - Volume control request (incoming)
//! - `cconnect.systemvolume` - Sink list update (outgoing)
//!
//! **Capabilities**:
//! - Incoming: `cconnect.systemvolume.request`
//! - Outgoing: `cconnect.systemvolume`
//!
//! ## Packet Format
//!
//! **Request (incoming)**:
//! ```json
//! {
//!     "type": "cconnect.systemvolume.request",
//!     "body": {
//!         "name": "Sink Name",
//!         "volume": 75,
//!         "muted": false,
//!         "enabled": true,
//!         "requestSinks": false
//!     }
//! }
//! ```
//!
//! **Sink List (outgoing)**:
//! ```json
//! {
//!     "type": "cconnect.systemvolume",
//!     "body": {
//!         "sinkList": [
//!             {
//!                 "name": "Realtek USB Audio",
//!                 "description": "Front Speaker",
//!                 "volume": 100,
//!                 "muted": false,
//!                 "maxVolume": 150,
//!                 "enabled": true
//!             }
//!         ]
//!     }
//! }
//! ```

use crate::{Device, Packet, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::any::Any;
use tokio::sync::mpsc;
use tracing::{debug, info, warn};

use super::audio_backend::{AudioBackend, AudioSink};
use super::{Plugin, PluginFactory};

/// Packet type for system volume requests (incoming)
pub const PACKET_TYPE_SYSTEMVOLUME_REQUEST: &str = "cconnect.systemvolume.request";

/// Packet type for sink list updates (outgoing)
pub const PACKET_TYPE_SYSTEMVOLUME: &str = "cconnect.systemvolume";

/// System volume request body (incoming)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemVolumeRequest {
    /// Name of the audio sink to control
    pub name: Option<String>,
    /// Volume level (0-100, can go higher for boost)
    pub volume: Option<i32>,
    /// Mute status
    pub muted: Option<bool>,
    /// Set as default/enabled sink
    pub enabled: Option<bool>,
    /// Request list of sinks from this device
    #[serde(rename = "requestSinks", default)]
    pub request_sinks: bool,
}

/// Sink information for protocol (outgoing)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SinkInfo {
    /// Unique sink name/identifier
    pub name: String,
    /// Human-readable description
    pub description: String,
    /// Current volume (0-100+)
    pub volume: i32,
    /// Whether the sink is muted
    pub muted: bool,
    /// Maximum volume (typically 150 for boost)
    #[serde(rename = "maxVolume")]
    pub max_volume: i32,
    /// Whether this is the active/default sink
    pub enabled: bool,
}

impl From<AudioSink> for SinkInfo {
    fn from(sink: AudioSink) -> Self {
        Self {
            name: sink.id.to_string(), // Use ID as unique identifier
            description: sink.name,
            volume: sink.volume,
            muted: sink.muted,
            max_volume: sink.max_volume,
            enabled: sink.is_default,
        }
    }
}

/// Sink list response body (outgoing)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SinkListResponse {
    /// List of available sinks
    #[serde(rename = "sinkList")]
    pub sink_list: Vec<SinkInfo>,
}

/// System Volume plugin
pub struct SystemVolumePlugin {
    device_id: Option<String>,
    packet_sender: Option<mpsc::Sender<(String, Packet)>>,
    /// Cache of known sinks (keyed by name from protocol)
    sink_cache: std::collections::HashMap<String, u32>,
}

impl SystemVolumePlugin {
    /// Create a new System Volume plugin
    pub fn new() -> Self {
        Self {
            device_id: None,
            packet_sender: None,
            sink_cache: std::collections::HashMap::new(),
        }
    }

    /// Send sink list to remote device
    async fn send_sink_list(&mut self) -> Result<()> {
        let sinks = AudioBackend::list_sinks();

        // Update cache
        self.sink_cache.clear();
        for sink in &sinks {
            self.sink_cache.insert(sink.id.to_string(), sink.id);
        }

        let sink_list: Vec<SinkInfo> = sinks.into_iter().map(SinkInfo::from).collect();

        info!("Sending {} sinks to remote device", sink_list.len());

        let response = SinkListResponse { sink_list };
        let packet = Packet::new(PACKET_TYPE_SYSTEMVOLUME, serde_json::to_value(response)?);

        if let (Some(sender), Some(device_id)) = (&self.packet_sender, &self.device_id) {
            sender
                .send((device_id.clone(), packet))
                .await
                .map_err(|e| crate::ProtocolError::Transport(format!("Failed to send packet: {}", e)))?;
        }

        Ok(())
    }

    /// Handle volume request from remote device
    async fn handle_volume_request(&mut self, packet: &Packet) -> Result<()> {
        let request: SystemVolumeRequest = serde_json::from_value(packet.body.clone()).map_err(
            |e| crate::ProtocolError::InvalidPacket(format!("Failed to parse volume request: {}", e)),
        )?;

        debug!("Received volume request: {:?}", request);

        // Handle sink list request
        if request.request_sinks {
            info!("Remote device requested audio sink list");
            self.send_sink_list().await?;
            return Ok(());
        }

        // Find the sink by name
        let sink_id = if let Some(name) = &request.name {
            // Try to parse as ID first (our protocol uses ID as name)
            if let Ok(id) = name.parse::<u32>() {
                Some(id)
            } else {
                // Fall back to cache lookup or name search
                self.sink_cache.get(name).copied().or_else(|| {
                    AudioBackend::find_sink_by_name(name).map(|s| s.id)
                })
            }
        } else {
            // Use default sink if no name specified
            AudioBackend::get_default_sink_id()
        };

        let Some(sink_id) = sink_id else {
            warn!("Could not find sink: {:?}", request.name);
            return Ok(());
        };

        // Apply volume change
        if let Some(volume) = request.volume {
            info!("Setting volume to {}% for sink {}", volume, sink_id);
            if !AudioBackend::set_volume(sink_id, volume) {
                warn!("Failed to set volume for sink {}", sink_id);
            }
        }

        // Apply mute change
        if let Some(muted) = request.muted {
            info!("Setting mute to {} for sink {}", muted, sink_id);
            if !AudioBackend::set_mute(sink_id, muted) {
                warn!("Failed to set mute for sink {}", sink_id);
            }
        }

        // Send updated sink list after changes
        self.send_sink_list().await?;

        Ok(())
    }
}

impl Default for SystemVolumePlugin {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Plugin for SystemVolumePlugin {
    fn name(&self) -> &str {
        "systemvolume"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn incoming_capabilities(&self) -> Vec<String> {
        vec![
            PACKET_TYPE_SYSTEMVOLUME_REQUEST.to_string(),
            "kdeconnect.systemvolume.request".to_string(),
        ]
    }

    fn outgoing_capabilities(&self) -> Vec<String> {
        vec![
            PACKET_TYPE_SYSTEMVOLUME.to_string(),
            "kdeconnect.systemvolume".to_string(),
        ]
    }

    async fn init(
        &mut self,
        device: &Device,
        packet_sender: mpsc::Sender<(String, Packet)>,
    ) -> Result<()> {
        self.device_id = Some(device.id().to_string());
        self.packet_sender = Some(packet_sender);

        // Check if audio backend is available
        if !AudioBackend::is_available() {
            warn!("wpctl not available - system volume control will not work");
        }

        Ok(())
    }

    async fn start(&mut self) -> Result<()> {
        info!("SystemVolume plugin started");

        // Send initial sink list to remote device
        if AudioBackend::is_available() {
            if let Err(e) = self.send_sink_list().await {
                warn!("Failed to send initial sink list: {}", e);
            }
        }

        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        info!("SystemVolume plugin stopped");
        Ok(())
    }

    async fn handle_packet(&mut self, packet: &Packet, _device: &mut Device) -> Result<()> {
        if packet.is_type(PACKET_TYPE_SYSTEMVOLUME_REQUEST)
            || packet.is_type("kdeconnect.systemvolume.request")
        {
            self.handle_volume_request(packet).await
        } else {
            Ok(())
        }
    }
}

/// Factory for creating SystemVolumePlugin instances
pub struct SystemVolumePluginFactory;

impl PluginFactory for SystemVolumePluginFactory {
    fn name(&self) -> &str {
        "systemvolume"
    }

    fn incoming_capabilities(&self) -> Vec<String> {
        vec![
            PACKET_TYPE_SYSTEMVOLUME_REQUEST.to_string(),
            "kdeconnect.systemvolume.request".to_string(),
        ]
    }

    fn outgoing_capabilities(&self) -> Vec<String> {
        vec![
            PACKET_TYPE_SYSTEMVOLUME.to_string(),
            "kdeconnect.systemvolume".to_string(),
        ]
    }

    fn create(&self) -> Box<dyn Plugin> {
        Box::new(SystemVolumePlugin::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sink_info_from_audio_sink() {
        let audio_sink = AudioSink {
            id: 50,
            name: "Test Speaker".to_string(),
            volume: 75,
            muted: false,
            is_default: true,
            max_volume: 150,
        };

        let sink_info: SinkInfo = audio_sink.into();
        assert_eq!(sink_info.name, "50");
        assert_eq!(sink_info.description, "Test Speaker");
        assert_eq!(sink_info.volume, 75);
        assert!(!sink_info.muted);
        assert!(sink_info.enabled);
        assert_eq!(sink_info.max_volume, 150);
    }

    #[test]
    fn test_parse_volume_request() {
        let json = serde_json::json!({
            "name": "50",
            "volume": 80,
            "muted": false,
            "requestSinks": false
        });

        let request: SystemVolumeRequest = serde_json::from_value(json).unwrap();
        assert_eq!(request.name, Some("50".to_string()));
        assert_eq!(request.volume, Some(80));
        assert_eq!(request.muted, Some(false));
        assert!(!request.request_sinks);
    }

    #[test]
    fn test_parse_request_sinks() {
        let json = serde_json::json!({
            "requestSinks": true
        });

        let request: SystemVolumeRequest = serde_json::from_value(json).unwrap();
        assert!(request.request_sinks);
        assert!(request.name.is_none());
        assert!(request.volume.is_none());
    }
}
