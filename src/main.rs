use serde::Deserialize;
use toml;

#[derive(Debug, serde::Deserialize)]
struct WeatherData {
    name: String,
    main: Main,
    weather: Vec<Weather>,
}

#[derive(Debug, serde::Deserialize)]
struct Main {
    temp: f32,
    feels_like: f32,
    temp_min: f32,
    temp_max: f32,
}

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
struct Weather {
    description: String,
    main: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    pub api_key: String,
}

fn main() {
    let config_str = std::fs::read_to_string("config.toml").expect("Failed to read config file");
    let config: Config = toml::from_str(&config_str).expect("Failed to parse config file");

    let mut city = String::new();
    println!("Enter a city name:");
    std::io::stdin()
        .read_line(&mut city)
        .expect("Failed to read line");

    let city = city.trim();

    match get_weather(city, &config.api_key) {
        Ok(weather_data) => {
            display_weather(&weather_data);
        }
        Err(err) => {
            println!("Error: {}", err);
            std::process::exit(1);
        }
    }
}

fn display_weather(weather_data: &WeatherData) {
    println!("\nWeather for {}: ", weather_data.name);
    println!("Temperature: {:.1}°C", weather_data.main.temp);
    println!("Feels like: {:.1}°C", weather_data.main.feels_like);
    println!("Min temperature: {:.1}°C", weather_data.main.temp_min);
    println!("Max temperature: {:.1}°C", weather_data.main.temp_max);
    println!("Description: {}", weather_data.weather[0].description);
}

fn get_weather(city: &str, api_key: &str) -> Result<WeatherData, Box<dyn std::error::Error>> {
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );
    let resp = reqwest::blocking::get(&url)?.json::<WeatherData>()?;
    Ok(resp)
}
