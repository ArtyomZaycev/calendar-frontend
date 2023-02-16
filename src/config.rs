use std::{fs, path::Path};

use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Config {
    pub api_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_url: "http://127.0.0.1:8081/".to_owned(),
        }
    }
}

impl Config {
    pub fn load(path: &Path) -> Self {
        match fs::read_to_string(path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_else(|err| {
                println!("Error deserializing config: {err:?}, using fallback");
                Default::default()
            }),
            Err(err) => {
                println!("Error reading config: {err:?}, using fallback");
                Default::default()
            }
        }
    }
}
