// API
use reqwest::header::USER_AGENT;

// JSON stuff
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserData {
  pub login: String,
  pub name: Option<String>,
  pub bio: Option<String>,
  pub public_repos: i64,
  pub public_gists: i64,
  pub followers: i64,
  pub following: i64,
  pub html_url: String,
  pub location: Option<String>,
}

const GITHUB_ENDPOINT: &str = "https://api.github.com/users/";

pub async fn get(username: String) -> Result<UserData, Box<dyn std::error::Error>> {
  let url = format!("{}{}", GITHUB_ENDPOINT, username);

  // Get the body of the request
  let client = reqwest::Client::new();
  let res = client
    .get(url)
    .header(USER_AGENT, "octofetch cli")
    .send()
    .await?
    .text()
    .await?;
  // The json of the api's body
  let user: UserData = serde_json::from_str(&res)?;

  Ok(user)
}
