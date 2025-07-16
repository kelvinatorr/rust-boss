use std::error::Error;
use std::fmt;
use std::{env, num::ParseIntError, process, str::ParseBoolError};

#[derive(Debug)]
struct ArgParseError {
    msg: String,
}
impl fmt::Display for ArgParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Something went wrong!")
    }
}
impl From<ParseIntError> for ArgParseError {
    fn from(err: ParseIntError) -> Self {
        ArgParseError {
            msg: err.to_string(),
        }
    }
}
impl From<ParseBoolError> for ArgParseError {
    fn from(err: ParseBoolError) -> Self {
        ArgParseError {
            msg: err.to_string(),
        }
    }
}
impl Error for ArgParseError {}

fn main() {
    println!("Rust Implementation Of Interval Boss");
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err.msg);
        process::exit(1);
    });

    println!("Get ready is {}", config.get_ready);
    println!("First timer is {}", config.first_timer);
    println!("Second timer is {}", config.second_timer);
    println!("Rounds is {}", config.rounds);
    println!("One shot is {}", config.one_shot);

    run(config);
}

fn run(_config: Config) {
    println!("Running!");
}

struct Config {
    get_ready: u32,
    first_timer: u32,
    second_timer: u32,
    rounds: u16,
    one_shot: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, ArgParseError> {
        if args.len() < 6 {
            return Err(ArgParseError {
                msg: String::from("not enough arguments"),
            });
        }
        let get_ready: u32 = args[1].trim().parse()?;
        let first_timer: u32 = args[2].trim().parse()?;
        let second_timer: u32 = args[3].trim().parse()?;
        let rounds: u16 = args[4].trim().parse()?;
        let one_shot: bool = args[5].trim().parse()?;

        Ok(Config {
            get_ready,
            first_timer,
            second_timer,
            rounds,
            one_shot,
        })
    }
}
