#[macro_use]
extern crate diesel;

use clap::{Parser, Subcommand};

mod actions;
mod link;
mod models;
mod schema;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug)]
enum Action {
    Get {
        #[arg(short, long)]
        url: String,
    },
    Add {
        #[arg(short, long)]
        url: String,
        #[arg(short, long)]
        id: Option<String>
    }
}

fn main() {

    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let storage = link::Storage::new();
    let args = Args::parse();

    let url = match args.action {
        Action::Get { url } => storage.get_by_url(&url).unwrap(),
        Action::Add { url, id } => storage.put(url, id).unwrap()
    };

    println!("{:?}", url);
}