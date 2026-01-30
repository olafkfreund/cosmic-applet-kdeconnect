//! XDG Desktop Portal integration for Screen Share
//!
//! Uses the ScreenCast portal to request permission and get PipeWire stream info.

#[cfg(feature = "screenshare")]
use ashpd::desktop::{
    screencast::{CursorMode, Screencast, SourceType},
    PersistMode,
};
#[cfg(feature = "screenshare")]
use std::os::fd::OwnedFd;
#[cfg(feature = "screenshare")]
use tracing::{debug, error, info};

use crate::Result;

/// Screen share portal session info
#[derive(Debug)]
pub struct PortalSession {
    /// PipeWire file descriptor
    #[cfg(feature = "screenshare")]
    pub pipewire_fd: OwnedFd,
    /// PipeWire node ID for the stream
    pub pipewire_node_id: u32,
}

#[cfg(feature = "screenshare")]
impl PortalSession {
    /// Get the raw fd value for GStreamer
    pub fn fd(&self) -> i32 {
        use std::os::fd::AsRawFd;
        self.pipewire_fd.as_raw_fd()
    }
}

/// Request screen share permission via XDG Desktop Portal
///
/// This shows the system screen selection dialog and returns the PipeWire
/// stream information needed for GStreamer capture.
#[cfg(feature = "screenshare")]
pub async fn request_screencast() -> Result<PortalSession> {
    info!("Requesting screen share permission via Desktop Portal");

    let screencast = Screencast::new().await.map_err(|e| {
        error!("Failed to connect to ScreenCast portal: {}", e);
        crate::ProtocolError::Plugin(format!("Portal connection failed: {}", e))
    })?;

    // Create a session
    let session = screencast.create_session().await.map_err(|e| {
        error!("Failed to create screencast session: {}", e);
        crate::ProtocolError::Plugin(format!("Session creation failed: {}", e))
    })?;

    debug!("Created screencast session");

    // Select sources - allow monitor or window, with cursor embedded
    screencast
        .select_sources(
            &session,
            CursorMode::Embedded, // Include cursor in the stream
            SourceType::Monitor | SourceType::Window,
            false, // multiple: allow selecting one source
            None,  // restore_token: no previous session to restore
            PersistMode::DoNot, // don't persist this session
        )
        .await
        .map_err(|e| {
            error!("Failed to select sources: {}", e);
            crate::ProtocolError::Plugin(format!("Source selection failed: {}", e))
        })?;

    debug!("Sources selected, starting session");

    // Start the session - this shows the permission dialog
    // Pass None for window identifier (headless/CLI context)
    let response = screencast
        .start(&session, None)
        .await
        .map_err(|e| {
            error!("Failed to start screencast: {}", e);
            crate::ProtocolError::Plugin(format!("Screencast start failed: {}", e))
        })?
        .response()
        .map_err(|e| {
            error!("Screencast request was cancelled or failed: {}", e);
            crate::ProtocolError::Plugin(format!("Screencast response failed: {}", e))
        })?;

    // Get the streams from the response
    let streams = response.streams();
    if streams.is_empty() {
        error!("No streams available from screencast");
        return Err(crate::ProtocolError::Plugin(
            "No streams available".to_string(),
        ));
    }

    let stream = &streams[0];
    let node_id = stream.pipe_wire_node_id();

    debug!("Got PipeWire node ID: {}", node_id);

    // Open the PipeWire remote
    let fd = screencast.open_pipe_wire_remote(&session).await.map_err(|e| {
        error!("Failed to open PipeWire remote: {}", e);
        crate::ProtocolError::Plugin(format!("PipeWire remote failed: {}", e))
    })?;

    info!(
        "Screen share permission granted: node_id={}",
        node_id
    );

    Ok(PortalSession {
        pipewire_fd: fd,
        pipewire_node_id: node_id,
    })
}

/// Stub when screenshare feature is disabled
#[cfg(not(feature = "screenshare"))]
pub async fn request_screencast() -> Result<PortalSession> {
    Err(crate::ProtocolError::Plugin(
        "screenshare feature not enabled".to_string(),
    ))
}

/// Stub PortalSession when feature is disabled
#[cfg(not(feature = "screenshare"))]
impl PortalSession {
    pub fn fd(&self) -> i32 {
        -1
    }
}

#[cfg(all(test, feature = "screenshare"))]
mod tests {
    // Portal tests require a running D-Bus session and user interaction
    // These are integration tests that should be run manually
}
