use crate::consts::API_URL;
use reqwest::Error;

#[tokio::main]
pub async fn create_user() -> Result<(), Error> {
    let client = reqwest::Client::new();

    let params = [
        ("username", "John"),
        ("firstname", "Ivanov"),
        ("lastname", "Ivanov"),
        ("newPassword", "Ivanov123"),
    ];
    let response = client
        .post(&format!("{API_URL}/users"))
        .form(&params)
        .send()
        .await?;

    println!("{}", response.text().await?);

    Ok(())
}
