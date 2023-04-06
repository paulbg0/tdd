use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(short, long)]
    pub create_user: bool,
}
