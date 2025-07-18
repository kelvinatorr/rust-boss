use std::{env, process};

use rustboss::Config;

#[tokio::main]
async fn main() {
    println!("Rust Implementation Of Interval Boss");
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err.msg);
        process::exit(1);
    });

    println!("Got config - {}", config);

    if let Err(e) = rustboss::run(config).await {
        println!("Application error: {e}");
        process::exit(1);
    }
    crossterm::terminal::disable_raw_mode().unwrap();
    process::exit(0);
}

