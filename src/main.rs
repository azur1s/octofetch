// API
use reqwest;
use reqwest::header::USER_AGENT;

// Args
use structopt::StructOpt;

// Misc
use termion::color;
use serde::{Serialize, Deserialize};
use serde_json;

mod content_box;

#[derive(StructOpt)]
struct Args {
    #[structopt(short = "u", long = "username")]
    username: String,
}

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

    let args = Args::from_args();

    let url = format!( "https://api.github.com/users/{}", args.username );
    
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

