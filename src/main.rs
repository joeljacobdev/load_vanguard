use std::{collections::HashMap, fs::File};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
enum RequestMethod {
    GET,
    PUT,
    PATCH,
    POST,
}

#[derive(Debug, Deserialize, Serialize)]
struct Endpoint {
    path: String,
    method: RequestMethod,
    params: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Senario {
    title: String,
    frequency: i32,
    apis: Vec<Endpoint>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    base_url: String,
    headers: HashMap<String, String>,
    senarios: Vec<Senario>,
}

impl Endpoint {
    fn prepare_endpoint(self: &Endpoint, base_url: &String) -> String {
        let mut url = base_url.to_string();
        url.push_str(self.path.as_str());
        if url.ends_with("/") == false {
            url.push('/');
        }
        if self.params.is_empty() == false {
            url.push('?');
        }
        for (param, value) in self.params.iter() {
            if url.ends_with("&") == false && url.ends_with('?') == false {
                url.push('&');
            }
            url.push_str(format!("{param}={value}").as_str())
        }
        return url;
    }

    async fn call_api(
        self: &Endpoint,
        config: &Config,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let url = self.prepare_endpoint(&config.base_url);
        let response = reqwest::get(&url).await?;
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|value| value.to_str().ok())
            .and_then(|str_value| str_value.split(";").next())
            .map(|str_value| str_value.trim());

        if let Some(ct) = content_type {
            assert_eq!(ct, "application/json");
        } else {
            eprintln!("Content type not found");
        }

        if response.status().is_success() == true {
            let json: serde_json::Value = response.json().await?;
            return Ok(json);
        } else {
            eprintln!("Request has failed with status={}", response.status());
        }
        Ok(serde_json::Value::Null)
    }
}

#[tokio::main]
async fn main() {
    let file_name = std::env::args()
        .nth(1)
        .unwrap_or("loadtest.yml".to_string());

    let file = match File::open(&file_name) {
        Ok(file) => file,
        Err(e) => {
            println!("Failed to open file {} {:?}", file_name, e.to_string());
            return;
        }
    };

    let config: Config = match serde_yaml::from_reader(file) {
        Ok(config) => config,
        Err(_) => {
            eprintln!("Failed to open file {} ", file_name);
            return;
        }
    };

    println!("Read config = {:?}\n", config);
    for senario in config.senarios.iter() {
        let mut json: serde_json::Value = serde_json::Value::Null;
        for api in senario.apis.iter() {
            json = match api.call_api(&config).await {
                Ok(response) => response,
                Err(_) => {
                    eprintln!("Error occured in api call");
                    serde_json::Value::Null
                }
            };
            println!("{}", json)
        }
    }
}
