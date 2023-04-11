use crate::consts::API_URL;
use crate::user::{self, create_user};
use colored::*;
use reqwest::Client;
use serde_json::Value;
use std::fs::File;
use std::io::{Error, Write};

pub fn register() -> Result<(), Error> {
    let filename = "token.txt";

    let new_token = &create_user().unwrap()["token"];

    if !std::path::Path::new(filename).exists() {
        let mut file = File::create(filename)?;
        writeln!(file, "{}", new_token)?;
    }

    Ok(())
}

#[tokio::main]
pub async fn login() -> Result<(), reqwest::Error> {
    let mut username = String::new();
    let mut password = String::new();

    println!("Enter your username");
    std::io::stdin().read_line(&mut username).unwrap();
    println!("Enter your password");
    std::io::stdin().read_line(&mut password).unwrap();

    let params = [("username", &username), ("password", &password)];

    let client = Client::new();

    let response = client
        .post(&format!("{}/users/get-token", API_URL))
        .form(&params)
        .send()
        .await?;

    if response.status().is_success() {
        let data: Value = serde_json::from_str(&response.text().await?).unwrap();
        let mut file = File::open("token.txt").unwrap();

        writeln!(file, "{}", data["access_token"].to_string()).unwrap();
    } else {
        let error: Value = serde_json::from_str(&response.text().await?).unwrap();

        println!(
            "{}",
            format!("Error: {}", error["message"].to_string())
                .red()
                .bold()
        );
    }

    Ok(())
}

pub fn logging() -> Result<(), Error> {
    let token = std::fs::read_to_string("token.txt").expect("Unable to read token.txt");

    if !token.len() == 32 {
        println!("{}", "You are not logged in!".red().bold());
        println!("{}", "Enter `r` to register `l` to login ".italic());

        let mut user_choice = String::new();
        loop {
            std::io::stdin().read_line(&mut user_choice)?;

            match user_choice.trim() {
                "r" => {
                    register()?;
                    break;
                }
                "l" => {
                    login().unwrap();
                    break;
                }
                _ => {
                    println!("{}", "Invalid input!".red().bold());
                }
            }
        }
    }

    Ok(())
}
