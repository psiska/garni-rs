use crate::domain::config::{ALL_PARAM, Channel, Conversion, Field, PrometheusParam};
use crate::domain::model::GarniRecord;

fn convert_f_to_c(f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0
}

fn convert_mph_to_ms(mph: f32) -> f32 {
    mph * 0.44704
}

fn convert_inch_to_mm(inches: f32) -> f32 {
    inches * 25.4
}

fn get_channel_temp(channel: &Channel, gr: GarniRecord) -> Option<f32> {
    match channel {
        Channel::Indoor => Some(gr.indoor_temperature_in_f),
        Channel::Outdoor => Some(gr.outdoor_temperature_in_f),
        Channel::Channel1 => gr.channel_1_temperature_in_f,
        Channel::Channel2 => gr.channel_2_temperature_in_f,
        Channel::Channel3 => gr.channel_3_temperature_in_f,
        Channel::Channel4 => gr.channel_4_temperature_in_f,
        Channel::Channel5 => gr.channel_5_temperature_in_f,
        Channel::Channel6 => gr.channel_6_temperature_in_f,
        Channel::Channel7 => gr.channel_7_temperature_in_f,
        Channel::Channel8 => gr.channel_8_temperature_in_f,
    }
}

fn get_channel_humidity(channel: &Channel, gr: GarniRecord) -> Option<f32> {
    match channel {
        Channel::Indoor => Some(gr.indoor_humidity_in_percent),
        Channel::Outdoor => Some(gr.outdoor_humidity_in_percent),
        Channel::Channel1 => gr.channel_1_humidity_in_percent,
        Channel::Channel2 => gr.channel_2_humidity_in_percent,
        Channel::Channel3 => gr.channel_3_humidity_in_percent,
        Channel::Channel4 => gr.channel_4_humidity_in_percent,
        Channel::Channel5 => gr.channel_5_humidity_in_percent,
        Channel::Channel6 => gr.channel_6_humidity_in_percent,
        Channel::Channel7 => gr.channel_7_humidity_in_percent,
        Channel::Channel8 => gr.channel_8_humidity_in_percent,
    }
}

fn get_value(pp: &PrometheusParam, gr: GarniRecord) -> Option<f32> {
    match pp.from_field {
        Field::Baromin => Some(gr.barometric_pressure),
        Field::TempF(ch) => get_channel_temp(ch, gr),
        Field::DewPtF => Some(gr.dew_point_in_f),
        Field::Humidity(ch) => get_channel_humidity(ch, gr),
        Field::WindSpeedMph => Some(gr.wind_speed_in_mph),
        Field::WindGustMph => Some(gr.wind_gust_in_mph),
        Field::WindDir => Some(gr.wind_direction_in_degrees as f32),
        Field::RainInInches => Some(gr.rain_in_inches),
        Field::DailyRainInInches => Some(gr.daily_rain_in_inches),
        Field::SolarRadiation => Some(gr.solar_radiation),
        Field::UVIndex => Some(gr.uv_index),
    }
}

pub fn record_to_prometheus_text(gr: GarniRecord) -> String {
    ALL_PARAM
        .iter()
        .map(|pp| {
            let value = get_value(pp, gr.clone());
            let v = if *pp.convert != Conversion::None && value.is_some() {
                match pp.convert {
                    Conversion::None => value.unwrap(),
                    Conversion::FToC => convert_f_to_c(value.unwrap()),
                    Conversion::InchToMM => convert_inch_to_mm(value.unwrap()),
                    Conversion::MPHToMS => convert_mph_to_ms(value.unwrap()),
                }
            } else {
                value.unwrap_or(0.0)
            };
            let entry = vec![
                format!("# HELP {param} {help}", param = pp.name, help = pp.help),
                format!(
                    "# TYPE {param} {tpe}",
                    param = pp.name,
                    tpe = pp.p_type.tpe_string()
                ),
                format!("{param} {value}", param = pp.name, value = v),
            ];
            entry.join("\n")
        })
        .collect::<Vec<String>>()
        .join("\n")
}
