# Garni-rs

Small server for receiving weather data from [Garni meteo stations](https://www.garnitechnology.com/).
Currently only supports the [Garni 1025 Arcus](https://www.garnitechnology.com/garni-1025-arcus/) weather station.

Metrics are stored in memory and served via /weather/prometheus endpoint. Currently, only the v0.0.4 format is supported.

## Plan

- prometheus v1.0.0 metrics support
- OpenMetricsText v1.0.0 support
- configurable listen port


## Example of local Garni update push


Garni meteo station is configured to push data to "/weatherstation/updateweatherstation.php"
using the GET method with data passed using query parameters.

Example of the data sent:
```text
GET /weatherstation/updateweatherstation.php?ID=XX&PASSWORD=&action=updateraww&realtime=1&rtfreq=5&dateutc=now&baromin=30.04&tempf=45.1&dewptf=43.5&humidity=94&windspeedmph=0.0&windgustmph=0.0&winddir=192&rainin=0.0&dailyrainin=0.0&solarradiation=37.05&UV=0.0&indoortempf=72.5&indoorhumidity=40&soiltempf=70.8&soilmoisture=40&soiltemp2f=63.5&soilmoisture2=58
```