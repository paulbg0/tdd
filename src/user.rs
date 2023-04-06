use crate::consts::API_URL;
use reqwest::Error;

#[tokio::main]
pub async fn create_user() -> Result<(), Error> {
    let prompts = vec![
        "Enter your username:",
        "Enter your first name:",
        "Enter your last name:",
        "Enter your new password:",
    ];

    let mut inputs: Vec<String> = vec![String::new(); prompts.len()];

    for (i, prompt) in prompts.iter().enumerate() {
        println!("{} ", prompt);
        std::io::stdin().read_line(&mut inputs[i]).unwrap();
    }

    let username: &String = &inputs[0];
    let first_name: &String = &inputs[1];
    let last_name: &String = &inputs[2];
    let new_password: &String = &inputs[3];

    let client = reqwest::Client::new();

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
