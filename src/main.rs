use std::{env, process};

fn main() {
    println!("Rust Implementation Of Interval Boss");
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Get ready is {}", config.get_ready);
    println!("First timer is {}", config.first_timer);
    println!("Second timer is {}", config.second_timer);
    println!("Rounds is {}", config.rounds);
    println!("One shot is {}", config.one_shot);

    run(config);
}

fn run(config: Config) {
    println!("Running!");
}

struct Config {
    get_ready: String,
    first_timer: String,
    second_timer: String,
    rounds: String,
    one_shot: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 6 {
            return Err("not enough arguments");
        }
        let get_ready = args[1].clone();
        let first_timer = args[2].clone();
        let second_timer = args[3].clone();
        let rounds = args[4].clone();
        let one_shot = args[5].clone();

        Ok(Config {
            get_ready,
            first_timer,
            second_timer,
            rounds,
            one_shot,
        })
    }
}
