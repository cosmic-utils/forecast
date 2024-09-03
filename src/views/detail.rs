use chrono::Local;
use cosmic::iced::alignment::Horizontal;
use cosmic::iced::Length;
use cosmic::prelude::CollectionWidget;
use cosmic::widget;
use cosmic::Element;

use crate::app::config::TimeFmt;
use crate::app::{App, Message};
use crate::model::weather::WeatherData;

impl App
where
    Self: cosmic::Application,
{
    pub fn view_detail_forecast(&self) -> Element<Message> {
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

        let last_updated = match self.config.timefmt {
            TimeFmt::TwelveHr => {
                self
                .weather_data
                .properties
                .meta
                .updated_at
                .format("%_I:%M %p")
                .to_string()
            }
            TimeFmt::TwentyFourHr => {
                self
                .weather_data
                .properties
                .meta
                .updated_at
                .format("%_H:%M")
                .to_string()
            }
        };

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
                                    widget::text(format!("{}°", self.set_temp_units(air_temperature)))
                                        .size(42)
                                        .style(cosmic::style::Text::Accent)
                                },
                            )),
                    ),
            )
            .push(
                widget::settings::view_section("Details")
                    .add(
                    widget::row()
                        .push(
                            widget::text("Air Pressure")
                        )
                        .push_maybe(data.instant.details.air_pressure_at_sea_level.map(
                            |air_pressure| {
                                widget::text(format!("{}", air_pressure))
                                    .width(Length::Fill)
                                    .horizontal_alignment(Horizontal::Right)
                            }
                        )),
                    )
                    .add(
                    widget::row()
                        .push(
                            widget::text("Cloud Area")
                        )
                        .push_maybe(data.instant.details.cloud_area_fraction.map(
                            |cloud_area| {
                                widget::text(format!("{}", cloud_area))
                                    .width(Length::Fill)
                                    .horizontal_alignment(Horizontal::Right)
                            }
                        )),
                    )
                    .add(
                    widget::row()
                        .push(
                            widget::text("Relative Hummidity")
                        )
                        .push_maybe(data.instant.details.relative_humidity.map(
                            |relative_humidity| {
                                widget::text(format!("{}", relative_humidity))
                                    .width(Length::Fill)
                                    .horizontal_alignment(Horizontal::Right)
                            }
                        )),
                    )
                    .add(
                    widget::row()
                        .push(
                            widget::text("Wind Direction")
                        )
                        .push_maybe(data.instant.details.wind_from_direction.map(
                            |wind_direction| {
                                widget::text(format!("{}", wind_direction))
                                    .width(Length::Fill)
                                    .horizontal_alignment(Horizontal::Right)
                            }
                        )),
                    )
                    .add(
                    widget::row()
                        .push(
                            widget::text("Wind Speed")
                        )
                        .push_maybe(data.instant.details.wind_speed.map(
                            |wind_speed| {
                                widget::text(format!("{}", wind_speed))
                                    .width(Length::Fill)
                                    .horizontal_alignment(Horizontal::Right)
                            }
                        )),
                    )
                
            )
            .push(widget::text(format!("Last updated: {}", last_updated)))
            .push(widget::text(
                "Weather data from the Norwegian Meteorological Institute",
            ));

        column.into()
    }
}
