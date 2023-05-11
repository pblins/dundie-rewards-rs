use std::collections::HashMap;

use reqwest::Error as ReqwestError;
use serde::{Deserialize, Serialize};
use serde_json::{Error as SerdeError, Value};

const API_BASE_URL: &str = "https://economia.awesomeapi.com.br/json/last/USD-";

#[derive(Debug)]
pub enum ExchangeError {
    Reqwest(ReqwestError),
    Serde(SerdeError),
    NotFound,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct USDRate {
    #[serde(default = "default_code")]
    pub code: String,
    #[serde(default = "default_code")]
    pub codein: String,
    #[serde(default = "default_name")]
    pub name: String,
    #[serde(alias = "high")]
    pub value: String,
}

fn default_code() -> String {
    "USD".to_string()
}

fn default_name() -> String {
    "Dolar/Dolar".to_string()
}

pub fn get_rates(currencies: Vec<String>) -> Result<HashMap<String, USDRate>, ExchangeError> {
    let mut result: HashMap<String, USDRate> = HashMap::new();
    let mut rate;

    for currency in currencies {
        if currency == default_code() {
            match serde_json::from_str::<USDRate>("{\"high\": \"1\" }") {
                Ok(converted) => rate = converted,
                Err(error) => return Err(ExchangeError::Serde(error)),
            };
        } else {
            let response = fetch_api(&currency)?;
            rate = convert_response(response, &currency)?;
        }
        result.insert(currency, rate);
    }

    Ok(result)
}

fn fetch_api(currency: &String) -> Result<String, ExchangeError> {
    match reqwest::blocking::get(format!("{}{}", API_BASE_URL, currency)) {
        Ok(response_object) => match response_object.text() {
            Ok(response) => Ok(response),
            Err(error) => Err(ExchangeError::Reqwest(error)),
        },
        Err(error) => Err(ExchangeError::Reqwest(error)),
    }
}

fn convert_response(response: String, currency: &String) -> Result<USDRate, ExchangeError> {
    match serde_json::from_str::<Value>(&response) {
        Ok(mut converted_value) => match converted_value.get_mut(format!("USD{}", currency)) {
            Some(value_response) => {
                match serde_json::from_str::<USDRate>(&value_response.take().to_string()) {
                    Ok(result) => Ok(result),
                    Err(error) => Err(ExchangeError::Serde(error)),
                }
            }
            None => Err(ExchangeError::NotFound),
        },
        Err(error) => Err(ExchangeError::Serde(error)),
    }
}
