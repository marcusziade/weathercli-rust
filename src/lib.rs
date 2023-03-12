use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WeatherData {
    pub name: String,
    pub main: Main,
    pub weather: Vec<Weather>,
}

#[derive(Debug, Deserialize)]
pub struct Main {
    pub temp: f32,
    pub feels_like: f32,
    pub temp_min: f32,
    pub temp_max: f32,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Weather {
    pub description: String,
    pub main: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub api_key: String,
}

pub fn get_weather(city: &str, api_key: &str) -> Result<WeatherData, Box<dyn std::error::Error>> {
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );
    let resp = reqwest::blocking::get(&url)?.json::<WeatherData>()?;
    Ok(resp)
}

pub fn display_weather(weather_data: &WeatherData, mut output: impl std::io::Write) {
    writeln!(output, "\nWeather for {}: ", weather_data.name).unwrap();
    writeln!(output, "Temperature: {:.1}°C", weather_data.main.temp).unwrap();
    writeln!(output, "Feels like: {:.1}°C", weather_data.main.feels_like).unwrap();
    writeln!(output, "Min temperature: {:.1}°C", weather_data.main.temp_min).unwrap();
    writeln!(output, "Max temperature: {:.1}°C", weather_data.main.temp_max).unwrap();
    writeln!(output, "Description: {}", weather_data.weather[0].description).unwrap();
}
