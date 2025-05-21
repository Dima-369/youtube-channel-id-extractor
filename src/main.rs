use clap::{Arg, Command};
use reqwest::blocking::Client;
use url::Url;

fn extract_handle(url: &str) -> Option<String> {
    Url::parse(url)
        .ok()?
        .path_segments()?
        .find(|s| s.starts_with('@'))
        .map(|s| s.to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("YouTube Channel ID Extractor")
        .version("1.0")
        .author("Your Name <your_email@example.com>")
        .about("Extracts YouTube Channel ID from a URL")
        .arg(
            Arg::new("url")
                .help("The YouTube channel URL")
                .required(true),
        )
        .get_matches();

    let api_key = "AIzaSyAJlj1c-bcQSCxUOutGUVR3uOzdjHwk3Xk";

    // Get the URL from the command line arguments
    let url = matches.get_one::<String>("url").unwrap();

    // Extract YouTube handle
    let handle = extract_handle(url).ok_or("Invalid URL format")?;

    // Call YouTube API
    let client = Client::new();
    let response = client
        .get("https://www.googleapis.com/youtube/v3/search")
        .query(&[
            ("part", "snippet"),
            ("type", "channel"),
            ("q", &handle.replace('@', "")),
            ("key", &api_key),
        ])
        .send()?
        .json::<serde_json::Value>()?;

    // Extract channel ID from API response
    let channel_id = response["items"][0]["id"]["channelId"]
        .as_str()
        .ok_or("Channel ID not found")?;

    println!("{}", channel_id);
    Ok(())
}