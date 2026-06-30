
#[derive(Eq, PartialEq)]
pub(crate) enum Conversion {
    None,
    FToC,
    InchToMM,
    MPHToMS,
}

pub(crate) enum Channel {
    Indoor,
    Outdoor,
    Channel1,
    Channel2,
    Channel3,
    Channel4,
    Channel5,
    Channel6,
    Channel7,
    Channel8,
}

pub(crate) enum Field {
    Baromin,
    TempF(Channel),
    DewPtF,
    Humidity(Channel),
    WindSpeedMph,
    WindGustMph,
    WindDir,
    RainInInches,
    DailyRainInInches,
    SolarRadiation,
    UVIndex,
}

pub(crate) enum MetricsType {
    Unknown,
    Gauge,
    Counter,
    StateSet,
    Info,
    Histogram,
    GaugeHistogram,
    Summary,
}

impl MetricsType {
    pub(crate) fn tpe_string(&self) -> &'static str {
        match self {
            MetricsType::Unknown => "unknown",
            MetricsType::Gauge => "gauge",
            MetricsType::Counter => "counter",
            MetricsType::StateSet => "stateset",
            MetricsType::Info => "info",
            MetricsType::Histogram => "histogram",
            MetricsType::GaugeHistogram => "gaugehistogram",
            MetricsType::Summary => "summary",
        }
    }
}


#[derive(Copy, Clone)]
pub(crate) struct PrometheusParam<'a> {
    pub(crate) name: &'a str,
    pub(crate) from_field: &'a Field,
    pub(crate) help: &'a str,
    pub(crate) p_type: &'a MetricsType,
    pub(crate) convert: &'a Conversion,
}

