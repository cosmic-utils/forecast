use config::{AppError, AppTheme, PressureUnits, SpeedUnits, TimeFmt, WeatherStateConfig};
use cosmic::cosmic_config::Update;
use cosmic::cosmic_theme::ThemeMode;
use cosmic::iced::keyboard::{Key, Modifiers};
use cosmic::surface;
use cosmic::widget::about::About;
use cosmic::widget::menu::action::MenuAction;
use cosmic::widget::menu::key_bind::KeyBind;
use cosmic::{
    app::{context_drawer::ContextDrawer, Core, Task},
    cosmic_config::{self, CosmicConfigEntry},
    cosmic_theme, executor,
    iced::{event, keyboard::Event as KeyEvent, window, Event, Length, Subscription},
    theme, widget, ApplicationExt, Apply, Element,
};
use serde::{Deserialize, Serialize};
use std::any::TypeId;
use std::collections::{HashMap, VecDeque};

pub mod config;
pub mod icon_cache;
pub mod key_bind;
pub mod localize;
pub mod menu;

use crate::app::config::{Units, WeatherConfig};
use crate::app::icon_cache::icon_cache_get;
use crate::app::key_bind::key_binds;
use crate::fl;
use crate::model::location::Location;
use crate::model::weather::{WeatherData, WeatherRequestStatus};

#[derive(Clone, Debug)]
pub enum Message {
    ChangeCity(String),
    DefaultCity,
    ChangeApiKey,
    Quit,
    SystemThemeModeChange,
    ToggleContextPage(ContextPage),
    Key(Modifiers, Key),
    Modifiers(Modifiers),
    UpdateWeatherConfigFromFilesystem(WeatherConfig),
    UpdateWeatherStateConfigFromFilesystem(WeatherStateConfig),
    DefaultPage(NavPage),
    Units(Units),
    TimeFmt(TimeFmt),
    PressureUnits(PressureUnits),
    SpeedUnits(SpeedUnits),
    AppTheme(AppTheme),
    DialogComplete((String, String)),
    DialogCancel,
    UpdateLocations(Vec<Location>),
    SetLocation(Location),
    SetWeatherData((WeatherStateConfig, WeatherRequestStatus)),
    ApiKeyUpdate(String),
    SaveApiKey,
    OpenWebsite(String),
    Error(AppError),
    CloseContextPage,
    Surface(surface::Action),
}

