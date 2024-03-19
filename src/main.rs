use cosmic::app::{Command, Core, Settings};
use cosmic::iced::Length;
use cosmic::widget::{column, container, scrollable};
use cosmic::{executor, ApplicationExt, Apply, Element};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::default()
        .antialiasing(true)
        .client_decorations(true)
        .debug(false);
        
    cosmic::app::run::<App>(settings, ())?;
    
    Ok(())
}

#[derive(Clone, Debug)]
pub enum Message {
}

pub struct App {
    core: Core,
}

impl cosmic::Application for App {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    
    const APP_ID: &'static str = "com.jwestall.CosmicWeather";
    
    fn core(&self) -> &Core {
        &self.core
    }
    
    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }
    
    fn init(core: Core, _input: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut app = App {
            core,
        };
        let command = app.update_title();
        
        (app, command)
    }
    
    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }
    
    fn view(&self) -> Element<Self::Message> {
        let page_view = cosmic::widget::text("App is under construction!");
        
        column()
            .spacing(24)
            .push(container(page_view).width(Length::Fill))
            .apply(container)
            .width(Length::Fill)
            .max_width(1000)
            .apply(container)
            .center_x()
            .width(Length::Fill)
            .apply(scrollable)
            .into()
    }
}

impl App where Self: cosmic::Application, {
    fn update_title(&mut self) -> Command<Message> {
        let window_title = format!("Cosmic Weather");
        
        self.set_header_title(window_title.clone());
        self.set_window_title(window_title, cosmic::iced::window::Id::MAIN)
    }
}
