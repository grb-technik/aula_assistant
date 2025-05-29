use chrono::Local;
use colored::{ColoredString, Colorize};

#[derive(Debug)]
pub enum LogLevel {
    DEBUG,
    INFO,
    WARN,
    ERROR,
    FATAL,
}

impl ToString for LogLevel {
    fn to_string(&self) -> String {
        match self {
            LogLevel::DEBUG => String::from("DEBUG"),
            LogLevel::INFO => String::from("INFO"),
            LogLevel::WARN => String::from("WARN"),
            LogLevel::ERROR => String::from("ERROR"),
            LogLevel::FATAL => String::from("FATAL"),
        }
    }
}

impl LogLevel {
    fn to_colored_string(&self) -> ColoredString {
        match self {
            LogLevel::DEBUG => self.to_string().blue().bold(),
            LogLevel::INFO => self.to_string().green().bold(),
            LogLevel::WARN => self.to_string().yellow().bold(),
            LogLevel::ERROR => self.to_string().red().bold(),
            LogLevel::FATAL => self.to_string().bright_red().bold(),
        }
    }
}

pub fn log(level: LogLevel, message: &str) {
    const DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    let log_str = format!(
        "{} [{}] {}",
        level.to_colored_string(),
        Local::now().format(DATETIME_FORMAT),
        message
    );

    println!("{}", log_str);
    // TODO: impl. file logging as well
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::log($crate::LogLevel::DEBUG, &format!($($arg)*));
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::log($crate::LogLevel::INFO, &format!($($arg)*));
    };
}

#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => {
        $crate::log($crate::LogLevel::WARN, &format!($($arg)*));
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::log($crate::LogLevel::ERROR, &format!($($arg)*));
    };
}

#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => {
        $crate::log($crate::LogLevel::FATAL, &format!($($arg)*));
    };
}
