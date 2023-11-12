use std::{collections::HashMap, fs::File};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Endpoint {
    path: String,
    params: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Senario {
    senario: String,
    frequency: i32,
    apis: Vec<Endpoint>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    base_url: String,
    headers: HashMap<String, String>,
    senarios: Vec<Senario>,
}

fn main() {
    let file_name = std::env::args()
        .nth(1)
        .unwrap_or("loadtest.yml".to_string());

    let file = match File::open(&file_name) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Failed to open file {} ", file_name);
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

    println!("{:?}", config);
}
