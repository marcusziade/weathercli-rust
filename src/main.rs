use weather::{get_weather, Config, display_weather};

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
            display_weather(&weather_data, &mut std::io::stdout());
        }
        Err(err) => {
            println!("Error: {}", err);
            std::process::exit(1);
        }
    }
}