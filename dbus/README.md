# D-Bus Interface Definitions

This directory contains D-Bus interface definitions for COSMIC Connect services.

## Files

### org.cosmicde.PhoneAuth.xml
D-Bus interface definition for phone-based biometric authentication. Defines methods, signals, and properties for the authentication service.

### org.cosmicde.PhoneAuth.service
D-Bus service activation file. Tells D-Bus how to start the authentication service.

### org.cosmicde.PhoneAuth.conf
D-Bus security policy configuration. Controls who can access the authentication service.

### polkit/org.cosmicde.PhoneAuth.policy
Polkit authorization rules. Defines privilege requirements for administrative actions.

## Installation

### For Development (Session Bus)

```bash
# Install D-Bus service file
mkdir -p ~/.local/share/dbus-1/services
cp org.cosmicde.PhoneAuth.service ~/.local/share/dbus-1/services/

# Install D-Bus configuration
mkdir -p ~/.local/share/dbus-1/session.d
cp org.cosmicde.PhoneAuth.conf ~/.local/share/dbus-1/session.d/

# Install Polkit policy
sudo cp polkit/org.cosmicde.PhoneAuth.policy /usr/share/polkit-1/actions/
```

### For System-Wide Installation

```bash
# Install D-Bus service file
sudo cp org.cosmicde.PhoneAuth.service /usr/share/dbus-1/services/

# Install D-Bus configuration
sudo cp org.cosmicde.PhoneAuth.conf /usr/share/dbus-1/session.d/

# Install Polkit policy
sudo cp polkit/org.cosmicde.PhoneAuth.policy /usr/share/polkit-1/actions/

# Reload Polkit
sudo systemctl reload polkit
```

## Validation

### Validate XML Interface

```bash
# Check XML syntax
xmllint --noout org.cosmicde.PhoneAuth.xml

# Validate against D-Bus DTD
xmllint --valid --noout org.cosmicde.PhoneAuth.xml
```

### Test D-Bus Configuration

```bash
# Check D-Bus configuration syntax
dbus-daemon --config-file=org.cosmicde.PhoneAuth.conf --print-address --nofork --session
```

### Verify Polkit Policy

```bash
# List available actions
pkaction | grep org.cosmicde.PhoneAuth

# Check authorization for current user
pkcheck --action-id org.cosmicde.PhoneAuth.request --process $$
pkcheck --action-id org.cosmicde.PhoneAuth.admin --process $$
```

## Introspection (After Service is Running)

```bash
# Introspect the interface
gdbus introspect --session \
  --dest org.cosmicde.PhoneAuth \
  --object-path /org/cosmicde/PhoneAuth

# Call a method (example)
gdbus call --session \
  --dest org.cosmicde.PhoneAuth \
  --object-path /org/cosmicde/PhoneAuth \
  --method org.cosmicde.PhoneAuth.RequestAuth "username" "sudo"

# Monitor signals
gdbus monitor --session --dest org.cosmicde.PhoneAuth

# Get properties
gdbus call --session \
  --dest org.cosmicde.PhoneAuth \
  --object-path /org/cosmicde/PhoneAuth \
  --method org.freedesktop.DBus.Properties.Get \
  "org.cosmicde.PhoneAuth" "ConnectedDevices"
```

## Testing with d-feet

The [d-feet](https://wiki.gnome.org/Apps/DFeet) GUI tool provides an easy way to explore and test D-Bus interfaces:

```bash
# Install d-feet
sudo apt install d-feet  # Ubuntu/Debian
sudo dnf install d-feet  # Fedora

# Launch d-feet and connect to Session Bus
d-feet
```

## Interface Overview

### Methods

- **RequestAuth(username, auth_type) → request_id**
  - Initiate a new authentication request
  - Returns a unique ID for tracking

- **CheckAuth(request_id) → (approved, biometric_type)**
  - Check the status of a pending request
  - Returns approval status and biometric method used

- **CancelAuth(request_id) → success**
  - Cancel a pending authentication request
  - Returns true if successfully cancelled

- **GetPendingRequests() → requests**
  - Administrative method to list all pending requests
  - Requires Polkit `org.cosmicde.PhoneAuth.admin` authorization

### Signals

- **AuthCompleted(request_id, approved, biometric_type)**
  - Emitted when an authentication request completes
  - PAM modules should listen for this signal

- **AuthTimeout(request_id)**
  - Emitted when a request times out
  - Treated as authentication failure

### Properties

- **DefaultTimeout** (uint64, read-only)
  - Default timeout in seconds for auth requests
  - Default: 30 seconds

- **ConnectedDevices** (uint32, read-only)
  - Number of connected devices capable of auth
  - If 0, RequestAuth will fail immediately

## Security Considerations

### Authentication Flow

1. PAM module calls `RequestAuth()` with username and auth type
2. Service sends request to paired mobile devices
3. User approves/denies on their phone using biometrics
4. Service emits `AuthCompleted` signal
5. PAM module receives signal and grants/denies access

### Authorization Levels

- **Regular users** can:
  - Request authentication for their own account
  - Check status of their own requests
  - Cancel their own requests
  - Receive authentication signals

- **Administrators** can:
  - View all pending requests across all users
  - Configure system-wide authentication settings

### Polkit Actions

- `org.cosmicde.PhoneAuth.request` - Request authentication (active user)
- `org.cosmicde.PhoneAuth.cancel` - Cancel own requests (active user)
- `org.cosmicde.PhoneAuth.admin` - View all requests (admin)
- `org.cosmicde.PhoneAuth.configure` - Modify settings (admin)

## Implementation Notes

The actual D-Bus service implementation will be in `cosmic-connect-daemon` using the `zbus` Rust library. The interface definition here serves as the contract between the daemon and PAM modules.

### Object Path

The service will be available at object path: `/org/cosmicde/PhoneAuth`

### Bus Type

The service runs on the **session bus** (not system bus) to maintain user isolation and follow COSMIC Desktop patterns.

## Related Files

- `cosmic-connect-daemon/src/dbus.rs` - Existing D-Bus implementation
- `com.system76.CosmicConnect.service` - Main service D-Bus activation
- PAM module implementation (to be created in Phase 2)

## References

- [D-Bus Specification](https://dbus.freedesktop.org/doc/dbus-specification.html)
- [Polkit Authorization](https://www.freedesktop.org/software/polkit/docs/latest/)
- [zbus Documentation](https://docs.rs/zbus/)
- [PAM Programming Guide](http://www.linux-pam.org/Linux-PAM-html/)
