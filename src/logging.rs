use crate::consts::API_URL;
use crate::user::create_user;
use colored::*;
use reqwest::Client;
use serde_json::Value;
use std::fs;
use std::io;

#[tokio::main]
async fn login() -> Result<(), reqwest::Error> {
    let mut username = String::new();
    let mut password = String::new();

    println!("Enter your username");
    std::io::stdin().read_line(&mut username).unwrap();
    println!("Enter your password");
    std::io::stdin().read_line(&mut password).unwrap();

    let params = [
        ("username", &username.trim()),
        ("password", &password.trim()),
    ];

    let client = Client::new();

    let response = client
        .post(&format!("{}/users/get-token", API_URL))
        .form(&params)
        .send()
        .await?;

    if response.status().is_success() {
        let data: Value = serde_json::from_str(&response.text().await?).unwrap();

        let token = data["access_token"].to_string().replace("\"", "");
        fs::write("token.txt", &token).unwrap();
    } else {
        let error: Value = serde_json::from_str(&response.text().await?).unwrap();

        println!(
            "{}",
            format!("Error: {}", error["message"].to_string())
                .red()
                .bold()
        );
        std::process::exit(1);
    }

    Ok(())
}

pub fn logging() {
    match fs::read_to_string("token.txt") {
        Ok(token) => token,
        Err(_) => {
            println!("Token not found. Would you like to register or login? (r/l)");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");
            if input.trim() == "r" {
                register().expect("Failed to register");
            } else if input.trim() == "l" {
                login().expect("Failed to login");
            } else {
                println!("Invalid input");
                std::process::exit(1);
            }
            fs::read_to_string("token.txt").expect("Failed to read token")
        }
    };
}

fn register() -> Result<(), io::Error> {
    let token = create_user().unwrap()["access_token"]
        .to_string()
        .replace("\"", "");

    fs::write("token.txt", &token)?;

    Ok(())
}

pub fn logout() -> Result<(), io::Error> {
    let mut user_choice = String::new();

    loop {
        println!("Are you sure you want to logout? (y/n)");

        std::io::stdin().read_line(&mut user_choice).unwrap();

        if user_choice.trim() == "y" {
            fs::remove_file("token.txt")?;
            break;
        } else if user_choice.trim() == "n" {
            break;
        } else {
            println!("Invalid input");
        }
    }

    Ok(())
}
