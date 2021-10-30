use termion::color;
use std::process;

mod api;
mod content_box;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let username = std::env::args().nth(1).expect("No username given. Exiting...");

    // -v : print version and exit
    if username.eq("-v") {
        const VERSION: &'static str = env!("CARGO_PKG_VERSION");
        println!("octofetch v{}", VERSION);
        process::exit(1);
    }

    let user: api::UserData = api::get(username).await?;

    // Colors
    let main = color::Fg(color::Magenta);
    let accent = color::Fg(color::White);

    // The fetch
    let mut info = content_box::ContentBox{ pushed_lines: Vec::new(), longest_line: 0 };

    info.push(format!("{}Username: {}{}"  , main, accent, user.login));
    if user.bio != None {
        info.push(format!("{}Bio: {}{}"  , main, accent, user.bio.unwrap()));
    }
    info.push(format!("{}Repos: {}{}"     , main, accent, user.public_repos));
    info.push(format!("{}Gists: {}{}"     , main, accent, user.public_gists));
    info.push(format!("{}Follower: {}{}"  , main, accent, user.followers));
    info.push(format!("{}Following: {}{}" , main, accent, user.following));
    info.push(format!("{}Url: {}{}"       , main, accent, user.html_url));

    println!("{}", info.to_string());

    Ok(())
}
