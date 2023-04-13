use crate::consts::API_URL;
use crate::errors::NotFound;
use colored::*;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client, Error,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

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

    loop {
        for (i, prompt) in prompts.iter().enumerate() {
            println!("{} ", prompt);
            std::io::stdin().read_line(&mut inputs[i]).unwrap();
        }

        if !inputs[0].trim().is_empty() {
            break;
        }

        println!(
            "{}",
            "Title cannot be empty. Please try again.".red().bold()
        );
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

    if response.status().is_success() {
        let task: Task = serde_json::from_str(&response.text().await?).unwrap();

        println!(
            "Task {}: {}Description: {}Created at: {}\nIs done: {}\n",
            task.id, task.title, task.desc, task.created_at, task.marked_as_done
        );
    } else {
        let err: Value = serde_json::from_str(&response.text().await?).unwrap();

        println!("Error: {}", err["error"]);
    }

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

    if response.status().is_success() {
        println!("{}", format!("Task {} was deleted", id).green().bold());
    } else {
        println!("{}", "Unable to delete task".red().bold());
    }

    Ok(())
}

#[tokio::main]
pub async fn update_task(id: u32) -> Result<(), Error> {
    let token = std::fs::read_to_string("token.txt").expect("Unable to read token.txt");

    let mut props: HashMap<&str, &str> = HashMap::new();

    let mut input = String::new();
    let mut prop_choice = String::new();
    println!(
        "{}",
        "What do you want to update? (title(t), description(d), is_done(i))".bold()
    );

    loop {
        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "title" | "t" => {
                println!("Enter the new title: ");
                std::io::stdin().read_line(&mut prop_choice).unwrap();
                props.insert("title", &prop_choice.trim());

                break;
            }
            "description" | "d" => {
                println!("Enter the new description: ");
                std::io::stdin().read_line(&mut prop_choice).unwrap();
                props.insert("desc", &prop_choice.trim());

                break;
            }
            "is_done" | "i" => {
                println!("Done (y/n)?");
                std::io::stdin().read_line(&mut prop_choice).unwrap();
                match prop_choice.trim() {
                    "y" => {
                        props.insert("marked_as_done", "1");
                    }
                    "n" => {
                        props.insert("marked_as_done", "0");
                    }
                    _ => {
                        println!("{}", "Invalid input!".red().bold());
                    }
                }
                break;
            }
            _ => {
                println!("{}", "Invalid input!".red().bold());
            }
        }
    }

    let client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
    );

    let response = client
        .put(&format!("{}/tasks/{}", API_URL, id))
        .headers(headers.clone())
        .json(&props)
        .send()
        .await?;

    if response.status().is_success() {
        let data: Value = serde_json::from_str(&response.text().await?).unwrap();

        println!(
            "{}",
            format!("Task {} was updated!", data["id"]).green().bold()
        );
    } else {
        let data: Value = serde_json::from_str(&response.text().await?).unwrap();

        println!("Error {}", data["message"].to_string().red().bold());
    }

    Ok(())
}
