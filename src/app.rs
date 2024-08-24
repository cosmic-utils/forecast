use cosmic::iced::keyboard::{Key, Modifiers};
use cosmic::widget::menu::action::MenuAction;
use cosmic::widget::menu::key_bind::KeyBind;
use cosmic::{
    app::{Command, Core},
    cosmic_config::{self, CosmicConfigEntry},
    cosmic_theme, executor,
    iced::{event, keyboard::Event as KeyEvent, window, Alignment, Event, Length, Subscription},
    theme, widget,
    widget::{column, container, nav_bar, scrollable},
    ApplicationExt, Apply, Element,
};
use std::collections::{HashMap, VecDeque};

pub mod config;
pub mod icon_cache;
pub mod key_bind;
pub mod localize;
pub mod menu;

use crate::app::config::{Config, Units};
use crate::app::icon_cache::icon_cache_get;
use crate::app::key_bind::key_binds;
use crate::fl;
use crate::model::location::Location;

#[derive(Clone, Debug)]
pub enum Message {
    ChangeCity,
    Quit,
    ToggleContextPage(ContextPage),
    LaunchUrl(String),
    Key(Modifiers, Key),
    Modifiers(Modifiers),
    Config(Config),
    Units(Units),
    DialogComplete(String),
    DialogCancel,
    DialogUpdate(DialogPage),
}

