use chrono::DateTime;
use chrono::Local;
use cosmic::iced::Alignment;
use cosmic::iced_widget::scrollable::Direction;
use cosmic::iced_widget::scrollable::Scrollbar;
use cosmic::widget;
use cosmic::Element;

use crate::app::config::TimeFmt;
use crate::app::{App, Message};
use crate::fl;
use crate::model::weather::Details;
use crate::model::weather::Timeseries;
use crate::model::weather::WeatherData;

impl App
where
    Self: cosmic::Application,
{
    pub fn view_daily_forecast(&self) -> Element<'_, Message> {
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

        let timeseries: Vec<Element<Message>> = weather_data
            .properties
            .timeseries
            .iter()
            .filter(|timeseries| self.check_time(timeseries, current_time))
            .map(|ts| {
                let data_6_hrs = match &ts.data.next_6_hours {
                    Some(data) => match &data.details {
                        Some(details) => details,
                        None => &Details::default(),
                    },
                    None => &Details::default(),
                };

                widget::column()
                    .align_x(Alignment::Center)
                    .padding(spacing.space_xs)
                    .spacing(spacing.space_xs)
                    .push(widget::text(self.format_date(ts)))
                    .push_maybe(ts.data.next_12_hours.as_ref().map(|next_12_hours| {
                        let symbol = next_12_hours.summary.symbol_code.clone();
                        widget::icon(WeatherData::icon_handle(symbol)).size(50)
                    }))
                    .push_maybe(data_6_hrs.air_temperature_max.map(|air_temperature_max| {
                        widget::text(format!("{}°", self.set_temp_units(air_temperature_max)))
                            .size(24)
                            .class(cosmic::style::Text::Accent)
                    }))
                    .push_maybe(data_6_hrs.air_temperature_min.map(|air_temperature_min| {
                        widget::text(format!("{}°", self.set_temp_units(air_temperature_min)))
                            .size(24)
                    }))
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

    fn format_date(&self, ts: &Timeseries) -> String {
        ts.time.format("%a").to_string()
    }

    fn check_time(&self, timeseries: &Timeseries, current_time: DateTime<Local>) -> bool {
        let timezone = current_time
            .format("%:::z")
            .to_string()
            .parse::<i64>()
            .unwrap_or_default();
        let timehour = timeseries
            .time
            .format("%H")
            .to_string()
            .parse::<i64>()
            .unwrap_or_default();
        let comparetime = if (12 + timezone) > 12 {
            timezone
        } else {
            timezone + 12
        };

        timeseries.time > current_time && timehour == comparetime
    }
}
