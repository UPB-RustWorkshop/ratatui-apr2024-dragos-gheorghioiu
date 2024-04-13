use std::error;

use crate::connection::{self, CityInfo};

/// Application result type.
pub type AppResult<T> = Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub cities: Vec<String>,
    pub current_city_weather: CityInfo,
    pub index_selected: usize,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self {
            running: true,
            cities: Vec::new(),
            index_selected: 0,
            current_city_weather: CityInfo {
                name: "".to_string(),
                weather: ("".to_string(), "".to_string()),
            },
        }
    }

    pub async fn add_cities(&mut self) {
        self.cities = connection::get_cities().await;
    }

    pub async fn add_weather(&mut self, city: String) {
        self.current_city_weather = connection::get_temperature(city).await;
    }
}
