# libcosmic Development Best Practices Skill

## Purpose

This skill provides comprehensive best practices, patterns, and guidelines for developing COSMIC Desktop applications and applets using libcosmic and Rust.

## When to Use This Skill

Use this skill when:
- Starting a new COSMIC application or applet
- Refactoring existing code
- Optimizing performance
- Implementing state management
- Working with COSMIC theming
- Integrating with the COSMIC desktop
- Debugging issues
- Following COSMIC design principles

## Core Principles

### 1. Rust Best Practices First

libcosmic is Rust-first, so always follow Rust best practices:
- **Avoid `unwrap()` and `expect()`** - Use proper error handling
- **Use `Result` and `Option`** - Propagate errors with `?`
- **Log errors, don't panic** - Use `tracing::error!()` or `tracing::warn!()`
- **Prefer explicit over implicit** - Make intentions clear
- **Use type safety** - Let the compiler help you

```rust
// ❌ BAD - Can panic
let value = some_option.unwrap();
let result = risky_operation().expect("Failed!");

// ✅ GOOD - Proper error handling
let value = some_option.ok_or(Error::MissingValue)?;
if let Err(e) = risky_operation() {
    tracing::error!("Operation failed: {}", e);
    return Err(e);
}
```

### 2. Memory Safety and Performance

- **Minimize allocations** - Reuse buffers and structures
- **Use `Cow` for conditional ownership** - Avoid unnecessary clones
- **Profile before optimizing** - Measure, don't guess
- **Async for I/O, sync for compute** - Choose appropriately

```rust
use std::borrow::Cow;

// ✅ GOOD - Avoids unnecessary allocation
fn format_status(online: bool) -> Cow<'static, str> {
    if online {
        Cow::Borrowed("Online")
    } else {
        Cow::Borrowed("Offline")
    }
}
```

### 3. User Experience First

- **Responsive UI** - Never block the main thread
- **Smooth animations** - Use COSMIC's built-in transitions
- **Follow COSMIC design language** - Use standard widgets
- **Respect user preferences** - Theme, font size, accessibility

## Project Structure Best Practices

### Recommended File Organization

```
your-cosmic-app/
├── src/
│   ├── main.rs              # Entry point + App struct
│   ├── config.rs            # Configuration types
│   ├── core.rs              # Core business logic
│   ├── pages/               # Page components (apps only)
│   │   ├── mod.rs
│   │   ├── home.rs
│   │   └── settings.rs
│   ├── components/          # Reusable UI components
│   │   ├── mod.rs
│   │   └── device_card.rs
│   └── utils/               # Helper functions
│       ├── mod.rs
│       └── format.rs
├── i18n/                    # Translations
│   ├── en/
│   └── de/
├── data/                    # Resources
│   ├── *.desktop
│   ├── icons/
│   └── config/
└── Cargo.toml
```

### Module Organization Pattern

```rust
// src/main.rs - Keep minimal
use cosmic::{app, Application};

mod config;
mod core;
mod components;
mod pages;

fn main() -> cosmic::iced::Result {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    
    app::run::<MyApp>((), ())?
}

// Application struct and implementation...
```

## State Management Best Practices

### 1. Centralized State

Keep application state in a single struct:

```rust
struct AppState {
    // Core data
    devices: Vec<Device>,
    selected_device: Option<usize>,
    
    // UI state
    search_query: String,
    filter_active: bool,
    
    // Async state
    loading: bool,
    error: Option<String>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            devices: Vec::new(),
            selected_device: None,
            search_query: String::new(),
            filter_active: false,
            loading: false,
            error: None,
        }
    }
}
```

### 2. Message-Driven Architecture

Use clear, typed messages:

```rust
#[derive(Debug, Clone)]
enum Message {
    // User actions
    DeviceSelected(usize),
    SearchChanged(String),
    FilterToggled,
    RefreshRequested,
    
    // Async responses
    DevicesLoaded(Result<Vec<Device>, Error>),
    DeviceUpdated(Device),
    
    // System events
    ConfigChanged(Config),
    ThemeChanged(Theme),
}
```

### 3. Update Function Organization

Structure your update function clearly:

