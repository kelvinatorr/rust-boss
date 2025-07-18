use crossterm::event::{self, Event, KeyCode};
use signal_hook::{consts::SIGHUP, iterator::Signals};
use std::error::Error;
use std::fmt;
use std::time::Duration;
use std::{num::ParseIntError, str::ParseBoolError};
use tokio::time;

#[derive(Debug)]
pub struct ArgParseError {
    pub msg: String,
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

pub struct Config {
    get_ready: u32,
    first_timer: u32,
    second_timer: u32,
    rounds: u16,
    one_shot: bool,
}
impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Get Ready: {} | First Timer: {} | Second Timer: {} | Round: {} | One Shot: {}",
            self.get_ready, self.first_timer, self.second_timer, self.rounds, self.one_shot
        )
    }
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, ArgParseError> {
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

#[derive(Debug)]
struct Timer {
    duration: u64,
    // finish_sound: String
}

impl Timer {
    async fn start(&self) {
        if self.duration == 0 {
            return;
        }
        time::sleep(Duration::from_secs(self.duration)).await;
    }

    fn pause(&self) {
        dbg!("Pause called on {}", self);
    }
}

pub async fn run(config: Config) -> Result<(), Box<dyn Error>> {
    dbg!("Running!");

    // Create timers
    let gr_timer = Timer {
        duration: config.get_ready.into(),
    };
    let first_timer = Timer {
        duration: config.first_timer.into(),
    };
    let second_timer = Timer {
        duration: config.second_timer.into(),
    };
    // Put in vector
    let timers = vec![gr_timer, first_timer, second_timer];

    // Run each one in sequence
    let timer = tokio::spawn(async move {
        run_timers(timers).await;
    });
    dbg!("Done starting timer");

    let sig_listener = tokio::spawn(async {
        let mut signals = Signals::new([SIGHUP]).unwrap();
        for sig in signals.forever() {
            println!("Received signal {:?}", sig);
            return;
        }
    });

    let keeb_listener = tokio::spawn(async {
        crossterm::terminal::enable_raw_mode().unwrap();
        loop {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Char(' ') => {
                        dbg!("Got space...");
                        break;
                    }
                    _ => {
                        // Handle all other keys
                        dbg!("You pressed another key.");
                    }
                }
            }
        }
        crossterm::terminal::disable_raw_mode().unwrap();
    });

    tokio::select! {
        _ = timer => {
            dbg!("Done sleeping");
        }
        _ = sig_listener => {
            dbg!("sig_listener returned");
        }
        _ = keeb_listener => {
            dbg!("keeb_listener returned");
        }
    }
    Ok(())
}

async fn run_timers(timers: Vec<Timer>) {
    for t in &timers {
        println!("starting {t:?}");
        t.start().await;
        println!("finished {t:?}");
    }
}
