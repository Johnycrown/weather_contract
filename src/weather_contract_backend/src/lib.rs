use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};
use ic_cdk_macros::*;
use reqwest;

#[derive(Debug, Deserialize, CandidType)]
struct ContractState {
    api_key: String,
    city: String,
    country_code: String,
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[derive(CandidType, Deserialize)]
struct Weather {
    temperature: u32,
    pressure: u32,
    humidity: u32,
    wind_speed: u32,
}

#[derive(Default)]
struct WeatherContract {
    weather_data: Option<Weather>,
}

fn init() {
    // Initialize the WeatherContract state here
    let _ = WeatherContract::default(); // We ignore the return value as there's nothing to return
}
#[update]
async fn fetch_weather_data(city: String, country_code: String, api_key: String) {
    let api_endpoint = "http://api.openweathermap.org/data/2.5/weather?q=";
    let full_url = format!("{}{},{}&APPID={}", api_endpoint, city, country_code, api_key);

    match reqwest::get(&full_url).await {
        Ok(response) => {
            if response.status().is_success() {
                if let Ok(weather_json) = response.json::<serde_json::Value>().await {
                    let temperature = weather_json["main"]["temp"].as_f64().unwrap_or(0.0);
                    let pressure = weather_json["main"]["pressure"].as_f64().unwrap_or(0.0);
                    let humidity = weather_json["main"]["humidity"].as_f64().unwrap_or(0.0);
                    let wind_speed = weather_json["wind"]["speed"].as_f64().unwrap_or(0.0);

                    let weather_data = Weather {
                        temperature: temperature as u32,
                        pressure: pressure as u32,
                        humidity: humidity as u32,
                        wind_speed: wind_speed as u32,
                    };

                    // Save weather_data to storage
                    ic_cdk::print("Saving weather data to storage");
                    ic_cdk::print(&format!("{:?}", &weather_data));
                    WEATHER_MAP.with(|map| {
                        map.borrow_mut()
                            .insert(0, weather_data)
                            .expect("Failed to insert weather data into map");
                    });
                }
            }
        }
        Err(e) => ic_cdk::print(e.to_string()),
    }
}

#[query]
fn get_weather_data(city: String, country_code: String) -> Option<Weather> {
    WEATHER_MAP.with(|map| map.borrow().get(&(0, city.clone(), country_code.clone())).cloned())
}

impl Storable for Weather {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

const MAXIMUM_VALUE_SIZE: u32 = 50;

impl BoundedStorable for Weather {
    const MAX_SIZE: u32 = MAXIMUM_VALUE_SIZE;
    const IS_FIXED_SIZE: bool = false;
}
impl BoundedStorable for (u64, String, String) {
    const MAX_SIZE: u32 = MAXIMUM_VALUE_SIZE;
    const IS_FIXED_SIZE: bool = false;
}


type Memory = VirtualMemory<DefaultMemoryImpl>;







thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static WEATHER_MAP: RefCell<StableBTreeMap<(u64, String, String), Weather, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        ));
}








