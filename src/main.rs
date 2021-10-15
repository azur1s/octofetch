// API
use reqwest;
use reqwest::header::USER_AGENT;

// Misc
use serde::{Serialize, Deserialize};
use serde_json;

// TUI
use tui::widgets::*;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::text::{ Span, Spans};
use tui::style::{ Color, Style };
use tui::layout::Rect;
use crossterm::{ execute, terminal };
use std::io;

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

    let id = std::env::args().nth(1).expect("No username given :(");

    let url = format!( "https://api.github.com/users/{}", id );
    
    // Get the body of the request
    let client = reqwest::Client::new();
    let res = client.get(url).header(USER_AGENT, "rust cli").send().await?.text().await?;

    // The json of the api's body
    let user: UserData = serde_json::from_str(&res)?;

    let spans = vec![
        (" Username: ", user.login),
        ("    Repos: ", user.public_repos.to_string()),
        ("    Gists: ", user.public_gists.to_string()),
        ("Followers: ", user.followers.to_string()),
        ("Following: ", user.following.to_string()),
        ("     User: ", user.html_url)
    ];

    let data: Vec<Spans> = spans
        .iter()
        .map(|(s1, s2)| {
            Spans::from(vec![
                Span::styled(s1.to_string(), Style::default().fg(Color::Magenta)),
                Span::raw(s2)
            ])
        })
        .collect();
    
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    execute!(io::stdout(), terminal::Clear(terminal::ClearType::FromCursorUp))?;

    terminal.draw(|f| {

        const BORDER_OFFSET: u16 = 2;
        let size = Rect::new( 0, 0, 50, data.len() as u16 + BORDER_OFFSET );
        
        let fetch = Paragraph::new(data)
            .block(Block::default()
            .title(Span::styled(" octofetch ", Style::default().fg(Color::Magenta) ))
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded));

        f.render_widget(fetch, size);
        
    })?;

    Ok(())
}
