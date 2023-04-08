use args::Args;
use clap::Parser;
use colored::*;
use task::{create_task, delete_task, get_tasks, view_task};
use user::{create_user, view_profile};

mod args;
mod consts;
mod errors;
mod task;
mod user;

fn main() {
    let args = Args::parse();

    match args {
        Args {
            create_user: true, ..
        } => create_user().unwrap(),
        Args {
            view_profile: true, ..
        } => view_profile(12).unwrap(),
        Args {
            create_task: true, ..
        } => create_task().unwrap(),
        Args {
            get_tasks: true, ..
        } => get_tasks().unwrap(),
        Args {
            delete_task: id, ..
        } if id > 0 => delete_task(id).expect("Invalid ID"),
        Args { view_task: id, .. } if id > 0 => view_task(id).expect("Invalid ID"),
        _ => println!(
            "{}\n{}",
            "No valid arguments provided.".red().bold(),
            "Please use --h for more information.".italic()
        ),
    }
}
