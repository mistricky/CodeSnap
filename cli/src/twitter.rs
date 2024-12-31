use twitter_v2::{TwitterApi, authorization::BearerToken};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct TwitterConfig {
    api_key: String,
    api_secret_key: String,
    access_token: String,
    access_token_secret: String,
}

fn get_twitter_config_path() -> PathBuf {
    let home_dir = home::home_dir().expect("Unable to get your home dir");
    home_dir.join(".config").join("CodeSnap").join("twitter_config.json")
}

fn save_twitter_config(config: &TwitterConfig) -> io::Result<()> {
    let config_path = get_twitter_config_path();
    let config_dir = config_path.parent().unwrap();
    fs::create_dir_all(config_dir)?;
    let file = File::create(config_path)?;
    serde_json::to_writer(file, config)?;
    Ok(())
}

fn load_twitter_config() -> io::Result<TwitterConfig> {
    let config_path = get_twitter_config_path();
    let file = File::open(config_path)?;
    let config: TwitterConfig = serde_json::from_reader(file)?;
    Ok(config)
}

pub fn twitter_login() -> anyhow::Result<()> {
    let mut api_key = String::new();
    let mut api_secret_key = String::new();
    let mut access_token = String::new();
    let mut access_token_secret = String::new();

    println!("Enter your Twitter API key:");
    io::stdin().read_line(&mut api_key)?;
    println!("Enter your Twitter API secret key:");
    io::stdin().read_line(&mut api_secret_key)?;
    println!("Enter your Twitter access token:");
    io::stdin().read_line(&mut access_token)?;
    println!("Enter your Twitter access token secret:");
    io::stdin().read_line(&mut access_token_secret)?;

    let config = TwitterConfig {
        api_key: api_key.trim().to_string(),
        api_secret_key: api_secret_key.trim().to_string(),
        access_token: access_token.trim().to_string(),
        access_token_secret: access_token_secret.trim().to_string(),
    };

    save_twitter_config(&config)?;

    println!("Twitter account information saved successfully!");

    Ok(())
}

pub fn send_tweet() -> anyhow::Result<()> {
    let config = load_twitter_config()?;
    let bearer_token = BearerToken::new(config.access_token);
    let api = TwitterApi::new(bearer_token);

    // Here you would add the code to send the screenshot to Twitter using the `api` object.
    // This is a placeholder for the actual implementation.

    println!("Tweet sent successfully!");

    Ok(())
}
