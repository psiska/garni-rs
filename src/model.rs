use serde::Deserialize;
use std::sync::{Arc, RwLock};

// /weatherstation/updateweatherstation.php?ID=H50&PASSWORD=&action=updateraww&realtime=1
// &rtfreq=5&dateutc=now
// &baromin=30.04&tempf=45.1&dewptf=43.5&humidity=94
// &windspeedmph=0.0&windgustmph=0.0&winddir=192
// &rainin=0.0&dailyrainin=0.0
// &solarradiation=37.05&UV=0.0
// &indoortempf=72.5&indoorhumidity=40
// &soiltempf=70.8&soilmoisture=40
// &soiltemp2f=63.5&soilmoisture2=58

#[derive(Deserialize, Default, Clone)]
pub(crate) struct GarniRecord {
    #[serde(rename = "ID")]
    pub(crate) id: String,
    #[serde(rename = "PASSWORD")]
    password: Option<String>,

    action: String,
    realtime: i32,
    rtfreq: i32,
    dateutc: String,

    // Actual data
    #[serde(rename = "baromin")]
    pub(crate) barometric_pressure: f32,
    #[serde(rename = "tempf")]
    pub(crate) outdoor_temperature_in_f: f32,
    #[serde(rename = "dewptf")]
    pub(crate) dew_point_in_f: f32,
    #[serde(rename = "humidity")]
    pub(crate) outdoor_humidity_in_percent: f32,

    #[serde(rename = "windspeedmph")]
    pub(crate) wind_speed_in_mph: f32,
    #[serde(rename = "windgustmph")]
    pub(crate) wind_gust_in_mph: f32,
    #[serde(rename = "winddir")]
    pub(crate) wind_direction_in_degrees: i32,

    #[serde(rename = "rainin")]
    pub(crate) rain_in_inches: f32,
    #[serde(rename = "dailyrainin")]
    pub(crate) daily_rain_in_inches: f32,

    #[serde(rename = "solarradiation")]
    pub(crate) solar_radiation: f32,
    #[serde(rename = "UV")]
    pub(crate) uv_index: f32,

    #[serde(rename = "indoortempf")]
    pub(crate) indoor_temperature_in_f: f32,
    #[serde(rename = "indoorhumidity")]
    pub(crate) indoor_humidity_in_percent: f32,

    #[serde(rename = "soiltempf")]
    pub(crate) channel_1_temperature_in_f: Option<f32>,
    #[serde(rename = "soilmoisture")]
    pub(crate) channel_1_humidity_in_percent: Option<f32>,

    #[serde(rename = "soiltemp2f")]
    pub(crate) channel_2_temperature_in_f: Option<f32>,
    #[serde(rename = "soilmoisture2")]
    pub(crate) channel_2_humidity_in_percent: Option<f32>,

    #[serde(rename = "soiltemp3f")]
    pub(crate) channel_3_temperature_in_f: Option<f32>,
    #[serde(rename = "soilmoisture3")]
    pub(crate) channel_3_humidity_in_percent: Option<f32>,

    #[serde(rename = "soiltemp4f")]
    pub(crate) channel_4_temperature_in_f: Option<f32>,
    #[serde(rename = "soilmoisture4")]
    pub(crate) channel_4_humidity_in_percent: Option<f32>,

    #[serde(rename = "soiltemp5f")]
    pub(crate) channel_5_temperature_in_f: Option<f32>,
    #[serde(rename = "soilmoisture5")]
    pub(crate) channel_5_humidity_in_percent: Option<f32>,

    #[serde(rename = "soiltemp6f")]
    pub(crate) channel_6_temperature_in_f: Option<f32>,
    #[serde(rename = "soilmoisture6")]
    pub(crate) channel_6_humidity_in_percent: Option<f32>,

    #[serde(rename = "soiltemp7f")]
    pub(crate) channel_7_temperature_in_f: Option<f32>,
    #[serde(rename = "soilmoisture7")]
    pub(crate) channel_7_humidity_in_percent: Option<f32>,

    #[serde(rename = "soiltemp8f")]
    pub(crate) channel_8_temperature_in_f: Option<f32>,
    #[serde(rename = "soilmoisture8")]
    pub(crate) channel_8_humidity_in_percent: Option<f32>,
}

#[derive(Default)]
pub struct State {
    pub(crate) db: Option<GarniRecord>,
}


pub type AppState = Arc<RwLock<State>>;

pub fn create_app_state() -> AppState {
    Arc::new(RwLock::new(State::default()))
}