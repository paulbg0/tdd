use crate::consts::API_URL;
use crate::errors::NotFound;
use colored::*;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client, Error,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    title: String,
    desc: String,
    marked_as_done: bool,
    created_at: String,
}

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

    let response_text = response.text().await?;
    let task: Task = serde_json::from_str(&response_text).unwrap();

    if task.title != "" {
        println!(
            "{}",
            format!("Task {} was created", task.id).purple().bold()
        );
    } else {
        println!("{}", "Unable to create task".red().bold());
    }

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

    let response_text = response.text().await?;
    let tasks: Vec<Task> = serde_json::from_str(&response_text).unwrap();

    if !tasks.is_empty() {
        println!(
            "{}",
            format!("{} {}", tasks.len().to_string(), "task(s) found\n")
                .purple()
                .bold(),
        );

        for task in tasks {
            println!(
                "Task {}: {}Description: {}Created at: {}\nIs done: {}\n",
                task.id, task.title, task.desc, task.created_at, task.marked_as_done
            );
        }
    } else {
        println!("{}", "No tasks found".red().bold());
    }

    Ok(())
}

#[tokio::main]
pub async fn view_task(id: u32) -> Result<(), Error> {
    let client = Client::new();

    let token: String = std::fs::read_to_string("token.txt").expect("Unable to read token.txt");

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
    );

    let response = client
        .get(&format!("{}/tasks/{}", API_URL, id))
        .headers(headers)
        .send()
        .await?;

    let response_text = response.text().await?;

    let task: Task = serde_json::from_str(&response_text).unwrap();

    println!(
        "Task {}: {}Description: {}Created at: {}\nIs done: {}\n",
        task.id, task.title, task.desc, task.created_at, task.marked_as_done
    );

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

    let status = response.status();
    let response_text = response.text().await?;

    match status {
        reqwest::StatusCode::NOT_FOUND => {
            let not_found: NotFound = serde_json::from_str(&response_text).unwrap();

            println!("{}", not_found.message.red().bold());
        }
        _ => {
            let task: Task = serde_json::from_str(&response_text).unwrap();

            println!(
                "{}",
                format!("Task {} was deleted", task.id).purple().bold()
            );
        }
    }
    Ok(())
}
