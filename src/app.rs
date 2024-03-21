use std::collections::HashMap;
use cosmic::app::{Command, Core};
use cosmic::iced::{Length, window};
use cosmic::widget::{column, container, scrollable};
use cosmic::widget::menu::key_bind::KeyBind;
use cosmic::{executor, ApplicationExt, Apply, Element};
use cosmic::widget::menu::action::MenuAction;
use cosmic::widget::segmented_button::Entity;

use crate::key_bind::key_binds;
use crate::menu;

#[derive(Clone, Debug)]
pub enum Message {
    AddCity,
    RemoveCity,
    Quit,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    AddCity,
    RemoveCity,
    Quit,
}

impl MenuAction for Action {
    type Message = Message;
    
    fn message(&self, _entity_op: Option<Entity>) -> Self::Message {
        match self {
            Action::AddCity => Message::AddCity,
            Action::RemoveCity => Message::RemoveCity,
            Action::Quit => Message::Quit,
        }
    }
}

pub struct App {
    core: Core,
    key_binds: HashMap<KeyBind, Action>,
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
            key_binds: key_binds(),
        };
        let command = app.update_title();
        
        (app, command)
    }
    
    fn header_start(&self) -> Vec<Element<Self::Message>> {
        vec![menu::menu_bar(&self.key_binds)]
    }
    
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::AddCity => {
                // TODO
            }
            Message::RemoveCity => {
                // TODO
            }
            Message::Quit => {
                return window::close(window::Id::MAIN);
            }
        }
    
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
