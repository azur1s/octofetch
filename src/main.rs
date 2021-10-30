use std::process;
use termion::color;

mod api;
mod content_box;

fn colorful_format(key: &str, text: String) -> String {
    let MAIN_COLOR = color::Fg(color::Magenta);
    let ACCENT_COLOR = color::Fg(color::White);

    return format!("{}{}: {}{}", MAIN_COLOR, key, ACCENT_COLOR, text);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().nth(1) == None {
	    eprintln!("No username given. Exiting...");
	    process::exit(1);
    }
    
    let username = std::env::args()
        .nth(1)
        .expect("No username given. Exiting...");

    // -v : print version and exit
    if username.eq("-v") {
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        println!("octofetch v{}", VERSION);
        process::exit(0);
    } else if username.eq("-h") {
        println!("Usage: octofetch <username>");
        process::exit(0);
    }

    let user = api::get(username).await?;

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
    info.push(colorful_format("Follower", user.followers.to_string()));
    info.push(colorful_format("Following", user.following.to_string()));
    info.push(colorful_format("Url", user.html_url));

    println!("{}", info.to_string());

    Ok(())
}