#[derive(Clone, Debug)]
pub struct Flags {
    pub handler: cosmic_config::Config,
    pub weather_config: WeatherConfig,
    pub weather_state_config: WeatherStateConfig,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContextPage {
    About,
    Settings,
    ChangeCity,
}

impl ContextPage {
    fn title(&self) -> String {
        match self {
            Self::About => fl!("about"),
            Self::Settings => fl!("settings"),
            Self::ChangeCity => fl!("change-city"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DialogPage {
    ApiKey,
    Info(AppError),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    About,
    Settings,
    ChangeCity,
    ChangeApiKey,
    Quit,
}

impl MenuAction for Action {
    type Message = Message;

    fn message(&self) -> Self::Message {
        match self {
            Action::About => Message::ToggleContextPage(ContextPage::About),
            Action::Settings => Message::ToggleContextPage(ContextPage::Settings),
            Action::ChangeCity => Message::ToggleContextPage(ContextPage::ChangeCity),
            Action::ChangeApiKey => Message::ChangeApiKey,
            Action::Quit => Message::Quit,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
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
    about: About,
    nav_model: widget::nav_bar::Model,
    key_binds: HashMap<KeyBind, Action>,
    modifiers: Modifiers,
    context_page: ContextPage,
    handler: cosmic_config::Config,
    pub weather_config: WeatherConfig,
    pub weather_state_config: WeatherStateConfig,
    city: String,
    app_locations: Vec<Location>,
    units: Vec<String>,
    timefmt: Vec<String>,
    pressure_units: Vec<String>,
    speed_units: Vec<String>,
    pages: Vec<String>,
    api_key: String,
    app_themes: Vec<String>,
    dialog_pages: VecDeque<DialogPage>,
}

impl cosmic::Application for App {
    type Executor = executor::Default;
    type Flags = Flags;
    type Message = Message;

    const APP_ID: &'static str = "com.jwestall.Forecast";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, flags: Self::Flags) -> (Self, Task<Self::Message>) {
        let mut nav_model = widget::nav_bar::Model::default();
        for &nav_page in NavPage::all() {
            let id = nav_model
                .insert()
                .icon(nav_page.icon())
                .text(nav_page.title())
                .data::<NavPage>(nav_page)
                .id();
            if nav_page == flags.weather_config.default_page {
                nav_model.activate(id);
            }
        }

        let mut commands = vec![];
        let app_units = vec![fl!("fahrenheit"), fl!("celsius")];
        let app_timefmt = vec![fl!("twelve-hr"), fl!("twenty-four-hr")];
        let app_pressure_units = vec![
            "hPa".to_string(),
            "bar".to_string(),
            "kPa".to_string(),
            "psi".to_string(),
            "mmHg".to_string(),
            "atm".to_string(),
        ];
        let app_speed_units = vec!["m/s".to_string(), "mph".to_string(), "km/h".to_string()];
        let app_themes = vec![fl!("light"), fl!("dark"), fl!("system")];
        let app_pages = vec![
            fl!("hourly-forecast"),
            fl!("daily-forecast"),
            fl!("details"),
        ];

        let about = About::default()
            .name(fl!("cosmic-ext-forecast"))
            .icon(cosmic::widget::icon::from_name(Self::APP_ID))
            .version(env!("CARGO_PKG_VERSION"))
            .author("Jacob Westall")
            .license("GPL-3.0")
            .links([
                (fl!("support"), "https://github.com/cosmic-utils/forecast"),
                (
                    fl!("repository"),
                    "https://github.com/cosmic-utils/forecast",
                ),
            ])
            .developers([("Jacob Westall".into(), "jacob@jwestall.com".into())]);

        let mut app = App {
            core,
            about,
            nav_model,
            key_binds: key_binds(),
            modifiers: Modifiers::empty(),
            context_page: ContextPage::Settings,
            handler: flags.handler,
            api_key: flags.weather_config.api_key.clone(),
            weather_config: flags.weather_config,
            weather_state_config: flags.weather_state_config,
            city: String::new(),
            app_locations: Vec::new(),
            units: app_units,
            timefmt: app_timefmt,
            pressure_units: app_pressure_units,
            speed_units: app_speed_units,
            pages: app_pages,
            app_themes,
            dialog_pages: VecDeque::new(),
        };

        // Default location to user location if empty
        // Denver if not found
        if app.weather_config.location.is_none() {
            let command = Task::done(cosmic::action::Action::App(Message::DefaultCity));

            commands.push(command);
        }

        // Do not open nav bar by default
        app.core.nav_bar_set_toggled(false);

        if app.weather_state_config.expires.is_none()
            || app.weather_state_config.expires <= Some(chrono::offset::Utc::now().into())
        {
            commands.push(app.update_weather_data());
        }

        let window_title = fl!("cosmic-ext-forecast").to_string();

        app.set_header_title(window_title.clone());

        if let Some(_id) = app.core.main_window_id() {
            commands.push(app.set_window_title(
                window_title,
                app.core().main_window_id().unwrap_or(window::Id::RESERVED),
            ));
        }

        (app, Task::batch(commands))
    }

    fn nav_model(&self) -> Option<&widget::nav_bar::Model> {
        Some(&self.nav_model)
    }

    fn context_drawer(&self) -> Option<ContextDrawer<'_, Self::Message>> {
        if !self.core.window.show_context {
            return None;
        }

        let title = self.context_page.title();

        Some(match self.context_page {
            ContextPage::About => cosmic::app::context_drawer::about(
                &self.about,
                |url| Message::OpenWebsite(url.to_string()),
                Message::CloseContextPage,
            )
            .title(title),
            ContextPage::Settings => cosmic::app::context_drawer::context_drawer(
                self.settings(),
                Message::CloseContextPage,
            )
            .title(title),
            ContextPage::ChangeCity => {
                let city = self.city.clone();

                let search = widget::text_input(fl!("search"), city)
                    .on_input(move |city| Message::ChangeCity(city))
                    .on_submit(|city| {
                        Message::DialogComplete((city.to_string(), self.api_key.clone()))
                    });

                cosmic::app::context_drawer::context_drawer(
                    self.changecity(),
                    Message::CloseContextPage,
                )
                .title(title)
                .header(search)
            }
        })
    }

    fn dialog(&self) -> Option<Element<'_, Message>> {
        let dialog_page = match self.dialog_pages.front() {
            Some(some) => some,
            None => return None,
        };

        let cosmic_theme::Spacing { space_xxs, .. } = theme::active().cosmic().spacing;

        let dialog = match dialog_page {
            DialogPage::ApiKey => {
                let content = widget::column(vec![])
                    .spacing(space_xxs)
                    .push(
                        widget::text_input(fl!("api-key"), self.api_key.as_str())
                            .on_input(Message::ApiKeyUpdate)
                            .on_submit(|_| Message::SaveApiKey),
                    )
                    .push(widget::text::body(fl!("provide-api-key")))
                    .push(widget::button::standard(fl!("create-account")).on_press(
                        Message::OpenWebsite("https://geocode.maps.co/join/".to_string()),
                    ));

                widget::dialog()
                    .title(fl!("api-key"))
                    .primary_action(
                        widget::button::suggested(fl!("save")).on_press(Message::SaveApiKey),
                    )
                    .secondary_action(
                        widget::button::standard(fl!("cancel")).on_press(Message::DialogCancel),
                    )
                    .control(content)
            }
            DialogPage::Info(app_errored) => {
                let mut content = widget::column::with_capacity(2).spacing(12);

                match app_errored {
                    AppError::Location(body) => {
                        let title = widget::text::title4("This request require API key");
                        content = content.push(title);
                        content = content.push(widget::text::body(body));
                        content = content.push(widget::text::body(fl!("edit-api-key-page")));
                    }
                    AppError::Weather(body) => {
                        let title = widget::text::title4("Fetching Weather");
                        content = content.push(title);
                        content = content.push(widget::text::body(body));
                    }
                }

                widget::dialog()
                    .secondary_action(
                        widget::button::standard(fl!("cancel")).on_press(Message::DialogCancel),
                    )
                    .control(content)
            }
        };

        Some(dialog.into())
    }

    fn header_start(&self) -> Vec<Element<'_, Self::Message>> {
        vec![menu::menu_bar(&self.core, &self.key_binds)]
    }

    fn on_nav_select(&mut self, id: widget::nav_bar::Id) -> Task<Message> {
        self.nav_model.activate(id);

        Task::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        struct ThemeSubscription;

        let subscriptions = vec![
            event::listen_with(|event, status, _win_id| match event {
                Event::Keyboard(KeyEvent::KeyPressed { key, modifiers, .. }) => match status {
                    event::Status::Ignored => Some(Message::Key(modifiers, key)),
                    event::Status::Captured => None,
                },
                Event::Keyboard(KeyEvent::ModifiersChanged(modifiers)) => {
                    Some(Message::Modifiers(modifiers))
                }
                _ => None,
            }),
            self.core()
                .watch_config::<WeatherConfig>(Self::APP_ID)
                .map(|update| {
                    for why in update.errors {
                        tracing::error!("app config error: {}", why);
                    }

                    Message::UpdateWeatherConfigFromFilesystem(update.config)
                }),
            self.core()
                .watch_config::<WeatherStateConfig>(Self::APP_ID)
                .map(|update| {
                    for why in update.errors {
                        tracing::error!("app config error: {}", why);
                    }

                    Message::UpdateWeatherStateConfigFromFilesystem(update.config)
                }),
            cosmic_config::config_subscription::<_, cosmic_theme::ThemeMode>(
                TypeId::of::<ThemeSubscription>(),
                cosmic_theme::THEME_MODE_ID.into(),
                cosmic_theme::ThemeMode::version(),
            )
            .map(|update: Update<ThemeMode>| {
                if !update.errors.is_empty() {
                    tracing::info!(
                        "errors loading theme mode {:?}: {:?}",
                        update.keys,
                        update.errors
                    );
                }
                Message::SystemThemeModeChange
            }),
        ];

        Subscription::batch(subscriptions)
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        let mut commands = vec![];
        match message {
            Message::ChangeCity(city) => {
                self.city = city;
            }
            Message::DefaultCity => {
                let user_city = match public_ip_address::perform_lookup(None) {
                    Ok(result) => match result.city {
                        Some(city) => city,
                        None => "Denver".to_string(),
                    },
                    Err(_) => "Denver".to_string(),
                };

                let command = Task::perform(
                    Location::get_location_data(user_city, self.api_key.clone()),
                    |data| match data {
                        Ok(data) => {
                            let Some(data) = data.first() else {
                                return cosmic::action::Action::App(Message::Error(
                                    AppError::Location("Could not get location data.".to_string()),
                                ));
                            };
                            cosmic::action::Action::App(Message::SetLocation(data.clone()))
                        }
                        Err(err) => cosmic::action::Action::App(Message::Error(
                            AppError::Location(err.to_string()),
                        )),
                    },
                );

                commands.push(command);
            }
            Message::ChangeApiKey => {
                // TODO
                self.dialog_pages.push_back(DialogPage::ApiKey)
            }
            Message::Quit => {
                if let Some(id) = self.core.main_window_id() {
                    return window::close(id);
                }
            }
            Message::ToggleContextPage(context_page) => {
                if self.context_page == context_page {
                    self.core.window.show_context = !self.core.window.show_context;
                } else {
                    self.context_page = context_page.clone();
                    self.core.window.show_context = true;
                }
            }
            Message::Key(modifiers, key) => {
                for (key_bind, action) in self.key_binds.iter() {
                    if key_bind.matches(modifiers, &key, None) {
                        return self.update(action.message());
                    }
                }
            }
            Message::Modifiers(modifiers) => {
                self.modifiers = modifiers;
            }
            Message::UpdateWeatherConfigFromFilesystem(config) => {
                if config != self.weather_config {
                    tracing::info!("Filesystem changes detected, updating config");
                    self.weather_config = config;
                }
            }
            Message::UpdateWeatherStateConfigFromFilesystem(weather_state_config) => {
                if weather_state_config != self.weather_state_config {
                    tracing::info!("Filesystem changes detected, updating weather state config");
                    self.weather_state_config = weather_state_config;
                }
            }
            Message::Units(units) => {
                if let Err(error) = self.weather_config.set_units(&self.handler, units) {
                    tracing::error!("failed to set: {}", error);
                }
            }
            Message::TimeFmt(timefmt) => {
                if let Err(error) = self.weather_config.set_timefmt(&self.handler, timefmt) {
                    tracing::error!("failed to set: {}", error);
                }
            }
            Message::PressureUnits(units) => {
                if let Err(error) = self.weather_config.set_pressure_units(&self.handler, units) {
                    tracing::error!("failed to set: {}", error);
                }
            }
            Message::SpeedUnits(speed) => {
                if let Err(error) = self.weather_config.set_speed_units(&self.handler, speed) {
                    tracing::error!("failed to set: {}", error);
                }
            }
            Message::AppTheme(theme) => {
                if let Err(error) = self.weather_config.set_app_theme(&self.handler, theme) {
                    tracing::error!("failed to set: {}", error);
                }
                commands.push(self.save_theme());
            }
            Message::DefaultPage(page) => {
                if let Err(error) = self.weather_config.set_default_page(&self.handler, page) {
                    tracing::error!("failed to set: {}", error);
                }
            }
            Message::DialogComplete((city, key)) => {
                let command =
                    Task::perform(Location::get_location_data(city, key), |data| match data {
                        Ok(data) => cosmic::action::Action::App(Message::UpdateLocations(data)),
                        Err(err) => cosmic::action::Action::App(Message::Error(
                            AppError::Location(err.to_string()),
                        )),
                    });

                commands.push(command);
            }
            Message::DialogCancel => {
                self.dialog_pages.pop_front();
            }
            Message::UpdateLocations(locations) => {
                self.app_locations = locations;
            }
            Message::SetLocation(location) => {
                if let Err(error) = self
                    .weather_config
                    .set_location(&self.handler, Some(location.display_name.clone()))
                {
                    tracing::error!("failed to set: {}", error);
                }

                if let Err(error) = self
                    .weather_config
                    .set_latitude(&self.handler, Some(location.lat.clone()))
                {
                    tracing::error!("failed to set: {}", error);
                }
                if let Err(error) = self
                    .weather_config
                    .set_longitude(&self.handler, Some(location.lon.clone()))
                {
                    tracing::error!("failed to set: {}", error);
                }
                commands.push(self.update_weather_data());

                self.core.window.show_context = !self.core.window.show_context;
            }
            Message::SetWeatherData((config_state, status)) => {
                match status {
                    WeatherRequestStatus::NotModified => {
                        self.weather_state_config.expires = config_state.expires;
                        self.weather_state_config.last_request = config_state.last_request;
                    }
                    WeatherRequestStatus::Other => {
                        self.weather_state_config = config_state;
                    }
                }

                return self.save_config_state();
            }
            Message::ApiKeyUpdate(key) => {
                self.api_key = key;
            }
            Message::SaveApiKey => {
                if let Err(error) = self
                    .weather_config
                    .set_api_key(&self.handler, self.api_key.clone())
                {
                    tracing::error!("failed to set: {}", error);
                }

                self.dialog_pages.pop_front();
            }
            Message::OpenWebsite(url) => {
                let _ = open::that(url);
            }
            Message::Error(err) => {
                eprintln!("Error: {}", err);
                self.dialog_pages.pop_front();
                self.dialog_pages.push_back(DialogPage::Info(err));
            }
            Message::SystemThemeModeChange => {
                commands.push(self.save_theme());
            }
            Message::CloseContextPage => {
                self.core.window.show_context = !self.core.window.show_context;
            }
            Message::Surface(a) => {
                return cosmic::task::message(cosmic::Action::Cosmic(
                    cosmic::app::Action::Surface(a),
                ));
            }
        }

        Task::batch(commands)
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let page_view = match self.nav_model.active_data::<NavPage>() {
            Some(NavPage::HourlyView) => self.view_hourly_forecast(),
            Some(NavPage::DailyView) => self.view_daily_forecast(),
            Some(NavPage::Details) => self.view_detail_forecast(),
            None => cosmic::widget::text("Unkown page selected.").into(),
        };

        widget::column(vec![])
            .spacing(24)
            .push(widget::container(page_view).width(Length::Fill))
            .apply(widget::container)
            .width(Length::Fill)
            .max_width(1000)
            .apply(widget::container)
            .center_x(Length::Fill)
            .width(Length::Fill)
            .apply(widget::scrollable)
            .into()
    }
}

impl App
where
    Self: cosmic::Application,
{
    fn save_config_state(&mut self) -> Task<Message> {
        if let Err(err) = self.weather_state_config.write_entry(&self.handler) {
            eprintln!("failed to save config: {}", err);
        }

        Task::none()
    }

    fn save_theme(&self) -> Task<Message> {
        cosmic::command::set_theme(self.weather_config.app_theme.theme())
    }

    fn update_weather_data(&self) -> Task<Message> {
        let last_request = self.weather_state_config.last_request.map(|lr| lr.to_utc());

        let (Some(lat), Some(long)) = (
            self.weather_config.latitude.as_ref(),
            self.weather_config.longitude.as_ref(),
        ) else {
            return Task::none();
        };

        let coords = (
            lat.parse::<f64>().expect("Error parsing string to f64"),
            long.parse::<f64>().expect("Error parsing string to f64"),
        );

        Task::perform(
            WeatherData::get_weather_data(coords, last_request),
            |data| match data {
                Ok(data) => {
                    let Some(data) = data else {
                        return cosmic::action::Action::App(Message::Error(AppError::Weather(
                            "Could not get weather data.".to_string(),
                        )));
                    };
                    cosmic::action::Action::App(Message::SetWeatherData(data.clone()))
                }
                Err(err) => {
                    cosmic::action::Action::App(Message::Error(AppError::Weather(err.to_string())))
                }
            },
        )
    }

    fn settings(&self) -> Element<'_, Message> {
        let selected_units = match self.weather_config.units {
            Units::Fahrenheit => 0,
            Units::Celsius => 1,
        };

        let selected_timefmt = match self.weather_config.timefmt {
            TimeFmt::TwelveHr => 0,
            TimeFmt::TwentyFourHr => 1,
        };

        let selected_pressure_units = match self.weather_config.pressure_units {
            PressureUnits::Hectopascal => 0,
            PressureUnits::Bar => 1,
            PressureUnits::Kilopascal => 2,
            PressureUnits::Psi => 3,
            PressureUnits::MmHg => 4,
            PressureUnits::Atmosphere => 5,
        };

        let selected_speed_units = match self.weather_config.speed_units {
            SpeedUnits::MetersPerSecond => 0,
            SpeedUnits::MilesPerHour => 1,
            SpeedUnits::KilometresPerHour => 2,
        };

        let selected_theme = match self.weather_config.app_theme {
            config::AppTheme::Light => 0,
            config::AppTheme::Dark => 1,
            config::AppTheme::System => 2,
        };

        let selected_page = match self.weather_config.default_page {
            NavPage::HourlyView => 0,
            NavPage::DailyView => 1,
            NavPage::Details => 2,
        };

        widget::settings::view_column(vec![
            widget::settings::section()
                .title(fl!("general"))
                .add(
                    widget::settings::item::builder(fl!("default-page")).control(widget::dropdown(
                        &self.pages,
                        Some(selected_page),
                        move |index| {
                            Message::DefaultPage(match index {
                                0 => NavPage::HourlyView,
                                1 => NavPage::DailyView,
                                _ => NavPage::Details,
                            })
                        },
                    )),
                )
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
                .add(
                    widget::settings::item::builder(fl!("time-format")).control(widget::dropdown(
                        &self.timefmt,
                        Some(selected_timefmt),
                        move |index| {
                            Message::TimeFmt(match index {
                                1 => TimeFmt::TwentyFourHr,
                                _ => TimeFmt::TwelveHr,
                            })
                        },
                    )),
                )
                .add(
                    widget::settings::item::builder("Pressure Units".to_string()).control(
                        widget::dropdown(
                            &self.pressure_units,
                            Some(selected_pressure_units),
                            move |index| {
                                Message::PressureUnits(match index {
                                    1 => PressureUnits::Bar,
                                    2 => PressureUnits::Kilopascal,
                                    3 => PressureUnits::Psi,
                                    4 => PressureUnits::MmHg,
                                    5 => PressureUnits::Atmosphere,
                                    _ => PressureUnits::Hectopascal,
                                })
                            },
                        ),
                    ),
                )
                .add(
                    widget::settings::item::builder("Speed Units".to_string()).control(
                        widget::dropdown(
                            &self.speed_units,
                            Some(selected_speed_units),
                            move |index| {
                                Message::SpeedUnits(match index {
                                    2 => SpeedUnits::KilometresPerHour,
                                    1 => SpeedUnits::MilesPerHour,
                                    _ => SpeedUnits::MetersPerSecond,
                                })
                            },
                        ),
                    ),
                )
                .into(),
            widget::settings::section()
                .title(fl!("appearance"))
                .add(
                    widget::settings::item::builder(fl!("theme")).control(widget::dropdown(
                        &self.app_themes,
                        Some(selected_theme),
                        move |index| {
                            Message::AppTheme(match index {
                                0 => AppTheme::Light,
                                1 => AppTheme::Dark,
                                _ => AppTheme::System,
                            })
                        },
                    )),
                )
                .into(),
        ])
        .into()
    }

    fn changecity(&self) -> Element<'_, Message> {
        let cosmic_theme::Spacing { space_xxs, .. } = theme::active().cosmic().spacing;

        let mut content = widget::column(vec![]).spacing(space_xxs);

        content = content.push(
            widget::settings::section().add(
                widget::settings::item_row(
                    vec![widget::text::body(fl!("current-location")).into()],
                )
                .apply(widget::container)
                .class(cosmic::theme::Container::List)
                .apply(widget::button::custom)
                .class(cosmic::theme::Button::Transparent)
                .on_press(Message::DefaultCity),
            ),
        );

        if !self.app_locations.is_empty() {
            let results: Vec<Element<Message>> = self
                .app_locations
                .iter()
                .map(|result| {
                    widget::settings::item_row(
                        vec![widget::text::body(&result.display_name).into()],
                    )
                    .apply(widget::container)
                    .class(cosmic::theme::Container::List)
                    .apply(widget::button::custom)
                    .class(cosmic::theme::Button::Transparent)
                    .on_press(Message::SetLocation(result.clone()))
                    .into()
                })
                .collect();

            content = content.push(widget::settings::section().extend(results));
        }

        content.into()
    }
}
