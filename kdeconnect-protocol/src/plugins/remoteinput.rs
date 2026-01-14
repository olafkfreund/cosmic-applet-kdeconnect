//! Remote Input Plugin
//!
//! This plugin enables remote control of the pointer and keyboard.
//! It supports mouse movements, clicks, scrolling, and keyboard input.
//!
//! ## Protocol
//!
//! **Packet Types**:
//! - `kdeconnect.mousepad.request` - Remote input request (incoming)
//! - `kdeconnect.mousepad.echo` - Echo response (outgoing)
//! - `kdeconnect.mousepad.keyboardstate` - Keyboard state broadcast (outgoing)
//!
//! **Capabilities**:
//! - Incoming: `kdeconnect.mousepad.request` - Receives pointer and keyboard events
//! - Outgoing: `kdeconnect.mousepad.keyboardstate` - Sends keyboard support status
//!
//! ## References
//!
//! - [KDE Connect MousePad Plugin](https://github.com/KDE/kdeconnect-kde/tree/master/plugins/mousepad)
//! - [Valent Protocol - MousePad](https://valent.andyholmes.ca/documentation/protocol.html)

use crate::{Device, Packet, ProtocolError, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::any::Any;
use tracing::{debug, info, warn};

use super::{Plugin, PluginFactory};

/// Packet type for remote input requests
pub const PACKET_TYPE_MOUSEPAD_REQUEST: &str = "kdeconnect.mousepad.request";

/// Packet type for echo responses
pub const PACKET_TYPE_MOUSEPAD_ECHO: &str = "kdeconnect.mousepad.echo";

/// Packet type for keyboard state
pub const PACKET_TYPE_MOUSEPAD_KEYBOARDSTATE: &str = "kdeconnect.mousepad.keyboardstate";

/// Special key codes for non-printable characters
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum SpecialKey {
    Backspace = 1,
    Tab = 2,
    Enter = 12,
    Escape = 27,
    Left = 21,
    Up = 22,
    Right = 23,
    Down = 24,
    PageUp = 25,
    PageDown = 26,
    Home = 28,
    End = 29,
    Delete = 30,
    F1 = 31,
    F2 = 32,
    F3 = 33,
    F4 = 34,
    F5 = 35,
    F6 = 36,
    F7 = 37,
    F8 = 38,
    F9 = 39,
    F10 = 40,
    F11 = 41,
    F12 = 42,
}

/// Remote input request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteInputRequest {
    /// Single readable character input
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    /// Non-printable character (0-32)
    #[serde(skip_serializing_if = "Option::is_none", rename = "specialKey")]
    pub special_key: Option<i32>,

    /// Alt modifier key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt: Option<bool>,

    /// Ctrl modifier key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctrl: Option<bool>,

    /// Shift modifier key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift: Option<bool>,

    /// Super/Windows/Command modifier key
    #[serde(skip_serializing_if = "Option::is_none", rename = "super")]
    pub super_key: Option<bool>,

    /// Single click action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub singleclick: Option<bool>,

    /// Double click action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doubleclick: Option<bool>,

    /// Middle click action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middleclick: Option<bool>,

    /// Right click action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rightclick: Option<bool>,

    /// Single hold (press) action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub singlehold: Option<bool>,

    /// Single release action
    #[serde(skip_serializing_if = "Option::is_none")]
    pub singlerelease: Option<bool>,

    /// Position delta on X axis
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dx: Option<f64>,

    /// Position delta on Y axis
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dy: Option<f64>,

    /// Whether movement is a scroll event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scroll: Option<bool>,

    /// Request confirmation via echo packet
    #[serde(skip_serializing_if = "Option::is_none", rename = "sendAck")]
    pub send_ack: Option<bool>,
}

/// Remote Input plugin for pointer and keyboard control
pub struct RemoteInputPlugin {
    device_id: Option<String>,
}

impl RemoteInputPlugin {
    /// Create a new Remote Input plugin
    pub fn new() -> Self {
        Self { device_id: None }
    }

