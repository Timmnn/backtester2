use colored::Colorize;
use std::fmt::Display;

pub struct Logger {}

pub enum LogLevel {
    Info,
    Debug,
    Error,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LogLevel::Info => "INFO".white(),
                LogLevel::Debug => "DEBUG".magenta(),
                LogLevel::Error => "ERROR".red(),
            }
        )
    }
}

impl Logger {
    pub fn log(level: LogLevel, message: &str) -> () {
        println!("[{}] {}", level, message)
    }
}
