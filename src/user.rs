use crate::consts::API_URL;
use colored::Colorize;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client, Error,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: i32,
    username: String,
    firstname: String,
    lastname: String,
    created_at: String,
}

#[tokio::main]
pub async fn create_user() -> Result<Value, Error> {
    let prompts = vec![
        "Enter your username:",
        "Enter your first name:",
        "Enter your last name:",
        "Enter your new password:",
    ];

    let mut inputs = vec![String::new(); prompts.len()];

    loop {
        for (i, prompt) in prompts.iter().enumerate() {
            println!("{} ", prompt);
            std::io::stdin().read_line(&mut inputs[i]).unwrap();
        }

        if inputs[3].trim().len() > 6 {
            break;
        }

        println!(
            "{}",
            "Password's length should be more than 6 symbols."
                .red()
                .bold()
        );
    }

    let username: &String = &inputs[0];
    let first_name: &String = &inputs[1];
    let last_name: &String = &inputs[2];
    let new_password: &String = &inputs[3];

    let client = Client::new();

    let params = [
        ("username", &username),
        ("firstname", &first_name),
        ("lastname", &last_name),
        ("newPassword", &new_password),
    ];

    let response = client
        .post(&format!("{}/users", API_URL))
        .form(&params)
        .send()
        .await?;

    let user: Value = serde_json::from_str(&response.text().await?).expect("Unable to parse JSON");

    Ok(user)
}

#[tokio::main]
pub async fn show_profile(mut id: i32) -> Result<(), Error> {
    // ? hardcode id for now
    id = 542;
    let client = Client::new();

    let token: String = std::fs::read_to_string("token.txt").expect("Unable to read token.txt");

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
    );

    let response = client
        .get(&format!("{}/users/{}", API_URL, id))
        .headers(headers)
        .send()
        .await?;

    let response_text = response.text().await?;
    let user: User = serde_json::from_str(&response_text).unwrap();

    println!(
        "{}\tUsername: {}\tFirst name: {}\tLast name: {}\tCreated at: {}",
        format!("ID: {}", user.id).purple().bold(),
        user.username,
        user.firstname,
        user.lastname,
        user.created_at
    );

    Ok(())
}
