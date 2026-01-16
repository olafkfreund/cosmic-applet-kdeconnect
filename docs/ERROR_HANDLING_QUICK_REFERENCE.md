# Error Handling Quick Reference

Quick reference guide for common error handling tasks in COSMIC Connect.

## Common Patterns

### Handle a Protocol Error

```rust
use cosmic_connect_protocol::ProtocolError;

match operation().await {
    Ok(result) => Ok(result),
    Err(e) => {
        let is_recoverable = error_handler.handle_error(
            &e,
            "operation name",
            Some(device_id)
        ).await;

        if is_recoverable {
            // Queue for retry
        }

        Err(e)
    }
}
```

### Show User Notification

```rust
// Network error
notifier.notify_network_error(device_name, error_msg).await?;

// File transfer error
notifier.notify_file_transfer_error(device_name, filename, error_msg).await?;

// Permission error
notifier.notify_permission_error(operation, details).await?;

// Disk full
notifier.notify_disk_full_error(path).await?;

// Certificate error
notifier.notify_certificate_error(device_name, details).await?;

// Generic error with recovery action
notifier.notify_error_with_recovery(
    "Title",
    "Message",
    Some(("action", "Button Label"))
).await?;
```

### Safe File Operations

```rust
use cosmic_connect_protocol::fs_utils::*;

// Create file (creates parent dirs, handles permissions, detects disk full)
let mut file = create_file_safe(path).await?;

// Write safely (detects disk full)
write_file_safe(&mut file, data).await?;

// Cleanup on failure
if result.is_err() {
    cleanup_partial_file(path).await;
}

// Get unique path (avoids conflicts)
let path = get_unique_download_path(dir, filename).await;
```

### Auto-Reconnection

```rust
// Automatic via RecoveryCoordinator (recommended)
recovery_coordinator.start().await?;

// Manual
if let Some(delay) = recovery_manager.should_reconnect(device_id).await {
    sleep(delay).await;
    connection_manager.connect(device_id, addr).await?;
}

// Reset on success
recovery_manager.reset_reconnection_strategy(device_id).await;
```

### Packet Retry

```rust
// Queue for retry
recovery_manager.queue_packet_retry(
    device_id.to_string(),
    packet.clone()
).await;

// Process retry queue (call periodically, e.g., every 5s)
recovery_coordinator.process_packet_retries().await?;

// Clear on reconnection
recovery_manager.clear_device_retry_queue(device_id).await;
```

### Transfer State Tracking

```rust
// Register new transfer
let state = TransferState::new(
    transfer_id,
    device_id,
    filename,
    file_path,
    total_size
);
recovery_manager.register_transfer(state).await?;

// Update progress
recovery_manager.update_transfer_progress(transfer_id, bytes).await?;

// Complete
recovery_manager.complete_transfer(transfer_id).await?;

// Resume interrupted transfer
if let Some(state) = recovery_manager.get_transfer_state(transfer_id).await {
    // Resume from state.bytes_received
}
```

## Error Classification Checks

```rust
// Is error recoverable?
if error.is_recoverable() {
    // Auto-retry possible
}

// Does error require user action?
if error.requires_user_action() {
    // Show notification
}

// Get user-friendly message
let message = error.user_message();
```

## Recovery Configuration

```rust
// Reconnection settings (in RecoveryManager)
const MAX_RECONNECT_ATTEMPTS: u32 = 5;
const INITIAL_RECONNECT_DELAY: Duration = Duration::from_secs(2);
const MAX_RECONNECT_DELAY: Duration = Duration::from_secs(60);

// Packet retry settings
const MAX_PACKET_RETRIES: u32 = 3;
const PACKET_RETRY_DELAY: Duration = Duration::from_millis(500);
```

## Periodic Tasks

```rust
// Packet retry processing (every 5 seconds)
tokio::spawn({
    let coordinator = recovery_coordinator.clone();
    async move {
        let mut interval = tokio::time::interval(Duration::from_secs(5));
        loop {
            interval.tick().await;
            let _ = coordinator.process_packet_retries().await;
        }
    }
});

// Cleanup old transfers (daily)
tokio::spawn({
    let coordinator = recovery_coordinator.clone();
    async move {
        let mut interval = tokio::time::interval(Duration::from_secs(86400));
        loop {
            interval.tick().await;
            let _ = coordinator.cleanup_old_transfers().await;
        }
    }
});
```

## Common Error Types

### Recoverable (auto-retry)
- `Timeout`
- `NetworkError`
- `NetworkUnreachable`
- `ConnectionRefused`
- `Io`

### User Action Required (show notification)
- `NotPaired`
- `PermissionDenied`
- `Certificate` / `CertificateValidation`
- `Configuration`
- `ProtocolVersionMismatch`
- `ResourceExhausted`

### Critical (log and report)
- `InvalidPacket`
- `TlsError`
- `InternalError`

## Logging Best Practices

```rust
// Recoverable errors
warn!("Recoverable error in {}: {}", context, error);

// User action required
warn!("User action required in {}: {}", context, error);

// Critical errors
error!("Critical error in {}: {}", context, error);

// Recovery actions
info!("Attempting reconnection to device {}", device_id);
debug!("Queued packet '{}' for retry", packet_type);
```

## Troubleshooting Commands

```bash
# Check recovery state
cat ~/.local/share/cosmic/cosmic-connect/recovery_state.json | jq

# Monitor daemon logs for errors
journalctl -u cosmic-connect -f | grep -E "(ERROR|WARN)"

# Check disk space
df -h ~/Downloads

# Check folder permissions
ls -ld ~/Downloads

# View recent transfers
cat ~/.local/share/cosmic/cosmic-connect/recovery_state.json | jq '.[] | {transfer_id, filename, progress: (.bytes_received / .total_size * 100)}'
```

## Quick Checklist

### Adding New Error-Prone Operation

- [ ] Wrap operation in Result<T>
- [ ] Use ProtocolError variants
- [ ] Call error_handler.handle_error()
- [ ] Check if recoverable and queue retry if appropriate
- [ ] Use appropriate logging level
- [ ] Add context (device_id, operation name)
- [ ] Write unit test for error case

### Adding New File Operation

- [ ] Use create_file_safe() instead of File::create()
- [ ] Use write_file_safe() instead of write_all()
- [ ] Call cleanup_partial_file() on error
- [ ] Use get_unique_download_path() for downloads
- [ ] Handle PermissionDenied and ResourceExhausted errors
- [ ] Register transfer with RecoveryManager
- [ ] Update progress during transfer

### Adding New Notification Type

- [ ] Create specialized method in CosmicNotifier
- [ ] Set appropriate urgency (Low/Normal/Critical)
- [ ] Choose timeout (5s-10s)
- [ ] Select appropriate icon
- [ ] Add recovery action if applicable
- [ ] Call from ErrorHandler.notify_error()
- [ ] Write test for notification