#[derive(Clone, Debug)]
pub struct Flags {
    pub config_handler: Option<cosmic_config::Config>,
    pub config: Config,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContextPage {
    About,
    Settings,
}

impl ContextPage {
    fn title(&self) -> String {
        match self {
            Self::About => fl!("about"),
            Self::Settings => fl!("settings"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DialogPage {
    Change(String),
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

    fn message(&self) -> Self::Message {
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
        &[Self::HourlyView, Self::DailyView, Self::Details]
    }

    fn title(&self) -> String {
        match self {
            Self::HourlyView => fl!("hourly-forecast"),
            Self::DailyView => fl!("daily-forecast"),
            Self::Details => fl!("details"),
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
    config_handler: Option<cosmic_config::Config>,
    pub config: Config,
    units: Vec<String>,
    dialog_pages: VecDeque<DialogPage>,
    dialog_page_text: widget::Id,
}

impl cosmic::Application for App {
    type Executor = executor::Default;
    type Flags = Flags;
    type Message = Message;

    const APP_ID: &'static str = "com.jwestall.Weather";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, flags: Self::Flags) -> (Self, Command<Self::Message>) {
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

        let app_units = vec![fl!("fahrenheit"), fl!("celsius")];

        let mut app = App {
            core,
            nav_model: nav_model,
            key_binds: key_binds(),
            modifiers: Modifiers::empty(),
            context_page: ContextPage::Settings,
            config_handler: flags.config_handler,
            config: flags.config,
            units: app_units,
            dialog_pages: VecDeque::new(),
            dialog_page_text: widget::Id::unique(),
        };

        // Default location to Denver if empty
        // TODO: Default to user location
        if app.config.location.is_empty() || app.config.location == "Unknown" {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    let data = &(Location::get_location_data("Denver")
                        .await
                        .unwrap()
                        .unwrap()[0]);

                    app.config.location = data.display_name.clone();
                    app.config.lon = data.lon.clone();
                    app.config.lat = data.lat.clone();
                });
        }

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

    fn dialog(&self) -> Option<Element<Message>> {
        let dialog_page = match self.dialog_pages.front() {
            Some(some) => some,
            None => return None,
        };

        let cosmic_theme::Spacing { space_xxs, .. } = theme::active().cosmic().spacing;

        let dialog = match dialog_page {
            DialogPage::Change(city) => widget::dialog(fl!("change-city"))
                .primary_action(
                    widget::button::suggested(fl!("save"))
                        .on_press_maybe(Some(Message::DialogComplete(city.to_string()))),
                )
                .secondary_action(
                    widget::button::standard(fl!("cancel")).on_press(Message::DialogCancel),
                )
                .control(
                    widget::column::with_children(vec![widget::text_input(
                        fl!("search"),
                        city.as_str(),
                    )
                    .id(self.dialog_page_text.clone())
                    .on_input(move |city| Message::DialogUpdate(DialogPage::Change(city)))
                    .into()])
                    .spacing(space_xxs),
                ),
        };

        Some(dialog.into())
    }

    fn header_start(&self) -> Vec<Element<Self::Message>> {
        vec![menu::menu_bar(&self.key_binds)]
    }

    fn on_nav_select(&mut self, id: nav_bar::Id) -> Command<Message> {
        self.nav_model.activate(id);

        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let subscriptions = vec![event::listen_with(|event, status| match event {
            Event::Keyboard(KeyEvent::KeyPressed { key, modifiers, .. }) => match status {
                event::Status::Ignored => Some(Message::Key(modifiers, key)),
                event::Status::Captured => None,
            },
            Event::Keyboard(KeyEvent::ModifiersChanged(modifiers)) => {
                Some(Message::Modifiers(modifiers))
            }
            _ => None,
        })];

        Subscription::batch(subscriptions)
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::ChangeCity => {
                // TODO
                self.dialog_pages
                    .push_back(DialogPage::Change(String::new()));
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
            },
            Message::Key(modifiers, key) => {
                for (key_bind, action) in self.key_binds.iter() {
                    if key_bind.matches(modifiers, &key) {
                        return self.update(action.message());
                    }
                }
            }
            Message::Modifiers(modifiers) => {
                self.modifiers = modifiers;
            }
            Message::Config(config) => {
                if config != self.config {
                    log::info!("Updating config");
                    self.config = config;
                }
            }
            Message::Units(units) => {
                self.config.units = units;
                return self.save_config();
            }
            Message::DialogComplete(city) => {
                // TODO: Add functaionality
                tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(async {
                        let data = &(Location::get_location_data(city.as_str())
                            .await
                            .unwrap()
                            .unwrap()[0]);

                        self.config.location = data.display_name.clone();
                        self.config.lat = data.lat.clone();
                        self.config.lon = data.lon.clone();
                    });

                self.save_config();
                self.dialog_pages.pop_front();
            }
            Message::DialogCancel => {
                self.dialog_pages.pop_front();
            }
            Message::DialogUpdate(dialog_page) => {
                self.dialog_pages[0] = dialog_page;
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let page_view = match self.nav_model.active_data::<NavPage>() {
            Some(NavPage::HourlyView) => self.view_hourly_forecast(),
            Some(NavPage::DailyView) => self.view_daily_forecast(),
            Some(NavPage::Details) => self.view_detail_forecast(),
            None => cosmic::widget::text("Unkown page selected.").into(),
        };

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

impl App
where
    Self: cosmic::Application,
{
    fn update_title(&mut self) -> Command<Message> {
        let window_title = format!("{}", fl!("cosmic-ext-weather"));

        self.set_header_title(window_title.clone());
        self.set_window_title(window_title)
    }

    fn save_config(&mut self) -> Command<Message> {
        if let Some(ref config_handler) = self.config_handler {
            if let Err(err) = self.config.write_entry(config_handler) {
                log::error!("failed to save config: {}", err);
            }
        }

        Command::none()
    }

    fn about(&self) -> Element<Message> {
        let spacing = theme::active().cosmic().spacing;
        let repository = "https://github.com/jwestall/cosmic-weather";
        let hash = env!("VERGEN_GIT_SHA");
        let short_hash: String = hash.chars().take(7).collect();
        let date = env!("VERGEN_GIT_COMMIT_DATE");
        widget::column::with_children(vec![
            widget::svg(widget::svg::Handle::from_memory(
                &include_bytes!("../res/icons/hicolor/scalable/apps/com.jwestall.Weather.svg")[..],
            ))
            .into(),
            widget::text::title3(fl!("cosmic-ext-weather")).into(),
            widget::button::link(repository)
                .on_press(Message::LaunchUrl(repository.to_string()))
                .padding(spacing.space_none)
                .into(),
            widget::button::link(fl!(
                "git-description",
                hash = short_hash.as_str(),
                date = date
            ))
            .on_press(Message::LaunchUrl(format!("{repository}/commits/{hash}")))
            .padding(spacing.space_none)
            .into(),
        ])
        .align_items(Alignment::Center)
        .spacing(spacing.space_xxs)
        .width(Length::Fill)
        .into()
    }

    fn settings(&self) -> Element<Message> {
        let selected_units = match self.config.units {
            Units::Fahrenheit => 0,
            Units::Celsius => 1,
        };

        widget::settings::view_column(vec![widget::settings::view_section(fl!("general"))
            .add(
                widget::settings::item::builder(fl!("units")).control(widget::dropdown(
                    &self.units,
                    Some(selected_units),
                    move |index| {
                        Message::Units(match index {
                            1 => Units::Celsius,
                            _ => Units::Fahrenheit,
                        })
                    },
                )),
            )
            .into()])
        .into()
    }
}
