use chrono::Local;
use cosmic::prelude::CollectionWidget;
use cosmic::widget;
use cosmic::Element;

use crate::app::config::PressureUnits;
use crate::app::config::SpeedUnits;
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
            TimeFmt::TwelveHr => self
                .weather_data
                .properties
                .meta
                .updated_at
                .format("%_I:%M %p")
                .to_string(),
            TimeFmt::TwentyFourHr => self
                .weather_data
                .properties
                .meta
                .updated_at
                .format("%_H:%M")
                .to_string(),
        };

        let pressure_units = match self.config.pressure_units {
            PressureUnits::Hectopascal => "hPa".to_string(),
            PressureUnits::Bar => "bar".to_string(),
            PressureUnits::Kilopascal => "kPa".to_string(),
            PressureUnits::Psi => "psi".to_string(),
        };

        let speed_units = match self.config.speed_units {
            SpeedUnits::MetersPerSecond => "m/s".to_string(),
            SpeedUnits::MilesPerHour => "mph".to_string(),
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
                                    widget::text(format!(
                                        "{}°",
                                        self.set_temp_units(air_temperature)
                                    ))
                                    .size(42)
                                    .style(cosmic::style::Text::Accent)
                                },
                            )),
                    ),
            )
            .push(
                widget::settings::view_section("Details")
                    .add(widget::settings::item(
                        "Air Pressure",
                        widget::text(format!(
                            "{:.1} {}",
                            self.calculate_pressure_units(
                                data.instant
                                    .details
                                    .air_pressure_at_sea_level
                                    .unwrap_or(0.0)
                            ),
                            pressure_units
                        )),
                    ))
                    .add(widget::settings::item(
                        "Cloud Area",
                        widget::text(format!(
                            "{} %",
                            data.instant.details.cloud_area_fraction.unwrap_or(0.0)
                        )),
                    ))
                    .add(widget::settings::item(
                        "Relative Hummidity",
                        widget::text(format!(
                            "{} %",
                            data.instant.details.relative_humidity.unwrap_or(0.0)
                        )),
                    ))
                    .add(widget::settings::item(
                        "Wind Direction",
                        widget::text(format!(
                            "{} °",
                            data.instant.details.wind_from_direction.unwrap_or(0.0)
                        )),
                    ))
                    .add(widget::settings::item(
                        "Wind Speed",
                        widget::text(format!(
                            "{:.1} {}",
                            self.calculate_speed_units(
                                data.instant.details.wind_speed.unwrap_or(0.0)
                            ),
                            speed_units
                        )),
                    )),
            )
            .push(widget::text(format!("Last updated: {}", last_updated)))
            .push(widget::text(
                "Weather data from the Norwegian Meteorological Institute",
            ));

        column.into()
    }

    fn calculate_pressure_units(&self, value: f64) -> f64 {
        match self.config.pressure_units {
            PressureUnits::Hectopascal => value,
            PressureUnits::Bar => value * 0.001 as f64,
            PressureUnits::Kilopascal => value * 0.1 as f64,
            PressureUnits::Psi => value * 0.0145037738 as f64,
        }
    }

    fn calculate_speed_units(&self, value: f64) -> f64 {
        match self.config.speed_units {
            SpeedUnits::MetersPerSecond => value,
            SpeedUnits::MilesPerHour => value / 0.44704 as f64,
        }
    }
}
