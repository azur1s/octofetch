use std::process;
use crossterm::style::{ SetForegroundColor, Color, ResetColor};
use config_manager::CustomColor;

mod api;
mod content_box;
mod config_manager;

/// Parses a string with predetermined colors
/// ### Arguments
///
/// * `key` - The key of the value
/// * `text` - The content
/// * `key_color` - The foreground color of the key
/// * `text_color` - The foreground color of the content
/// 
fn colorful_format(key: &str, text: &String, key_color: Color, text_color: Color) -> String {
  // One would preferably want to use colors that are ANSI to avoid 
  // issues with older/primitive terminals with limited color support
  let main_color = SetForegroundColor(key_color);
  let acccent_color = SetForegroundColor(text_color);
  return format!("{}{}: {}{}{}", main_color, key, acccent_color, text, ResetColor);
}

/// Parses a color from the CustomColor and returns Color
/// ### Arguments
/// * `color` - The CustomColor to be parsed
fn parse_color(color: CustomColor) -> Color {
  return Color::Rgb { r: color.red, g: color.green, b: color.blue };
}

/// Gets the amount of digits the rgb values have in total and returns it
/// ### Arguments
/// * `color` - The CustomColor object where the rgb values are fetched
fn color_char_count(color: &CustomColor) -> usize {
  let mut char_count = 0;
  char_count += color.red.to_string().len();
  char_count += color.green.to_string().len();
  char_count += color.blue.to_string().len();
  return char_count;
}

const HELP_MESSAGE: &str = "\
Usage:
    octofetch <username>
Other:
    -v          Print version and exit.
    -h          Print help and exit.
    -c <path>   Loads a custom config file
";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  
  if std::env::args().nth(1) == None {
    eprintln!("No username given, use <octofetch -h> for more info.");
    process::exit(1);
  }
  // Get the first argument
  let user_to_search = std::env::args().nth(1)
    .expect("No username given, use <octofetch -h> for more info.");
  
  if user_to_search.is_empty() { 
    process::exit(1);
  }

  let mut config_path: Option<&str> = None;

  // TODO: create pipeline for multiple arguments
  let args: Vec<String> = std::env::args().collect();
  let mut index: usize = 0;
  while args.len() > index {
    let arg: &str = &args[index];
    match arg {
      "-v" => {
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        println!("octofetch v{}", VERSION);
        process::exit(0);
      },
      "-h" => {
        print!("{}", HELP_MESSAGE);
        process::exit(0);
      },
      "-c" => {
        index = index + 1;
        if index < args.len() {
          config_path = Some(&args[index]);
          index = index + 1;
        } else {
          println!("No config path specified, use <octofetch -h> for more info. Using default config.");
        }
      },
      _ => index = index + 1,
    }
  }
  
  let user = api::get(user_to_search).await?;

  if user.login.is_empty() {
    println!("User not found");
    process::exit(0);
  }

  // Load the config from file
  let config = config_manager::load_config(config_path)?;
  // Parse the colors
  let total_chars = color_char_count(&config.main_color) + color_char_count(&config.accent_color);
  let main_color = parse_color(config.main_color);
  let accent_color = parse_color(config.accent_color);

  // The fetch
  let mut info = content_box::ContentBox {
    pushed_lines: Vec::new(),
    longest_line: 0,
    static_reduction: 24 + total_chars,
  };
  info.push(colorful_format("Username", &user.login, main_color, accent_color));
  if user.bio != None {
    info.push(colorful_format("Bio", &user.bio.unwrap(), main_color, accent_color));
  }
  info.push(colorful_format("Repos", &user.public_repos.to_string(), main_color, accent_color));
  info.push(colorful_format("Gists", &user.public_gists.to_string(), main_color, accent_color));
  info.push(colorful_format("Followers", &user.followers.to_string(), main_color, accent_color));
  info.push(colorful_format("Following", &user.following.to_string(), main_color, accent_color));
  if user.location != None {
    info.push(colorful_format("Location", &user.location.unwrap(), main_color, accent_color));
  }
  info.push(colorful_format("Url", &user.html_url, main_color, accent_color));

  println!("{}", info.to_string().trim_end());

  Ok(())
}