```rust
fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
    match message {
        // User Actions
        Message::DeviceSelected(idx) => {
            self.state.selected_device = Some(idx);
            Command::none()
        }
        
        Message::RefreshRequested => {
            self.state.loading = true;
            Command::perform(
                async { fetch_devices().await },
                Message::DevicesLoaded
            )
        }
        
        // Async Responses
        Message::DevicesLoaded(result) => {
            self.state.loading = false;
            match result {
                Ok(devices) => {
                    self.state.devices = devices;
                    self.state.error = None;
                }
                Err(e) => {
                    tracing::error!("Failed to load devices: {}", e);
                    self.state.error = Some(e.to_string());
                }
            }
            Command::none()
        }
        
        // System Events
        Message::ConfigChanged(config) => {
            self.config = config;
            self.save_config();
            Command::none()
        }
    }
}
```

## UI/UX Best Practices

### 1. Use COSMIC Widgets

Always prefer libcosmic widgets over custom implementations:

```rust
use cosmic::widget::{
    button, column, container, row, text, text_input,
    settings, list, scrollable, divider, icon,
};

// ✅ GOOD - Use built-in widgets
fn view(&self) -> Element<Message> {
    let search = text_input("Search devices...", &self.search_query)
        .on_input(Message::SearchChanged);
    
    let device_list = column(
        self.devices
            .iter()
            .map(|device| device_card(device).into())
            .collect()
    )
    .spacing(8);
    
    column![search, device_list]
        .padding(16)
        .into()
}
```

### 2. Consistent Spacing

Follow COSMIC spacing guidelines:

```rust
// Standard spacing values
const SPACING_XXXS: u16 = 2;
const SPACING_XXS: u16 = 4;
const SPACING_XS: u16 = 8;
const SPACING_S: u16 = 12;
const SPACING_M: u16 = 16;  // Most common
const SPACING_L: u16 = 24;
const SPACING_XL: u16 = 32;

// ✅ GOOD - Consistent spacing
column![
    header,
    content,
]
.spacing(SPACING_M)
.padding(SPACING_L)
```

### 3. Respect Panel Size (Applets)

Applets must adapt to panel size:

```rust
fn view(&self) -> Element<Message> {
    let suggested_size = self.core.applet.suggested_size(false);
    let icon_size = (suggested_size.0 as f32 * 0.6) as u16;
    
    self.core
        .applet
        .icon_button_from_handle(self.icon.clone())
        .on_press_down(Message::Toggle)
        .into()
}
```

### 4. Loading States

Always show feedback for async operations:

```rust
fn view_content(&self) -> Element<Message> {
    if self.loading {
        return column![
            cosmic::widget::spinner(),
            text("Loading devices..."),
        ]
        .align_items(Alignment::Center)
        .into();
    }
    
    if let Some(error) = &self.error {
        return column![
            icon::from_name("dialog-error-symbolic").size(48),
            text(error),
            button::standard("Retry").on_press(Message::RefreshRequested),
        ]
        .align_items(Alignment::Center)
        .spacing(12)
        .into();
    }
    
    // Normal content...
}
```

### 5. Empty States

Provide helpful empty states:

```rust
fn view_devices(&self) -> Element<Message> {
    if self.devices.is_empty() {
        return column![
            icon::from_name("phone-symbolic").size(64),
            text("No devices found").size(18),
            text("Make sure your device is on the same network"),
            button::suggested("Scan for devices")
                .on_press(Message::RefreshRequested),
        ]
        .align_items(Alignment::Center)
        .spacing(16)
        .padding(48)
        .into();
    }
    
    // Device list...
}
```

## Configuration Management

### Use cosmic-config

```rust
use cosmic::cosmic_config::{Config, ConfigGet, ConfigSet, CosmicConfigEntry};

// Define config struct
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    pub auto_connect: bool,
    pub notification_enabled: bool,
    pub device_timeout: u64,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            auto_connect: true,
            notification_enabled: true,
            device_timeout: 30,
        }
    }
}

// In your app
struct MyApp {
    config_handler: Option<Config>,
    config: AppConfig,
}

impl MyApp {
    fn load_config(&mut self) {
        let config_handler = Config::new("com.example.myapp", 1).ok();
        
        let config = if let Some(ref handler) = config_handler {
            AppConfig {
                auto_connect: handler.get("auto_connect").unwrap_or(true),
                notification_enabled: handler.get("notification_enabled").unwrap_or(true),
                device_timeout: handler.get("device_timeout").unwrap_or(30),
            }
        } else {
            AppConfig::default()
        };
        
        self.config_handler = config_handler;
        self.config = config;
    }
    
    fn save_config(&self) {
        if let Some(ref handler) = self.config_handler {
            let _ = handler.set("auto_connect", self.config.auto_connect);
            let _ = handler.set("notification_enabled", self.config.notification_enabled);
            let _ = handler.set("device_timeout", self.config.device_timeout);
        }
    }
}
```

