use dotenv::dotenv;
use std::env;
use std::io;
use serde::Deserialize;
use colored::*;

// Struct to deserialize the json response from the openWeatherMap API

#[derive(Deserialize, Debug)]
struct WeatherResponse{
    weather : Vec<Weather>,
    main : Main,
    wind : Wind,
    name : String
}

// Struct to represent the weather description
#[derive(Deserialize, Debug)]
struct Weather{
    description : String
}

// Struct to repressent the main weather parameters
#[derive(Deserialize, Debug)]
struct Main{
    temp : f64,
    humidity : f64,
    pressure : f64
}

// Struct to represent wind info
#[derive(Deserialize, Debug)]
struct Wind{
    speed : f64
}

// Function to get the weather info from OpenWeatherMap API
fn get_weather_info(city: &str, country_code: &str, api_key: &str) -> Result<WeatherResponse, reqwest::Error> {
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}&units=metric",
        city,
        country_code,
        api_key
    );

    let response = reqwest::blocking::get(&url)?;
    let response_json: WeatherResponse = response.json()?;
    Ok(response_json)
}




// Function to display the weather info
fn display_weather_info(response: &WeatherResponse){
    // Extract the weather info from the response
    let description = &response.weather[0].description;
    let temperature = response.main.temp;
    let pressure = response.main.pressure;
    let humidity = response.main.humidity;
    let wind_speed = response.wind.speed;

    let weather_text = format!(
        "Weather in {} : {} {}
        > Temperator : {:.1} C,
        > Pressure : {:.1} hPa,
        > Humidity : {:.1}%
        > Wind Speed : {:.1} m/s",
        response.name,
        description,
        get_temp_emoji(temperature),
        temperature,
        pressure,
        humidity,
        wind_speed
    );

    // Coloring the weather text based on weather conditions
    let weather_text_colored : ColoredString = match description.as_str(){
        "clear sky" => weather_text.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" => weather_text.dimmed(),
        "shower rain" | "rain" | "thunderstorm" | "snow" => weather_text.bright_cyan(),
        _ => weather_text.normal()
    };

    // Print the colored weather info
    println!("{}", weather_text_colored);

    // Function to get emoji based on temp
    fn get_temp_emoji(temp : f64)-> &'static str{
        if temp < 0.0 {
            "❄"
        }
        else if temp >= 0.0 && temp < 10.0 {
            "☁"
        }
        else if temp >= 10.0 && temp < 20.0{
            "🌥️"
        }
        else if temp >=20.0 && temp < 30.0{
            "🌤️"
        }
        else{
            "🌞"
        }
    }

}

fn main(){
    // Loading env variables
    dotenv().ok();

    println!("{}","Welcome to Weather Station!".bright_yellow());

    loop{
        // Reading city
        println!("{}","Please enter the name of the city :".bright_green());

        let mut city = String::new();

        io::stdin().read_line(&mut city).expect("Failed to read input!");
        let city = city.trim();

        // Reading country 
        println!("{}","Please enter the country code (e.g. : US for United States) :".bright_green());

        let mut country_code = String::new();

        io::stdin().read_line(&mut country_code).expect("Failed to read input!");
        let country_code = country_code.trim();

        let api_key = env::var("API_KEY").expect("API_KEY not found in .env");
        println!("API Key: {}", api_key);

        // Calling the function to fetch weather info
        match get_weather_info(&city, &country_code, &api_key){
            Ok(response)=>{
                display_weather_info(&response);
            }
            Err(err)=>{
                eprintln!("Error occured: {err}");
            }
        }   
        
        // Prompting user to continue or exit
        println!("{}", "Do you want to search for weather in another city? (yes/no) :".bright_green());
        let mut input = String::new();
        // Reading input for continuation
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim().to_lowercase();

        if input!= "yes"{
            println!("Thank you for using our software!");
            break;  //Exiting the loop if the user doesn't want to continue
        }
    }


}