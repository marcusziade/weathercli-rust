use weather::{get_weather, Config, display_weather, WeatherData, Weather, Main};

#[test]
fn test_get_weather() {
    let city = "Helsinki";
    let config_str = std::fs::read_to_string("config.toml").expect("Failed to read config file");
    let config: Config = toml::from_str(&config_str).expect("Failed to parse config file");

    let result = get_weather(city, &config.api_key);

    assert!(result.is_ok());
    let weather_data = result.unwrap();
    assert_eq!(weather_data.name, city);
    assert!(weather_data.main.temp > -100.0 && weather_data.main.temp < 100.0);
}

#[test]
fn test_display_weather() {
    let weather_data = WeatherData {
        name: "San Francisco".to_string(),
        main: Main {
            temp: 15.5,
            feels_like: 12.3,
            temp_min: 13.2,
            temp_max: 17.8,
        },
        weather: vec![Weather {
            description: "overcast clouds".to_string(),
            main: "Clouds".to_string(),
        }],
    };

    let expected_output = format!(
        "\nWeather for {}: \nTemperature: 15.5°C\nFeels like: 12.3°C\nMin temperature: 13.2°C\nMax temperature: 17.8°C\nDescription: overcast clouds\n",
        weather_data.name
    );

    let mut output = Vec::new();
    display_weather(&weather_data, &mut output);
    assert_eq!(String::from_utf8(output).unwrap(), expected_output);
}

