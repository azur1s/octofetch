// API
use reqwest;
use reqwest::header::USER_AGENT;

// JSON stuff
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct UserData {
    pub login: String,
    pub name: String,
    pub bio: Option<String>,
    pub public_repos: i64,
    pub public_gists: i64,
    pub followers: i64,
    pub following: i64,
    pub html_url: String,
}

pub async fn get(username: String) -> Result<UserData, Box<dyn std::error::Error>> {

    let url = format!( "https://api.github.com/users/{}", username );
    
    // Get the body of the request
    let client = reqwest::Client::new();
    let res = client.get(url).header(USER_AGENT, "octofetch cli").send().await?.text().await?;
    // The json of the api's body
    let user: UserData = serde_json::from_str(&res)?;

    Ok(user)

}
