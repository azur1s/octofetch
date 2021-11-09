use serde::Deserialize;
use std::fs;
use std::error::Error;

static DEFAULT_CONFIG: &str = r#"
  {
    "Header": " octofetch ",
    "Separator": ":",
    "Border": true,
    "MainColor": { "Red": 255, "Green": 0, "Blue": 255 },
    "AccentColor": { "Red": 255, "Green": 255, "Blue": 255 }
  }
"#;

/// The structure of Octofetch config
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
  /// The header on top of the border
  pub header: String,
  /// The separator between the key and value
  pub separator: String,
  /// Display the border or not
  pub border: bool,
  /// The main color across the program
  pub main_color: CustomColor,
  /// The accent color across the program
  pub accent_color: CustomColor,
  /// The key's texts
  pub keys: Keys
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

/// The key's text
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Keys {
  pub login: String,
  pub name: String,
  pub bio: String,
  pub public_repos: String,
  pub public_gists: String,
  pub followers: String,
  pub following: String,
  pub html_url: String,
  pub blog: String,
  pub location: String,
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