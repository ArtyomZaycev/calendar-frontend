use serde::Deserialize;
use url::Url;

#[derive(Clone, Deserialize)]
pub struct Config {
    pub is_localhost: bool,
    pub api_url: String,
}

impl Config {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn load() -> Self {
        Self {
            is_localhost: false,
            api_url: std::env::var("API_URL").expect("Error loading API_URL for env"),
        }
    }
    
    #[cfg(target_arch = "wasm32")]
    pub fn load() -> Self {
        let location = web_sys::window().unwrap().location();
        let mut hostname = location.hostname().unwrap();
        let mut port = location.port().unwrap();

        let is_localhost = hostname.eq("locahost") || hostname.eq("127.0.0.1");
        if is_localhost {
            hostname = "locahost".to_owned();
            port = "8081".to_owned();
        }

        let api_url = format!("http://api.{hostname}:{port}/");
        
        Self {
            is_localhost,
            api_url,
        }
    }
}