# Rust Weather Application

This application fetches and displays current weather information for a specified city using the OpenWeatherMap API. It uses the `reqwest` library for making HTTP requests, `serde` for deserializing JSON responses, and `tokio` for asynchronous execution.

## Features

- Fetches current temperature, "feels like" temperature, weather description, and wind speed for a given city.
- Displays the weather information in a formatted table.
- Handles HTTP request errors and provides feedback if the request is unsuccessful.

## Requirements

- **Rust**: Install Rust from [here](https://www.rust-lang.org/tools/install).
- **OpenWeatherMap API key**: You'll need a free API key from OpenWeatherMap. [Sign up here](https://openweathermap.org/api) to get your API key.

## Dependencies

- `reqwest`: For making HTTP requests to the OpenWeatherMap API.
- `serde`: For serializing and deserializing JSON data.
- `tokio`: For asynchronous execution.

You can add these dependencies to your `Cargo.toml`:

```toml
[dependencies]
reqwest = { version = "0.12", features = ["json"] }
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
```
## Usage
-  Set up your environment:
Install Rust by following the instructions here.
Obtain your OpenWeatherMap API key by signing up at OpenWeatherMap.
-  Clone the repository (if you haven't already):
sh
Copy code
git clone https://github.com/yourusername/rust-weather-app.git
``` Shell
cd rust-weather-app
```
-   Update the API key:
Replace the api_key variable in the code with your OpenWeatherMap API key.
```
let api_key = "YOUR_API_KEY"; 
```
-   Run the application:
``` Shell
cargo build
cargo run
```
-   Enter a city name when prompted. The app will fetch and display the current weather data for that city.

When you run the application and enter a city name (e.g., "London"), you should see output similar to the following:

### Response:

```
Enter the city name:
London
---------------------------------------------------------------------------------------------------------------------------------
| City: Budapest | Temprature: 294.4 | feels like: 293.81 | Wind Speed: 5.14 | Humidity: 47 | Weather Description: clear sky
---------------------------------------------------------------------------------------------------------------------------------
```
This will display the current temperature, "feels like" temperature, weather description, and wind speed for the entered city.