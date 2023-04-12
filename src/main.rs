use args::Args;
use clap::Parser;
use colored::*;
use logging::{logging, logout};
use task::{create_task, delete_task, get_tasks, update_task, view_task};
use user::show_profile;

mod args;
mod consts;
mod errors;
mod logging;
mod task;
mod user;

#[tokio::main]
async fn get_token() -> Result<(), reqwest::Error> {
    let params = [("username", "i"), ("password", "1234567")];

    let client = reqwest::Client::new();

    let response = client
        .post("http://demo2.z-bit.ee/users/get-token")
        .form(&params)
        .send()
        .await?;

    println!("{}", response.text().await?);

    Ok(())
}

fn main() {
    logging();

    let args = Args::parse();

    match args {
        Args {
            show_profile: true, ..
        } => show_profile(12).unwrap(),
        Args {
            create_task: true, ..
        } => create_task().unwrap(),
        Args {
            get_tasks: true, ..
        } => get_tasks().unwrap(),
        Args {
            delete_task: id, ..
        } if id > 0 => delete_task(id).expect("Invalid ID"),
        Args {
            update_task: id, ..
        } if id > 0 => update_task(id).expect("Invalid ID"),
        Args { view_task: id, .. } if id > 0 => view_task(id).expect("Invalid ID"),
        Args { logout: true, .. } => logout().unwrap(),
        _ => println!("{}", "-h for information about the commands".italic()),
    }
}
