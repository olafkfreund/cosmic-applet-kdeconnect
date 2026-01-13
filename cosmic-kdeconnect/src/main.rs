use cosmic::{app::Core, Application};

fn main() -> cosmic::iced::Result {
    tracing_subscriber::fmt::init();
    cosmic::app::run::<KdeConnectApp>((), ())
}

struct KdeConnectApp {
    core: Core,
}

#[derive(Debug, Clone)]
enum Message {}

impl Application for KdeConnectApp {
    type Message = Message;
    type Executor = cosmic::executor::Default;
    type Flags = ();
    const APP_ID: &'static str = "com.system76.CosmicKdeConnect";

    fn init(core: Core, _flags: Self::Flags) -> (Self, cosmic::iced::Command<Message>) {
        (Self { core }, cosmic::iced::Command::none())
    }

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn update(&mut self, _message: Self::Message) -> cosmic::iced::Command<Self::Message> {
        cosmic::iced::Command::none()
    }

    fn view(&self) -> cosmic::Element<Self::Message> {
        cosmic::widget::text("KDE Connect").into()
    }
}
