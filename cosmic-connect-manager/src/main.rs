mod dbus_client;

use cosmic::{
    app::{Core, Task},
    iced::Size,
    widget::{text, container},
    Application, Element,
};

use dbus_client::DbusClient;

const APP_ID: &str = "com.system76.CosmicConnectManager";

fn main() -> cosmic::iced::Result {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let settings = cosmic::app::Settings::default()
        .size(Size::new(900.0, 700.0));

    cosmic::app::run::<CosmicConnectManager>(settings, ())
}

/// Messages that can be sent to the application
#[derive(Debug, Clone)]
pub enum Message {
    // Placeholder for future messages
    None,
}

/// Main application state
pub struct CosmicConnectManager {
    core: Core,
    dbus_client: Option<DbusClient>,
}

impl Application for CosmicConnectManager {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;
    const APP_ID: &'static str = APP_ID;

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
        let app = CosmicConnectManager {
            core,
            dbus_client: None,
        };

        (app, Task::none())
    }

    fn header_start(&self) -> Vec<Element<Self::Message>> {
        vec![]
    }

    fn view(&self) -> Element<Self::Message> {
        let content = container(
            text("COSMIC Connect Manager")
                .size(24)
        )
        .padding(20)
        .center_x(cosmic::iced::Length::Fill)
        .center_y(cosmic::iced::Length::Fill);

        Element::from(content)
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::None => Task::none(),
        }
    }
}
