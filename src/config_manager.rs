use serde::Deserialize;
use std::fs;
use std::error::Error;

static DEFAULT_CONFIG: &str = r#"
  {
    "MainColor": { "Red": 255, "Green": 75, "Blue": 75 },
    "AccentColor": { "Red": 255, "Green": 255, "Blue": 255 }
  }
"#;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
  pub main_color: CustomColor,
  pub accent_color: CustomColor
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CustomColor {
  pub red: u8,
  pub green: u8,
  pub blue: u8,
}

/// Loads a config from file
/// ### Arguments
/// * `path` - Path to a custom config file
pub fn load_config(path: Option<&str>) -> Result<Config, Box<dyn Error>> {
  let str: String;
  if path == None {
    str = DEFAULT_CONFIG.to_string();
  } else {
    str = fs::read_to_string(path.unwrap())?.parse()?;
  }
  let conf: Config = serde_json::from_str(&str)?;
  Ok(conf)
}