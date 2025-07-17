use std::{env, process};

use rustboss::Config;

fn main() {
    println!("Rust Implementation Of Interval Boss");
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err.msg);
        process::exit(1);
    });

    println!("Got config - {}", config);
    if let Err(e) = rustboss::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

