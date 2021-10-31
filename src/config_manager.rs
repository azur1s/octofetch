use serde::Deserialize;
use std::fs;
use std::error::Error;

static DEFAULT_CONFIG: &str = r#"
  {
    "MainColor": { "Red": 255, "Green": 75, "Blue": 75 },
    "AccentColor": { "Red": 255, "Green": 255, "Blue": 255 }
  }
"#;

/// The structure of Octofetch config
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
  /// The main color across the program
  pub main_color: CustomColor,
  /// The accent color across the program
  pub accent_color: CustomColor
}

/// The Custom color structure used to create custom color objects that can then be parsed for `crossterm::style::Color`
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CustomColor {
  /// Red (0 - 255)
  pub red: u8,
  /// Green (0 - 255)
  pub green: u8,
  /// Blue (0 - 255)
  pub blue: u8,
}

/// Loads a config from file and returns it. If load fails, an error is thrown.
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