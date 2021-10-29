// API
use reqwest;
use reqwest::header::USER_AGENT;

// Misc
use termion::color;
use serde::{Serialize, Deserialize};
use serde_json;
use std::process;

mod content_box;

#[derive(Serialize, Deserialize)]
struct UserData {
    login: String,
    name: String,
    public_repos: i64,
    public_gists: i64,
    followers: i64,
    following: i64,
    html_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let username = std::env::args().nth(1).expect("No username given. Exiting...");

    // -v : print version and exit
    if username.eq("-v") {
        const VERSION: &'static str = env!("CARGO_PKG_VERSION");
        println!("octofetch v{}", VERSION);
        process::exit(1);
    }

    let url = format!( "https://api.github.com/users/{}", username );
    
    // Get the body of the request
    let client = reqwest::Client::new();
    let res = client.get(url).header(USER_AGENT, "octofetch cli").send().await?.text().await?;
    // The json of the api's body
    let user: UserData = serde_json::from_str(&res)?;
    
    let mut info = content_box::ContentBox{ pushed_lines: Vec::new(), longest_line: 0 };

    let main = color::Fg(color::Magenta);
    let accent = color::Fg(color::White);

    info.push(format!("{}Username: {}{}"  , main, accent, user.login));
    info.push(format!("{}Repos: {}{}"     , main, accent, user.public_repos));
    info.push(format!("{}Gists: {}{}"     , main, accent, user.public_gists));
    info.push(format!("{}Follower: {}{}"  , main, accent, user.followers));
    info.push(format!("{}Following: {}{}" , main, accent, user.following));
    info.push(format!("{}Url: {}{}"       , main, accent, user.html_url));

    println!("{}", info.to_string());

    Ok(())
}

