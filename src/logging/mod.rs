use chrono::Local;
use chrono::{NaiveDate, NaiveDateTime, Utc};
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
    pub fn log<T: AsRef<str>>(level: LogLevel, message: T) {
        println!(
            "[{}] [{}] {}",
            Logger::formatted_datetime(),
            level,
            message.as_ref()
        );
    }

    fn formatted_datetime() -> String {
        let now_local = Local::now();
        let naive_now = now_local.naive_local();
        naive_now.format("%d.%m.%Y %H:%M:%S.%3f").to_string()
    }
}
