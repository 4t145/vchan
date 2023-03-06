use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub r2d2: R2d2Config
}


impl Config {
    pub fn load() -> Self {
        let cfg_file = std::fs::read(env!("CONFIG")).map(String::from_utf8).expect("fail to decode config file into utf8").expect("fail to read config file");
        toml::de::from_str(&cfg_file).expect("fail to deserialize config")
    }
}


#[derive(Debug, Clone, Deserialize)]
pub struct R2d2Config {
    pub threads: usize,
    pub max_connection: usize,
    pub db_url: String
}