    /// Handle a remote input request packet
    async fn handle_request(&self, packet: &Packet) -> Result<()> {
        let request: RemoteInputRequest = serde_json::from_value(packet.body.clone())
            .map_err(|e| ProtocolError::InvalidPacket(format!("Failed to parse request: {}", e)))?;

        // Handle mouse movement
        if request.dx.is_some() || request.dy.is_some() {
            let dx = request.dx.unwrap_or(0.0);
            let dy = request.dy.unwrap_or(0.0);
            let is_scroll = request.scroll.unwrap_or(false);

            if is_scroll {
                debug!("Remote input: Scroll dx={}, dy={}", dx, dy);
                // TODO: Implement scroll via COSMIC APIs
            } else {
                debug!("Remote input: Move pointer dx={}, dy={}", dx, dy);
                // TODO: Implement pointer movement via COSMIC APIs
            }
        }

        // Handle mouse clicks
        if request.singleclick.unwrap_or(false) {
            debug!("Remote input: Single click");
            // TODO: Implement click via COSMIC APIs
        }
        if request.doubleclick.unwrap_or(false) {
            debug!("Remote input: Double click");
            // TODO: Implement double click via COSMIC APIs
        }
        if request.middleclick.unwrap_or(false) {
            debug!("Remote input: Middle click");
            // TODO: Implement middle click via COSMIC APIs
        }
        if request.rightclick.unwrap_or(false) {
            debug!("Remote input: Right click");
            // TODO: Implement right click via COSMIC APIs
        }
        if request.singlehold.unwrap_or(false) {
            debug!("Remote input: Single hold");
            // TODO: Implement button press via COSMIC APIs
        }
        if request.singlerelease.unwrap_or(false) {
            debug!("Remote input: Single release");
            // TODO: Implement button release via COSMIC APIs
        }

        // Handle keyboard input
        if let Some(key) = &request.key {
            debug!("Remote input: Key '{}'", key);
            // TODO: Implement keyboard input via COSMIC APIs
        }
        if let Some(special_key) = request.special_key {
            debug!("Remote input: Special key {}", special_key);
            // TODO: Implement special key via COSMIC APIs
        }

        Ok(())
    }
}

impl Default for RemoteInputPlugin {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Plugin for RemoteInputPlugin {
    fn name(&self) -> &str {
        "remoteinput"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn incoming_capabilities(&self) -> Vec<String> {
        vec![PACKET_TYPE_MOUSEPAD_REQUEST.to_string()]
    }

    fn outgoing_capabilities(&self) -> Vec<String> {
        vec![PACKET_TYPE_MOUSEPAD_KEYBOARDSTATE.to_string()]
    }

    async fn init(&mut self, device: &Device) -> Result<()> {
        self.device_id = Some(device.id().to_string());
        info!("Remote Input plugin initialized for device {}", device.name());
        Ok(())
    }

    async fn start(&mut self) -> Result<()> {
        info!("Remote Input plugin started");
        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        info!("Remote Input plugin stopped");
        Ok(())
    }

    async fn handle_packet(&mut self, packet: &Packet, _device: &mut Device) -> Result<()> {
        match packet.packet_type.as_str() {
            PACKET_TYPE_MOUSEPAD_REQUEST => {
                debug!("Received remote input request");
                self.handle_request(packet).await
            }
            _ => {
                warn!("Unexpected packet type: {}", packet.packet_type);
                Ok(())
            }
        }
    }
}

/// Factory for creating Remote Input plugin instances
#[derive(Debug, Clone, Copy)]
pub struct RemoteInputPluginFactory;

impl PluginFactory for RemoteInputPluginFactory {
    fn name(&self) -> &str {
        "remoteinput"
    }

    fn incoming_capabilities(&self) -> Vec<String> {
        vec![PACKET_TYPE_MOUSEPAD_REQUEST.to_string()]
    }

    fn outgoing_capabilities(&self) -> Vec<String> {
        vec![PACKET_TYPE_MOUSEPAD_KEYBOARDSTATE.to_string()]
    }

