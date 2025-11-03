use chrono::Local;
use cosmic::widget;
use cosmic::Element;

use crate::app::config::PressureUnits;
use crate::app::config::SpeedUnits;
use crate::app::config::TimeFmt;
use crate::app::{App, Message};
use crate::fl;
use crate::model::weather::WeatherData;

impl App
where
    Self: cosmic::Application,
{
    pub fn view_detail_forecast(&self) -> Element<'_, Message> {
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

        let pressure_units = match self.config.pressure_units {
            PressureUnits::Hectopascal => "hPa".to_string(),
            PressureUnits::Bar => "bar".to_string(),
            PressureUnits::Kilopascal => "kPa".to_string(),
            PressureUnits::Psi => "psi".to_string(),
            PressureUnits::MmHg => "mmHg".to_string(),
            PressureUnits::Atmosphere => "atm".to_string(),
        };

        let speed_units = match self.config.speed_units {
            SpeedUnits::MetersPerSecond => "m/s".to_string(),
            SpeedUnits::MilesPerHour => "mph".to_string(),
            SpeedUnits::KilometresPerHour => "km/h".to_string(),
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
                widget::settings::section()
                    .title(fl!("details"))
                    .add(widget::settings::item(
                        fl!("air_pressure"),
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
                        fl!("cloud_area"),
                        widget::text(format!(
                            "{} %",
                            data.instant.details.cloud_area_fraction.unwrap_or(0.0)
                        )),
                    ))
                    .add(widget::settings::item(
                        fl!("relative_hummidity"),
                        widget::text(format!(
                            "{} %",
                            data.instant.details.relative_humidity.unwrap_or(0.0)
                        )),
                    ))
                    .add(widget::settings::item(
                        fl!("wind_direction"),
                        widget::text(format!(
                            "{} °",
                            data.instant.details.wind_from_direction.unwrap_or(0.0)
                        )),
                    ))
                    .add(widget::settings::item(
                        fl!("wind_speed"),
                        widget::text(format!(
                            "{:.1} {}",
                            self.calculate_speed_units(
                                data.instant.details.wind_speed.unwrap_or(0.0)
                            ),
                            speed_units
                        )),
                    )),
            )
            .push(widget::text(format!(
                "{}: {}",
                fl!("last_updated"),
                last_updated
            )))
            .push(widget::text(fl!("data_from_metno")));

        column.into()
    }

    fn calculate_pressure_units(&self, value: f64) -> f64 {
        match self.config.pressure_units {
            PressureUnits::Hectopascal => value,
            PressureUnits::Bar => value * 0.001_f64,
            PressureUnits::Kilopascal => value * 0.1_f64,
            PressureUnits::Psi => value * 0.0145037738_f64,
            PressureUnits::MmHg => value * 0.7500616_f64,
            PressureUnits::Atmosphere => value * 0.0009869233_f64,
        }
    }

    fn calculate_speed_units(&self, value: f64) -> f64 {
        match self.config.speed_units {
            SpeedUnits::MetersPerSecond => value,
            SpeedUnits::MilesPerHour => value / 0.44704_f64,
            SpeedUnits::KilometresPerHour => value * 3.6,
        }
    }
}
