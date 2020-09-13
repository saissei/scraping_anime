// extern crate config;
extern crate serde_json;
use std::env;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Deserialize)]
pub struct AnimeConfig {
  pub url: String,
}

// use config::{Config, ConfigError, File, FileFormat};

/* impl Settings {
  pub fn new() -> Result<Self, ConfigError> {
    let mut c = Config::new();

    // Add 'Settings.toml'
    c.merge(File::new("config/Settings.json", FileFormat::Json).required(false))
      .unwrap();

    // Add 'Settings.$(RUST_ENV).toml`
    let name = format!(
      "Settings.{}",
      env::var("env").unwrap_or("development".into())
    );
    c.merge(File::new(&name, FileFormat::Json).required(false))
      .unwrap();
    c.try_into::<AnimeConfig>().unwrap();
    println!("{:#?}", &c.try_into::<AnimeConfig>().unwrap());
  }
} */

fn load_env() -> String {
  let mode = match env::var("APP_ENV") {
    Ok(value) => value.to_lowercase(),
    Err(_) => "test".to_string(),
  };
  return mode;
}

pub fn load_config() -> AnimeConfig {
  let mode = load_env();
  let file_name = "config/".to_string() + &mode + ".json";
  println!("{}", &file_name);
  let file = File::open(file_name).unwrap();
  let reader = BufReader::new(file);
  let conf: AnimeConfig = serde_json::from_reader(reader).unwrap();
  return conf;
}