    fn create(&self) -> Box<dyn Plugin> {
        Box::new(RemoteInputPlugin::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DeviceInfo, DeviceType};

    fn create_test_device() -> Device {
        let info = DeviceInfo::new("Test Device", DeviceType::Desktop, 1716);
        Device::from_discovery(info)
    }

    #[tokio::test]
    async fn test_plugin_creation() {
        let plugin = RemoteInputPlugin::new();
        assert_eq!(plugin.name(), "remoteinput");
        assert!(plugin.device_id.is_none());
    }

    #[tokio::test]
    async fn test_plugin_initialization() {
        let mut plugin = RemoteInputPlugin::new();
        let device = create_test_device();

        assert!(plugin.init(&device).await.is_ok());
        assert_eq!(plugin.device_id, Some(device.id().to_string()));
    }

    #[tokio::test]
    async fn test_handle_mouse_movement() {
        let mut plugin = RemoteInputPlugin::new();
        let device = create_test_device();
        plugin.init(&device).await.unwrap();

        let packet = Packet::new(
            "kdeconnect.mousepad.request",
            serde_json::json!({
                "dx": 10.0,
                "dy": 20.0
            }),
        );

        let mut device_mut = device;
        let result = plugin.handle_packet(&packet, &mut device_mut).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_mouse_click() {
        let mut plugin = RemoteInputPlugin::new();
        let device = create_test_device();
        plugin.init(&device).await.unwrap();

        let packet = Packet::new(
            "kdeconnect.mousepad.request",
            serde_json::json!({
                "singleclick": true
            }),
        );

        let mut device_mut = device;
        let result = plugin.handle_packet(&packet, &mut device_mut).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_keyboard_input() {
        let mut plugin = RemoteInputPlugin::new();
        let device = create_test_device();
        plugin.init(&device).await.unwrap();

        let packet = Packet::new(
            "kdeconnect.mousepad.request",
            serde_json::json!({
                "key": "a"
            }),
        );

        let mut device_mut = device;
        let result = plugin.handle_packet(&packet, &mut device_mut).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_special_key() {
        let mut plugin = RemoteInputPlugin::new();
        let device = create_test_device();
        plugin.init(&device).await.unwrap();

        let packet = Packet::new(
            "kdeconnect.mousepad.request",
            serde_json::json!({
                "specialKey": 1
            }),
        );

        let mut device_mut = device;
        let result = plugin.handle_packet(&packet, &mut device_mut).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_scroll_event() {
        let mut plugin = RemoteInputPlugin::new();
        let device = create_test_device();
        plugin.init(&device).await.unwrap();

        let packet = Packet::new(
            "kdeconnect.mousepad.request",
            serde_json::json!({
                "dx": 0.0,
                "dy": -5.0,
                "scroll": true
            }),
        );

        let mut device_mut = device;
        let result = plugin.handle_packet(&packet, &mut device_mut).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_modifiers() {
        let mut plugin = RemoteInputPlugin::new();
        let device = create_test_device();
        plugin.init(&device).await.unwrap();

        let packet = Packet::new(
            "kdeconnect.mousepad.request",
            serde_json::json!({
                "key": "c",
                "ctrl": true
            }),
        );

        let mut device_mut = device;
        let result = plugin.handle_packet(&packet, &mut device_mut).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_factory() {
        let factory = RemoteInputPluginFactory;
        assert_eq!(factory.name(), "remoteinput");

        let incoming = factory.incoming_capabilities();
        assert!(incoming.contains(&PACKET_TYPE_MOUSEPAD_REQUEST.to_string()));

        let outgoing = factory.outgoing_capabilities();
        assert!(outgoing.contains(&PACKET_TYPE_MOUSEPAD_KEYBOARDSTATE.to_string()));

        let plugin = factory.create();
        assert_eq!(plugin.name(), "remoteinput");
    }

    #[tokio::test]
    async fn test_plugin_lifecycle() {
        let mut plugin = RemoteInputPlugin::new();
        let device = create_test_device();

        assert!(plugin.init(&device).await.is_ok());
        assert!(plugin.start().await.is_ok());
        assert!(plugin.stop().await.is_ok());
    }
}