pub(crate) const ALL_PARAM: &'static [PrometheusParam] = &[
    PrometheusParam {
        name: "garni_weather_baromin",
        from_field: &Field::Baromin,
        help: "Barometric presure",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::None,
    },
    PrometheusParam {
        name: "garni_weather_temp_outside_in_F",
        from_field: &Field::TempF(Channel::Outdoor),
        help: "Temperature in F outside",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::None,
    },
    PrometheusParam {
        name: "garni_weather_temp_outside_in_C",
        from_field: &Field::TempF(Channel::Outdoor),
        help: "Temperature in C outside",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::FToC,
    },
    PrometheusParam {
        name: "garni_weather_humidity_outside_in_percent",
        from_field: &Field::Humidity(Channel::Outdoor),
        help: "Relative humidity outside",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::None,
    },
    PrometheusParam {
        name: "garni_weather_dew_point_in_F",
        from_field: &Field::DewPtF,
        help: "Dew point in F",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::None,
    },
    PrometheusParam {
        name: "garni_weather_dew_point_in_C",
        from_field: &Field::DewPtF,
        help: "Dew point in C",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::FToC,
    },
    PrometheusParam {
        name: "garni_weather_wind_speed_in_mph",
        from_field: &Field::WindSpeedMph,
        help: "Wind speed in mph",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::None,
    },
    PrometheusParam {
        name: "garni_weather_wind_speed_in_ms",
        from_field: &Field::WindSpeedMph,
        help: "Wind speed in m/s",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::MPHToMS,
    },
    PrometheusParam {
        name: "garni_weather_wind_gust_in_mph",
        from_field: &Field::WindGustMph,
        help: "Wind gust in mph",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::None,
    },
    PrometheusParam {
        name: "garni_weather_wind_gust_in_ms",
        from_field: &Field::WindGustMph,
        help: "Wind gust in m/s",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::MPHToMS,
    },
    PrometheusParam {
        name: "garni_weather_wind_direction_in_degrees",
        from_field: &Field::WindDir,
        help: "Wind direction in degrees",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::None,
    },
    PrometheusParam {
        name: "garni_weather_rain_in_inches",
        from_field: &Field::RainInInches,
        help: "Rain in inches",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::None,
    },
    PrometheusParam {
        name: "garni_weather_rain_in_mm",
        from_field: &Field::RainInInches,
        help: "Rain in mm",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::InchToMM,
    },
    PrometheusParam {
        name: "garni_weather_daily_rain_in_inches",
        from_field: &Field::DailyRainInInches,
        help: "Daily rain in inches",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::None,
    },
    PrometheusParam {
        name: "garni_weather_daily_rain_in_mm",
        from_field: &Field::DailyRainInInches,
        help: "Daily rain in mm",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::InchToMM,
    },
    PrometheusParam {
        name: "garni_weather_solar_radiation_in_w_m2",
        from_field: &Field::SolarRadiation,
        help: "Solar radiation in w/m2",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::None,
    },
    PrometheusParam {
        name: "garni_weather_uv_index",
        from_field: &Field::UVIndex,
        help: "UV index",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::None,
    },
    // Indoor temperature
    PrometheusParam {
        name: "garni_weather_indoor_temperature_in_F",
        from_field: &Field::TempF(Channel::Indoor),
        help: "Temperature indoor in F",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::None,
    },
    PrometheusParam {
        name: "garni_weather_indoor_temperature_in_C",
        from_field: &Field::TempF(Channel::Indoor),
        help: "Temperature indoor in C",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::FToC,
    },
    PrometheusParam {
        name: "garni_weather_indoor_humidity_in_percent",
        from_field: &Field::Humidity(Channel::Indoor),
        help: "Relative humidity indoor",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::None,
    },
    // Channel 1
    PrometheusParam {
        name: "garni_weather_temp_channel_1_in_F",
        from_field: &Field::TempF(Channel::Channel1),
        help: "Temperature in F Channel 1",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::None,
    },
    PrometheusParam {
        name: "garni_weather_temp_channel_1_in_C",
        from_field: &Field::TempF(Channel::Channel1),
        help: "Temperature in C Channel 1",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::FToC,
    },
    PrometheusParam {
        name: "garni_weather_humidity_channel_1_in_percent",
        from_field: &Field::Humidity(Channel::Channel1),
        help: "Relative humidity Channel 1",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::None,
    },
    // Channel 2
    PrometheusParam {
        name: "garni_weather_temp_channel_2_in_F",
        from_field: &Field::TempF(Channel::Channel2),
        help: "Temperature in F Channel 2",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::None,
    },
    PrometheusParam {
        name: "garni_weather_temp_channel_2_in_C",
        from_field: &Field::TempF(Channel::Channel2),
        help: "Temperature in C Channel 2",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::FToC,
    },
    PrometheusParam {
        name: "garni_weather_humidity_channel_2_in_percent",
        from_field: &Field::Humidity(Channel::Channel2),
        help: "Relative humidity Channel 2",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::None,
    },
    // Channel 3
    PrometheusParam {
        name: "garni_weather_temp_channel_3_in_F",
        from_field: &Field::TempF(Channel::Channel3),
        help: "Temperature in F Channel 3",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::None,
    },
    PrometheusParam {
        name: "garni_weather_temp_channel_3_in_C",
        from_field: &Field::TempF(Channel::Channel3),
        help: "Temperature in C Channel 3",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::FToC,
    },
    PrometheusParam {
        name: "garni_weather_humidity_channel_3_in_percent",
        from_field: &Field::Humidity(Channel::Channel3),
        help: "Relative humidity Channel 3",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::None,
    },
    // Channel 4
    PrometheusParam {
        name: "garni_weather_temp_channel_4_in_F",
        from_field: &Field::TempF(Channel::Channel4),
        help: "Temperature in F Channel 4",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::None,
    },
    PrometheusParam {
        name: "garni_weather_temp_channel_4_in_C",
        from_field: &Field::TempF(Channel::Channel4),
        help: "Temperature in C Channel 4",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::FToC,
    },
    PrometheusParam {
        name: "garni_weather_humidity_channel_4_in_percent",
        from_field: &Field::Humidity(Channel::Channel4),
        help: "Relative humidity Channel 4",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::None,
    },
    // Channel 5
    PrometheusParam {
        name: "garni_weather_temp_channel_5_in_F",
        from_field: &Field::TempF(Channel::Channel5),
        help: "Temperature in F Channel 5",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::None,
    },
    PrometheusParam {
        name: "garni_weather_temp_channel_5_in_C",
        from_field: &Field::TempF(Channel::Channel5),
        help: "Temperature in C Channel 5",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::FToC,
    },
    PrometheusParam {
        name: "garni_weather_humidity_channel_5_in_percent",
        from_field: &Field::Humidity(Channel::Channel5),
        help: "Relative humidity Channel 5",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::None,
    },
    // Channel 6
    PrometheusParam {
        name: "garni_weather_temp_channel_6_in_F",
        from_field: &Field::TempF(Channel::Channel6),
        help: "Temperature in F Channel 6",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::None,
    },
    PrometheusParam {
        name: "garni_weather_temp_channel_6_in_C",
        from_field: &Field::TempF(Channel::Channel6),
        help: "Temperature in C Channel 6",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::FToC,
    },
    PrometheusParam {
        name: "garni_weather_humidity_channel_6_in_percent",
        from_field: &Field::Humidity(Channel::Channel6),
        help: "Relative humidity Channel 6",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::None,
    },
    // Channel 7
    PrometheusParam {
        name: "garni_weather_temp_channel_7_in_F",
        from_field: &Field::TempF(Channel::Channel7),
        help: "Temperature in F Channel 7",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::None,
    },
    PrometheusParam {
        name: "garni_weather_temp_channel_7_in_C",
        from_field: &Field::TempF(Channel::Channel7),
        help: "Temperature in C Channel 7",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::FToC,
    },
    PrometheusParam {
        name: "garni_weather_humidity_channel_7_in_percent",
        from_field: &Field::Humidity(Channel::Channel7),
        help: "Relative humidity Channel 7",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::None,
    },
    // Channel 8
    PrometheusParam {
        name: "garni_weather_temp_channel_*_in_F",
        from_field: &Field::TempF(Channel::Channel8),
        help: "Temperature in F Channel 8",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::None,
    },
    PrometheusParam {
        name: "garni_weather_temp_channel_8_in_C",
        from_field: &Field::TempF(Channel::Channel8),
        help: "Temperature in C Channel 8",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::FToC,
    },
    PrometheusParam {
        name: "garni_weather_humidity_channel_8_in_percent",
        from_field: &Field::Humidity(Channel::Channel8),
        help: "Relative humidity Channel 8",
        p_type: &MetricsType::Gauge,
        convert: &Conversion::None,
    },
];