// API
use reqwest;
use reqwest::header::USER_AGENT;

// Args
use structopt::StructOpt;

// Misc
use termion::color;
use serde::{Serialize, Deserialize};
use serde_json;
use std::fmt;

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

struct ContentBox {
    pushed_lines: Vec<String>,
    longest_line: usize,
}

impl fmt::Display for ContentBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "╭{}╮\n", "─".repeat(self.longest_line + 2 - 18));

        for pushed_line in self.pushed_lines.iter() {
            write!(f, "│{}{} │\n", pushed_line, " ".repeat(self.longest_line - pushed_line.len() + 1));
        }

        write!(f, "╰{}╯\n", "─".repeat(self.longest_line + 2 - 18))
    }
}

impl ContentBox {
    fn push(&mut self, line: String) {
        let new_line = line.len();
        self.pushed_lines.push(line);
        if new_line > self.longest_line {
            self.longest_line = new_line;
        }
    }
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
    
    let mut info = ContentBox{ pushed_lines: Vec::new(), longest_line: 0 };

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