## Async Patterns

### 1. Background Tasks

Use subscriptions for ongoing background work:

```rust
use cosmic::iced::Subscription;

impl Application for MyApp {
    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::batch([
            // Device monitoring
            device_subscription(),
            // Connection health
            connection_subscription(),
        ])
    }
}

fn device_subscription() -> Subscription<Message> {
    cosmic::iced::subscription::channel(
        std::any::TypeId::of::<DeviceMonitor>(),
        100,
        |mut output| async move {
            let mut interval = tokio::time::interval(Duration::from_secs(5));
            
            loop {
                interval.tick().await;
                
                match check_devices().await {
                    Ok(devices) => {
                        let _ = output.send(Message::DevicesUpdated(devices)).await;
                    }
                    Err(e) => {
                        tracing::warn!("Device check failed: {}", e);
                    }
                }
            }
        }
    )
}
```

### 2. One-Shot Async Operations

Use Command::perform for single async tasks:

```rust
Message::SendFile(path) => {
    Command::perform(
        async move {
            send_file_async(path).await
        },
        |result| Message::FileSent(result)
    )
}
```

### 3. Cancellable Operations

Track operation IDs for cancellation:

```rust
struct AppState {
    current_operation: Option<OperationId>,
}

Message::StartOperation => {
    let id = OperationId::new();
    self.state.current_operation = Some(id);
    
    Command::perform(
        long_operation(id),
        Message::OperationComplete
    )
}

Message::CancelOperation => {
    self.state.current_operation = None;
    Command::none()
}

async fn long_operation(id: OperationId) -> Result<(), Error> {
    // Check if still valid periodically
    if !is_operation_active(id) {
        return Err(Error::Cancelled);
    }
    // ... do work
}
```

## Performance Optimization

### 1. Minimize Redraws

Only update what changes:

```rust
// ❌ BAD - Creates new strings every frame
fn view(&self) -> Element<Message> {
    text(format!("Device: {}", self.device_name))
}

// ✅ GOOD - Reuse allocation
struct AppState {
    device_label: String,
}

fn update(&mut self, message: Message) {
    match message {
        Message::DeviceChanged(name) => {
            self.state.device_label = format!("Device: {}", name);
        }
    }
}

fn view(&self) -> Element<Message> {
    text(&self.state.device_label)
}
```

### 2. Lazy Element Creation

Create elements only when needed:

```rust
fn view(&self) -> Element<Message> {
    let content = if self.show_details {
        Some(detailed_view(&self.device))
    } else {
        None
    };
    
    column![
        summary_view(&self.device),
        content.unwrap_or_else(|| Element::from(text("")))
    ]
    .into()
}
```

### 3. Use Scrollable Efficiently

```rust
use cosmic::widget::scrollable;

// ✅ GOOD - Efficient scrolling
fn view_list(&self) -> Element<Message> {
    scrollable(
        column(
            self.items
                .iter()
                .map(|item| item_view(item).into())
                .collect()
        )
    )
    .height(Length::Fill)
    .into()
}
```

### 4. Icon Caching

Cache icon handles:

```rust
use cosmic::widget::icon;

struct AppState {
    icons: HashMap<String, icon::Handle>,
}

impl AppState {
    fn get_icon(&mut self, name: &str, size: u16) -> icon::Handle {
        let key = format!("{}:{}", name, size);
        
        self.icons
            .entry(key)
            .or_insert_with(|| {
                icon::from_name(name).size(size).handle()
            })
            .clone()
    }
}
```

## Theming Best Practices

### 1. Use Theme Colors

Never hardcode colors:

```rust
use cosmic::theme;
use cosmic::iced::Color;

// ❌ BAD
let text_color = Color::from_rgb(1.0, 1.0, 1.0);

// ✅ GOOD - Use theme
fn view(&self) -> Element<Message> {
    text("Hello")
        .style(theme::Text::Color(
            self.core.system_theme().cosmic().accent_color().into()
        ))
}
```

