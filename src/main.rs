use reqwest;
use serde::Deserialize;
use tokio;

// Struct to deserialize the JSON response
#[derive(Deserialize)]
struct WeatherResponse {
    main: Main,
    weather: Vec<Weather>,
    wind: Wind,
}

#[derive(Deserialize)]
struct Main {
    temp: f64,
    #[serde(rename = "feels_like")]
    feels_like: f64,
}

#[derive(Deserialize)]
struct Weather {
    description: String,
}

#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}

async fn fetch_weather(city: &str, api_key: &str) -> Result<(), reqwest::Error> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city.trim(),
        api_key
    );

    let response = reqwest::get(&url).await?;

    if response.status().is_success() {
        // Parse the JSON response into our WeatherData struct
        let weather_data: WeatherResponse = response.json().await?;
        let res_print = format!(
            "| Temperature: {:.2}°C | Feels Like: {:.2}°C | Weather Description: {} |Wind Speed : {} |",
            weather_data.main.temp,
            weather_data.main.feels_like,
            weather_data.weather[0].description,
            weather_data.wind.speed,
        );

        println!("---------------------------------------------------------------------------------------------------------");
        println!("{}", res_print);
        println!("---------------------------------------------------------------------------------------------------------");
    } else {
        // Print an error message if the request was not successful
        println!("Error: {}", response.status());
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let api_key = "e1c1ab8ec18e066945e570f900fe53a2"; // Replace with your OpenWeatherMap API key
    let mut city = String::new();

    println!("Enter the city name:");
    std::io::stdin().read_line(&mut city).expect("Failed to read line");

    if let Err(e) = fetch_weather(&city, api_key).await {
        eprintln!("Error fetching weather data: {}", e);
    }
}

