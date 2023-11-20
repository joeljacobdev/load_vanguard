use dotenv::dotenv;
use log;
use serde::{Deserialize, Serialize};
use std::env;
use std::{collections::HashMap, fs::File, sync::Arc};
use tokio::{spawn, task::JoinHandle, time::Instant};

#[derive(Debug, Deserialize, Serialize, Clone)]
enum RequestMethod {
    GET,
    PUT,
    PATCH,
    POST,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Endpoint {
    path: String,
    identifier: String,
    method: RequestMethod,
    params: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Senario {
    title: String,
    frequency: i32,
    apis: Vec<Endpoint>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    base_url: String,
    headers: HashMap<String, String>,
    senarios: Vec<Senario>,
}

impl Endpoint {
    fn __prepare_endpoint(self: &Endpoint, base_url: &String) -> String {
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
        scene: &String,
        config: &Config,
        _previous_response: &serde_json::Value,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let url = self.__prepare_endpoint(&config.base_url);
        let start_time = Instant::now();
        let response = reqwest::get(&url).await?;
        let duration = start_time.elapsed();
        log::info!(
            "[{scene}] [{}] executed in duration = {duration:.1?}",
            self.identifier
        );
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

impl Senario {
    async fn execute(&self, scene: String, config: Arc<Config>) {
        let mut json: serde_json::Value = serde_json::Value::Null;
        for api in self.apis.iter() {
            json = match api.call_api(&scene, &config, &json).await {
                Ok(response) => response,
                Err(_) => {
                    eprintln!("Error occured in api call");
                    serde_json::Value::Null
                }
            };
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let rust_log = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    env::set_var("RUST_LOG", rust_log);

    env_logger::init();

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

    let config: Arc<Config> = match serde_yaml::from_reader(file) {
        Ok(config) => Arc::<Config>::new(config),
        Err(_) => {
            eprintln!("Failed to open file {} ", file_name);
            return;
        }
    };

    log::info!("Read config = {:?}\n", config);
    let senarios: Vec<Arc<Senario>> = config
        .senarios
        .iter()
        .map(|s| Arc::new(s.clone()))
        .collect();
    for senario in senarios.iter() {
        let mut tasks = Vec::<JoinHandle<()>>::new();
        for scene_no in 0..senario.frequency {
            let scene = format!("{} {scene_no}", senario.title);
            let cloned_config = Arc::clone(&config);
            let cloned_senario = Arc::clone(&senario);
            tasks.push(spawn(async move {
                cloned_senario.execute(scene, cloned_config).await;
            }));
        }
        for task in tasks {
            task.await.unwrap()
        }
    }
}
