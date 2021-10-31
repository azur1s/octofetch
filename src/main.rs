use std::process;
use termion::color;

mod api;
mod content_box;

/// Parses a string with predetermined colors
/// ### Arguments
///
/// * `key` - The key of the value
/// * `text` - The content
/// 
/// ### Todo
/// Make config file for colors in .config directory
fn colorful_format(key: &str, text: String) -> String {
  let main_color = color::Fg(color::Magenta);
  let acccent_color = color::Fg(color::White);
  return format!("{}{}: {}{}", main_color, key, acccent_color, text);
}

const HELP_MESSAGE: &str = "\
Usage:
    octofetch <username>
Other:
    -v    Print version and exit.
    -h    Print help and exit.
";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  
  if std::env::args().nth(1) == None {
    eprintln!("No username given, use <octofetch -h> for more info.");
    process::exit(1);
  }
  // Get the first argument
  let arg = std::env::args().nth(1)
    .expect("No username given, use <octofetch -h> for more info.");
  
  if arg.is_empty() { 
    process::exit(1);
  }

  // this might be scuffed
  if arg.eq("-v") {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    println!("octofetch v{}", VERSION);
    process::exit(0);
  } else if arg.eq("-h") {
    print!("{}", HELP_MESSAGE);
    process::exit(0);
  }
  
  let user = api::get(arg).await?;

  if user.login.is_empty() {
    println!("User not found");
    process::exit(0);
  }

  // The fetch
  let mut info = content_box::ContentBox {
    pushed_lines: Vec::new(),
    longest_line: 0,
  };
  info.push(colorful_format("Username", user.login));
  if user.bio != None {
    info.push(colorful_format("Bio", user.bio.unwrap()));
  }
  info.push(colorful_format("Repos", user.public_repos.to_string()));
  info.push(colorful_format("Gists", user.public_gists.to_string()));
  info.push(colorful_format("Followers", user.followers.to_string()));
  info.push(colorful_format("Following", user.following.to_string()));
  if user.location != None {
    info.push(colorful_format("Location", user.location.unwrap()));
  }
  info.push(colorful_format("Url", user.html_url));

  println!("{}", info.to_string().trim_end());

  Ok(())
}
