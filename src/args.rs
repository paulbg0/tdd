use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(long)]
    pub create_user: bool,

    #[arg(short, long)]
    pub view_profile: bool,

    #[arg(short, long)]
    pub create_task: bool,

    #[arg(short, long)]
    pub get_tasks: bool,

    #[arg(long, default_value = "0")]
    pub view_task: u32,

    #[arg(short, long, default_value = "0")]
    pub delete_task: u32,

    #[arg(short, long, default_value = "0")]
    pub update_task: u32,
}
