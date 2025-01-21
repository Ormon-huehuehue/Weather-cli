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
    temp : f63,
    humidity : f64,
    pressure : f64
}

// Struct to represent wind info
#[derive(Deserialize, Debug)]
struct Wind{
    speed : f64
}