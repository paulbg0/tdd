use crate::consts::API_URL;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client, Error,
};

#[tokio::main]
pub async fn create_task() -> Result<(), Error> {
    let prompts = vec!["Enter task title:", "Enter task description:"];

    let mut inputs = vec![String::new(); prompts.len()];

    for (i, prompt) in prompts.iter().enumerate() {
        println!("{} ", prompt);
        std::io::stdin().read_line(&mut inputs[i]).unwrap();
    }

    let title: &String = &inputs[0];
    let desc: &String = &inputs[1];

    let client = Client::new();

    let token = std::fs::read_to_string("token.txt").expect("Unable to read token.txt");

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
    );

    let params = [("title", &title), ("desc", &desc)];
    let response = client
        .post(&format!("{}/tasks", API_URL))
        .headers(headers)
        .form(&params)
        .send()
        .await?;

    println!("{}", response.text().await?);

    Ok(())
}

#[tokio::main]
pub async fn get_tasks() -> Result<(), Error> {
    let client = Client::new();

    let token: String = std::fs::read_to_string("token.txt").expect("Unable to read token.txt");

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
    );

    let response = client
        .get(&format!("{}/tasks", API_URL))
        .headers(headers)
        .send()
        .await?;

    println!("{}", response.text().await?);

    Ok(())
}

#[tokio::main]
pub async fn delete_task(id: u32) -> Result<(), Error> {
    let client = Client::new();

    let token: String = std::fs::read_to_string("token.txt").expect("Unable to read token.txt");

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
    );

    let response = client
        .delete(&format!("{}/tasks/{}", API_URL, id))
        .headers(headers)
        .send()
        .await?;

    println!("{}", response.text().await?);

    Ok(())
}
