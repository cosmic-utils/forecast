use chrono::Local;
use cosmic::iced::Alignment;
use cosmic::iced_widget::scrollable::Direction;
use cosmic::iced_widget::scrollable::Properties;
use cosmic::prelude::CollectionWidget;
use cosmic::widget;
use cosmic::Element;

use crate::app::{App, Message};
use crate::model::weather::WeatherData;

impl App
where
    Self: cosmic::Application,
{
    pub fn view_hourly_forecast(&self) -> Element<Message> {
        let current_time = Local::now();
        let location = self.config.location.clone();
        let spacing = cosmic::theme::active().cosmic().spacing;
        let data = &self
            .weather_data
            .properties
            .timeseries
            .iter()
            .min_by_key(|timeseries| (timeseries.time - current_time).num_seconds().abs())
            .map(|ts| ts.data.clone())
            .unwrap_or_default();

        let last_updated = self
            .weather_data
            .properties
            .meta
            .updated_at
            .format("%H:%M")
            .to_string();

        let timeseries: Vec<Element<Message>> =
            self.weather_data
                .properties
                .timeseries
                .iter()
                .filter(|timeseries| timeseries.time >= current_time)
                .map(|ts| {
                    widget::column()
                        .align_items(Alignment::Center)
                        .padding(spacing.space_xs)
                        .spacing(spacing.space_xs)
                        .push(widget::text(ts.time.format("%H:%M").to_string()))
                        .push_maybe(ts.data.next_1_hours.as_ref().map(|next_1_hours| {
                            let symbol = next_1_hours.summary.symbol_code.clone();
                            widget::icon(WeatherData::icon_handle(symbol)).size(50)
                        }))
                        .push_maybe(ts.data.instant.details.air_temperature.map(
                            |air_temperature| {
                                widget::text(format!("{}°", air_temperature))
                                    .size(24)
                                    .style(cosmic::style::Text::Accent)
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
                                    .unwrap_or(widget::text::title4("Unknown location")),
                            )
                            .push_maybe(data.instant.details.air_temperature.map(
                                |air_temperature| {
                                    widget::text(format!("{}°", air_temperature))
                                        .size(42)
                                        .style(cosmic::style::Text::Accent)
                                },
                            )),
                    ),
            )
            .push(
                widget::scrollable(widget::row::with_children(timeseries))
                    .direction(Direction::Horizontal(Properties::default())),
            )
            .push(widget::text(format!("Last updated: {}", last_updated)))
            .push(widget::text(
                "Weather data from the Norwegian Meteorological Institute",
            ));

        column.into()
    }
}
