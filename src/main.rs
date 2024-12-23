mod handler;
mod model;
use std::env;

use handler::{display_user_events, fetch_user_events};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);
    if args.len() < 2 {
        eprintln!("Usage: cargo run <username>");
        return Ok(());
    }

    let username = &args[1];
    println!("Fetching events for user: {}", username);

    match username.as_str().len() {
        0 => eprintln!("Please input a valid username"),
        _ => {
            let events = fetch_user_events(&username).await?;
            display_user_events(events);
        },
    }

   return Ok(());
}
