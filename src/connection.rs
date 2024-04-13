extern crate serde;
extern crate serde_json;

// use chrono::{DateTime, Local};
use reqwest;
use serde::Deserialize;

#[derive(Debug)]
pub struct CityInfo {
    // TODO: define elements in the structure
    pub name: String,
    pub weather: (String, String),
}

#[derive(Debug, Deserialize)]
pub struct Cities {
    cities: Vec<String>,
}

/// Method that is handling the request to the openweather api,
/// parsing the response
///
/// IP: 34.116.205.113
/// Port: 3000
///
/// Returns weather details about a certain city
pub async fn get_temperature(city: String) -> CityInfo {
    let client = reqwest::Client::new();
    let res = client
        .post("http://34.116.205.113:3000/cities/current_weather")
        .json(&serde_json::json!({ "city": city }))
        .send()
        .await;
    let body = res.unwrap().text().await.unwrap();
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    let weather = json["weather"]["description"].as_str().unwrap();
    let temperature = json["conditions"]["temp"].as_f64().unwrap();
    let city_info = CityInfo {
        name: city,
        weather: (weather.to_string(), temperature.to_string()),
    };
    city_info
}

pub async fn get_cities() -> Vec<String> {
    match reqwest::get("http://34.116.205.113:3000/cities").await {
        Ok(response) => {
            // Parse response
            // cities field is a list of cities from the response
            let json_str = response.text().await;
            let cities: Cities = serde_json::from_str(&json_str.unwrap()).unwrap();
            cities.cities
        }
        Err(error) => {
            // Handle error
            eprintln!("Error: {:?}", error);
            vec![]
        }
    }
}
