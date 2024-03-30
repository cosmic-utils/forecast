use std::collections::HashMap;
use cosmic::{
    executor, cosmic_theme, theme, widget, ApplicationExt, Apply, Element,
    app::{Command, Core},
    iced::{event, keyboard::Event as KeyEvent, Alignment, Length, Subscription, window, Event},
    widget::{column, container, nav_bar, scrollable},
};
use cosmic::iced::keyboard::{Key, Modifiers};
use cosmic::widget::menu::key_bind::KeyBind;
use cosmic::widget::menu::action::MenuAction;
use cosmic::widget::segmented_button::Entity;

use crate::key_bind::key_binds;
use crate::menu;
use crate::icon_cache::icon_cache_get;

#[derive(Clone, Debug)]
pub enum Message {
    ChangeCity,
    Quit,
    ToggleContextPage(ContextPage),
    LaunchUrl(String),
    Key(Modifiers, Key),
    Modifiers(Modifiers),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContextPage {
    About,
    Settings
}

impl ContextPage {
    fn title(&self) -> String {
        match self {
            Self::About => "About".to_string(),
            Self::Settings => "Settings".to_string(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    About,
    Settings,
    ChangeCity,
    Quit,
}

impl MenuAction for Action {
    type Message = Message;
    
    fn message(&self, _entity_op: Option<Entity>) -> Self::Message {
        match self {
            Action::About => Message::ToggleContextPage(ContextPage::About),
            Action::Settings => Message::ToggleContextPage(ContextPage::Settings),
            Action::ChangeCity => Message::ChangeCity,
            Action::Quit => Message::Quit,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NavPage {
    HourlyView,
    DailyView,
    Details,
}

impl NavPage {
    fn all() -> &'static [Self] {
        &[
            Self::HourlyView,
            Self::DailyView,
            Self::Details,
        ]
    }
    
    fn title(&self) -> String {
        match self {
            Self::HourlyView => "Hourly Forecast".to_owned(),
            Self::DailyView => "Daily Forecast".to_owned(),
            Self::Details => "Details".to_owned(),
        }
    }
    
    fn icon(&self) -> widget::icon::Icon {
        match self {
            Self::HourlyView => icon_cache_get("view-hourly", 16),
            Self::DailyView => icon_cache_get("view-daily", 16),
            Self::Details => icon_cache_get("view-detail", 16),
        }
    }
}

pub struct App {
    core: Core,
    nav_model: nav_bar::Model,
    key_binds: HashMap<KeyBind, Action>,
    modifiers: Modifiers,
    context_page: ContextPage,
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
        let mut nav_model = nav_bar::Model::default();
        for &nav_page in NavPage::all() {
            let id = nav_model
                .insert()
                .icon(nav_page.icon())
                .text(nav_page.title())
                .data::<NavPage>(nav_page)
                .id();
            if nav_page == NavPage::HourlyView {
                nav_model.activate(id);
            }
        }
        
        let mut app = App {
            core,
            nav_model: nav_model,
            key_binds: key_binds(),
            modifiers: Modifiers::empty(),
            context_page: ContextPage::Settings,
        };
        
        // Do not open nav bar by default
        app.core.nav_bar_set_toggled(false);
        
        let command = app.update_title();
        
        (app, command)
    }
    
    fn nav_model(&self) -> Option<&nav_bar::Model> {
        Some(&self.nav_model)
    }
    
    fn context_drawer(&self) -> Option<Element<Message>> {
        if !self.core.window.show_context {
            return None;
        }
        
        Some(match self.context_page {
            ContextPage::About => self.about(),
            ContextPage::Settings => self.settings(),
        })
    }
    
    fn header_start(&self) -> Vec<Element<Self::Message>> {
        vec![menu::menu_bar(&self.key_binds)]
    }
    
    fn on_nav_select(&mut self, id: nav_bar::Id) -> Command<Message> {
        self.nav_model.activate(id);
        
        Command::none()
    }
    
    fn subscription(&self) -> Subscription<Self::Message> {
        let mut subscriptions = vec![
            event::listen_with(|event, status| match event {
                Event::Keyboard(KeyEvent::KeyPressed { key, modifiers, .. }) => match status {
                    event::Status::Ignored => Some(Message::Key(modifiers, key)),
                    event::Status::Captured => None,
                },
                Event::Keyboard(KeyEvent::ModifiersChanged(modifiers)) => {
                    Some(Message::Modifiers(modifiers))
                }
                _ => None,
            }),
        ];
        
        Subscription::batch(subscriptions)
    }
    
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::ChangeCity => {
                // TODO
            }
            Message::Quit => {
                return window::close(window::Id::MAIN);
            }
            Message::ToggleContextPage(context_page) => {
                if self.context_page == context_page {
                    self.core.window.show_context = !self.core.window.show_context;
                } else {
                    self.context_page = context_page.clone();
                    self.core.window.show_context = true;
                }
                self.set_context_title(context_page.clone().title());
            }
            Message::LaunchUrl(url) => match open::that_detached(&url) {
                Ok(()) => {}
                Err(err) => {
                    log::warn!("failed to open {:?}: {}", url, err);
                }
            }
            Message::Key(modifiers, key) => {
                for (key_bind, action) in self.key_binds.iter() {
                    if key_bind.matches(modifiers, &key) {
                        return self.update(action.message(None));
                    }
                }
            }
            Message::Modifiers(modifiers) => {
                self.modifiers = modifiers;
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
    
    fn about(&self) -> Element<Message> {
        let cosmic_theme::Spacing { space_xxs, .. } = theme::active().cosmic().spacing;
        let repo = "https://github.com/jwestall/cosmic-weather";
        
        widget::column::with_children(vec![
            widget::text::title3("COSMIC Weather").into(),
            widget::button::link(repo)
                .on_press(Message::LaunchUrl(repo.to_string()))
                .padding(0)
                .into()
        ])
        .align_items(Alignment::Center)
        .spacing(space_xxs)
        .width(Length::Fill)
        .into()
    }
    
    fn settings(&self) -> Element<Message> {
        let cosmic_theme::Spacing { space_xxs, .. } = theme::active().cosmic().spacing;
        
        widget::column::with_children(vec![
            widget::text::title3("Settings").into(),
        ])
        .align_items(Alignment::Center)
        .spacing(space_xxs)
        .width(Length::Fill)
        .into()
    }
}
