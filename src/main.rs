// API
use reqwest;
use reqwest::header::USER_AGENT;

// Args
use structopt::StructOpt;

// Misc
use termion::color;
use serde::{Serialize, Deserialize};
use serde_json;

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

    let main = color::Fg(color::Magenta);
    let accent = color::Fg(color::White);

    println!(" {}{} {}@ {}GitHub", main, user.login, accent, main);
    println!(" ----------");
    println!(" {}Repos: {}{}", main, accent, user.public_repos.to_string());
    println!(" {}Gists: {}{}", main, accent, user.public_gists.to_string());
    println!(" {}Followers: {}{}", main, accent, user.followers.to_string());
    println!(" {}Following: {}{}", main, accent, user.following.to_string());
    println!(" {}User: {}{}", main, accent, user.html_url);
    println!("");

    Ok(())
}