### 2. Respect Dark/Light Mode

Test in both themes:

```rust
// COSMIC automatically handles theming, but be aware:
// - Use semantic colors (not specific RGB values)
// - Test UI in both dark and light mode
// - Icons should use symbolic variants (*-symbolic)
```

### 3. Custom Styling

When needed, use theme-aware styles:

```rust
use cosmic::theme::Container;

fn custom_container<'a>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    container(content)
        .style(Container::custom(|theme| {
            cosmic::widget::container::Appearance {
                text_color: Some(theme.cosmic().accent_text_color().into()),
                background: Some(theme.cosmic().accent_color().into()),
                border: cosmic::iced::Border {
                    radius: 8.0.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                ..Default::default()
            }
        }))
        .into()
}
```

## Error Handling Patterns

### 1. User-Friendly Errors

Convert technical errors to user-friendly messages:

```rust
#[derive(Debug)]
pub enum AppError {
    Network(std::io::Error),
    Parse(serde_json::Error),
    NotFound(String),
}

impl AppError {
    pub fn user_message(&self) -> String {
        match self {
            Self::Network(_) => "Unable to connect. Check your network connection.".to_string(),
            Self::Parse(_) => "Invalid data received. Please try again.".to_string(),
            Self::NotFound(item) => format!("{} not found.", item),
        }
    }
}

// In update
Message::Error(error) => {
    self.state.error_message = Some(error.user_message());
    Command::none()
}
```

### 2. Error Recovery

Provide recovery options:

```rust
fn view_error(&self, error: &str) -> Element<Message> {
    column![
        icon::from_name("dialog-error-symbolic").size(48),
        text(error),
        row![
            button::standard("Dismiss")
                .on_press(Message::ClearError),
            button::suggested("Retry")
                .on_press(Message::Retry),
        ]
        .spacing(8)
    ]
    .spacing(16)
    .align_items(Alignment::Center)
    .into()
}
```

## Testing Best Practices

### 1. Unit Tests for Logic

Test business logic separately:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_filtering() {
        let mut state = AppState::default();
        state.devices = vec![
            Device { name: "Phone".to_string(), online: true },
            Device { name: "Tablet".to_string(), online: false },
        ];
        
        state.search_query = "Phone".to_string();
        let filtered = state.filtered_devices();
        
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Phone");
    }
}
```

### 2. Integration Tests

Test message handling:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_device_selection() {
        let mut app = MyApp::new();
        
        // Simulate device load
        let devices = vec![Device::new("Test")];
        app.update(Message::DevicesLoaded(Ok(devices)));
        
        // Select first device
        app.update(Message::DeviceSelected(0));
        
        assert_eq!(app.state.selected_device, Some(0));
    }
}
```

## Logging Best Practices

### Use tracing consistently

```rust
use tracing::{debug, info, warn, error, trace};

// Different log levels
trace!("Detailed debug info: {:?}", packet);
debug!("Device discovered: {}", device.name);
info!("Connected to {}", device.name);
warn!("Connection unstable: {}", reason);
error!("Failed to send file: {}", error);

// Structured logging
use tracing::instrument;

#[instrument]
async fn connect_device(device_id: &str) -> Result<(), Error> {
    debug!("Connecting...");
    // ...
    info!("Connected successfully");
    Ok(())
}

// Configure in main
fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("my_app=debug".parse().unwrap())
        )
        .init();
}
```

## Common Pitfalls

### 1. Don't Block the UI Thread

```rust
// ❌ BAD - Blocks UI
Message::LoadData => {
    let data = expensive_operation(); // Freezes UI!
    self.data = data;
    Command::none()
}

// ✅ GOOD - Async operation
Message::LoadData => {
    Command::perform(
        async { expensive_operation().await },
        Message::DataLoaded
    )
}
```

### 2. Don't Leak Resources

```rust
// ❌ BAD - Resources not cleaned up
impl Drop for MyApp {
    fn drop(&mut self) {
        // Resources leak!
    }
}

// ✅ GOOD - Explicit cleanup
Message::Shutdown => {
    self.cleanup_resources();
    cosmic::iced::window::close()
}
```

### 3. Handle Window/Popup IDs Correctly

