use chrono::Local;
use cosmic::iced::Alignment;
use cosmic::iced_widget::scrollable::Direction;
use cosmic::iced_widget::scrollable::Scrollbar;
use cosmic::widget;
use cosmic::Element;

use crate::app::config::TimeFmt;
use crate::app::config::Units;
use crate::app::{App, Message};
use crate::fl;
use crate::model::weather::Timeseries;
use crate::model::weather::WeatherData;

impl App
where
    Self: cosmic::Application,
{
    pub fn view_hourly_forecast(&self) -> Element<'_, Message> {
        let current_time = Local::now();
        let location = self.config.location.clone();
        let spacing = cosmic::theme::active().cosmic().spacing;
        let Some(weather_data) = &self.config_state.weather_data else {
            return cosmic::widget::text(fl!("no_weather_data")).into();
        };
        let data = weather_data
            .properties
            .timeseries
            .iter()
            .min_by_key(|timeseries| (timeseries.time - current_time).num_seconds().abs())
            .map(|ts| ts.data.clone())
            .unwrap_or_default();

        let last_updated = match self.config.timefmt {
            TimeFmt::TwelveHr => weather_data
                .properties
                .meta
                .updated_at
                .format("%_I:%M %p")
                .to_string(),
            TimeFmt::TwentyFourHr => weather_data
                .properties
                .meta
                .updated_at
                .format("%_H:%M")
                .to_string(),
        };

        let timeseries: Vec<Element<Message>> =
            weather_data
                .properties
                .timeseries
                .iter()
                .filter(|timeseries| timeseries.time >= current_time)
                .map(|ts| {
                    widget::column()
                        .align_x(Alignment::Center)
                        .padding(spacing.space_xs)
                        .spacing(spacing.space_xs)
                        .push(widget::text(self.format_time(ts)))
                        .push_maybe(ts.data.next_1_hours.as_ref().map(|next_1_hours| {
                            let symbol = next_1_hours.summary.symbol_code.clone();
                            widget::icon(WeatherData::icon_handle(symbol)).size(50)
                        }))
                        .push_maybe(ts.data.instant.details.air_temperature.map(
                            |air_temperature| {
                                widget::text(format!("{}°", self.set_temp_units(air_temperature)))
                                    .size(24)
                                    .class(cosmic::style::Text::Accent)
                            },
                        ))
                        .into()
                })
                .collect();

        let column = widget::column()
            .padding(spacing.space_xs)
            .spacing(spacing.space_xs)
            .push(
                widget::row()
                    .spacing(spacing.space_m)
                    .push_maybe(data.next_1_hours.as_ref().map(|next_1_hours| {
                        let symbol = next_1_hours.summary.symbol_code.clone();
                        widget::icon(WeatherData::icon_handle(symbol)).size(150)
                    }))
                    .push(
                        widget::column()
                            .spacing(spacing.space_xs)
                            .push(
                                location
                                    .map(widget::text::title4)
                                    .unwrap_or(widget::text::title4(fl!("unknown_location"))),
                            )
                            .push_maybe(data.instant.details.air_temperature.map(
                                |air_temperature| {
                                    widget::text(format!(
                                        "{}°",
                                        self.set_temp_units(air_temperature)
                                    ))
                                    .size(42)
                                    .class(cosmic::style::Text::Accent)
                                },
                            )),
                    ),
            )
            .push(
                widget::scrollable(widget::row::with_children(timeseries))
                    .direction(Direction::Horizontal(Scrollbar::default())),
            )
            .push(widget::text(format!(
                "{}: {}",
                fl!("last_updated"),
                last_updated
            )))
            .push(widget::text(fl!("data_from_metno")));

        column.into()
    }

    pub fn set_temp_units(&self, temp: f64) -> i64 {
        match self.config.units {
            Units::Fahrenheit => ((temp * (9_f64 / 5_f64)) + 32_f64) as i64,
            Units::Celsius => temp as i64,
        }
    }

    fn format_time(&self, ts: &Timeseries) -> String {
        match self.config.timefmt {
            TimeFmt::TwelveHr => ts.time.format("%_I:%M %p").to_string(),
            TimeFmt::TwentyFourHr => ts.time.format("%_H:%M").to_string(),
        }
    }
}
