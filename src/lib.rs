use crossterm::event::{self, Event, KeyCode};
use signal_hook::{consts::SIGHUP, iterator::Signals};
use std::error::Error;
use std::fmt;
use std::time::Duration;
use std::{num::ParseIntError, str::ParseBoolError};
use tokio::sync::mpsc::Receiver;
use tokio::time;
use std::io::{Write, Stdout};
use std::thread;

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
struct Timer<'a> {
    duration_ms: u64,
    time_remaining_ms: u64,
    output: &'a Stdout
    // finish_sound: String
}

impl<'a> Timer<'a> {
    fn new(duration: u64, output: &'a Stdout) -> Self {
        let duration_ms = duration * 1000;
        Self {
            duration_ms,
            output,
            time_remaining_ms: duration_ms,
        }
    }
    async fn start(&mut self) {
        if self.duration_ms == 0 {
            return;
        }

        self.display_progress().unwrap();
        let mut interval = time::interval(Duration::from_millis(1));
        loop {
            interval.tick().await;
            self.time_remaining_ms -= 1;
            if self.time_remaining_ms % 1000 == 0 {
                self.display_progress().unwrap();
            }
            if self.time_remaining_ms == 0 {
                break;
            }
        }
    }

    fn display_progress(&mut self) -> Result<(), Box<dyn Error>> {
        let msg = format!("Time Remaining: {}\r\n", self.time_remaining_ms/1000);
        thread::scope(|s| {
            s.spawn(|| {
                write!(self.output, "{}", msg).unwrap();
                self.output.flush().unwrap();
            });
        });
        Ok(())
    }

    fn toggle_pause(&self) {
        dbg!("Pause called on {}", self);
    }
}

pub async fn run(config: Config) -> Result<(), Box<dyn Error>> {
    dbg!("Running!");

    let mut paused = false;

    let stdout = std::io::stdout();
    // Create timers
    let gr_timer = Timer::new(config.get_ready.into(), &stdout);
    let first_timer = Timer::new(config.first_timer.into(), &stdout);
    // let second_timer = Timer::new(config.second_timer.into());
    // Put in vector
    // let mut timers = vec![gr_timer, first_timer, second_timer];
    let mut timers = vec![gr_timer, first_timer];

    let (pause_tx, pause_rx) = tokio::sync::mpsc::channel(1);
    let pause_tx2 = pause_tx.clone();

    dbg!("Done starting timers");

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
        _ = run_timers(&mut timers, pause_rx) => {
            dbg!("Done sleeping");
        }
        _ = sig_listener => {
            dbg!("sig_listener returned");
            paused = !paused;
            pause_tx.send(paused).await.unwrap();
        }
        _ = keeb_listener => {
            dbg!("keeb_listener returned");
            paused = !paused;
            pause_tx2.send(paused).await.unwrap();
        }
    }
    Ok(())
}

async fn run_timers(timers: &mut Vec<Timer<'_>>, mut pause_rx: Receiver<bool>) {
    for t in timers {
        println!("starting {t:?}");
        let ct = t.start();
        tokio::select! {
            _ = ct => {
            }
            _ = pause_rx.recv() => {
                dbg!("huh");
                t.toggle_pause();
            }
        }
        println!("finished {t:?}");
    }
}