```rust
// ✅ GOOD - Track window IDs properly
struct AppState {
    popup_id: Option<window::Id>,
}

Message::OpenPopup => {
    let id = window::Id::unique();
    self.state.popup_id = Some(id);
    // Create popup with this ID
}

Message::ClosePopup => {
    if let Some(id) = self.state.popup_id.take() {
        return destroy_popup(id);
    }
    Command::none()
}
```

## Internationalization (i18n)

### Use i18n-embed

```rust
use i18n_embed::{
    fluent::{fluent_language_loader, FluentLanguageLoader},
    DesktopLanguageRequester,
};

// In main or app init
let localizer = fluent_language_loader!();
let requested_languages = DesktopLanguageRequester::requested_languages();
localizer.select(&requested_languages).unwrap();

// In UI
use i18n_embed_fl::fl;

fn view(&self) -> Element<Message> {
    column![
        text(fl!("welcome-message")),
        button::standard(fl!("connect-button"))
            .on_press(Message::Connect),
    ]
    .into()
}
```

## Accessibility

### 1. Semantic Widgets

Use appropriate widget types:

```rust
// ✅ GOOD - Semantic meaning clear
button::standard("Save")
    .on_press(Message::Save)

// ✅ GOOD - Proper roles
checkbox("Enable notifications", self.notifications_enabled)
    .on_toggle(Message::ToggleNotifications)
```

### 2. Keyboard Navigation

Ensure keyboard accessibility:

```rust
fn subscription(&self) -> Subscription<Message> {
    cosmic::keyboard::on_key_press(|key, modifiers| {
        match (key, modifiers) {
            (keyboard::Key::Named(Named::Escape), _) => {
                Some(Message::Close)
            }
            (keyboard::Key::Named(Named::Enter), _) => {
                Some(Message::Confirm)
            }
            _ => None,
        }
    })
}
```

## Build and Compilation

### Optimize Build Times

```toml
# In Cargo.toml
[profile.dev]
opt-level = 1           # Some optimization in debug
debug = true
split-debuginfo = "unpacked"

[profile.release]
opt-level = 3
lto = true              # Link time optimization
codegen-units = 1       # Better optimization
strip = true            # Remove debug symbols

# Use sccache
# In shell: export RUSTC_WRAPPER=sccache

# Or mold linker
# In .cargo/config.toml
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=mold"]
```

## Deployment Best Practices

### 1. Desktop Entry

Complete desktop entry:

```desktop
[Desktop Entry]
Type=Application
Name=My App
Comment=Description of app
Exec=my-app
Icon=my-app
Categories=COSMIC;Utility;
Keywords=keyword1;keyword2;
Terminal=false
StartupNotify=true

# For applets, add:
NoDisplay=true
X-CosmicApplet=true
X-CosmicHoverPopup=Auto
X-OverflowPriority=10
```

### 2. Versioning

Follow semantic versioning:

```toml
[package]
version = "0.1.0"  # Major.Minor.Patch

# Update for:
# Major: Breaking changes
# Minor: New features, backwards compatible
# Patch: Bug fixes
```

## Summary Checklist

✅ **Code Quality:**
- [ ] No `unwrap()` or `expect()` in production code
- [ ] All errors logged with tracing
- [ ] Proper async/await usage
- [ ] State management is clear
- [ ] Types are well-documented

✅ **UI/UX:**
- [ ] Uses COSMIC widgets
- [ ] Loading states shown
- [ ] Empty states handled
- [ ] Errors are user-friendly
- [ ] Responsive to panel size (applets)

✅ **Performance:**
- [ ] No blocking operations
- [ ] Icons cached
- [ ] Minimal allocations
- [ ] Tested with many items

✅ **Integration:**
- [ ] cosmic-config for persistence
- [ ] Respects system theme
- [ ] i18n implemented
- [ ] Desktop entry correct

✅ **Testing:**
- [ ] Unit tests for logic
- [ ] Integration tests for flows
- [ ] Tested in dark/light mode
- [ ] Keyboard navigation works

## Resources

- [libcosmic Book](https://pop-os.github.io/libcosmic-book/)
- [libcosmic API Docs](https://pop-os.github.io/libcosmic/cosmic/)
- [COSMIC App Template](https://github.com/pop-os/cosmic-app-template)
- [COSMIC Applet Template](https://github.com/pop-os/cosmic-applet-template)
- [COSMIC Applets Source](https://github.com/pop-os/cosmic-applets)
- [libcosmic Examples](https://github.com/pop-os/libcosmic/tree/master/examples)
