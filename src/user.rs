use crate::consts::API_URL;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client, Error,
};

#[tokio::main]
pub async fn create_user() -> Result<(), Error> {
    let prompts = vec![
        "Enter your username:",
        "Enter your first name:",
        "Enter your last name:",
        "Enter your new password:",
    ];

    let mut inputs = vec![String::new(); prompts.len()];

    for (i, prompt) in prompts.iter().enumerate() {
        println!("{} ", prompt);
        std::io::stdin().read_line(&mut inputs[i]).unwrap();
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

    println!("{}", response.text().await?);

    Ok(())
}

#[tokio::main]
pub async fn view_profile(mut id: i32) -> Result<(), Error> {
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

    println!("{}", response.text().await?);

    Ok(())
}
