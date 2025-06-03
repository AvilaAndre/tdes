use clap::ValueEnum;
use paste::paste;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display, Formatter},
    fs::{File, OpenOptions},
    io::{self, BufWriter, Write},
    path::Path,
};

use super::Context;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, ValueEnum, Serialize, Deserialize)]
pub enum LoggerLevel {
    Debug = 0,
    Info = 1,
    Warn = 2,
    Error = 3,
    Internal = 4,
}

impl Display for LoggerLevel {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let text = match self {
            LoggerLevel::Debug => "DEBUG",
            LoggerLevel::Info => "INFO",
            LoggerLevel::Warn => "WARN",
            LoggerLevel::Error => "ERROR",
            LoggerLevel::Internal => "SIMUL",
        };
        write!(f, "{text}")
    }
}

pub struct Logger {
    level: LoggerLevel,
    writer: Option<BufWriter<File>>,
    flush_threshold: usize,
    unflushed_count: usize,
}

impl Logger {
    pub fn new(level: LoggerLevel) -> Self {
        Self {
            level,
            writer: None,
            flush_threshold: 200,
            unflushed_count: 0,
        }
    }

    pub fn set_file<P: AsRef<Path>>(&mut self, file_path: P) -> io::Result<()> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(file_path)?;

        self.writer = Some(BufWriter::new(file));
        Ok(())
    }

    pub fn close_file(&mut self) {
        if let Some(mut writer) = self.writer.take() {
            writer.flush().ok();
        }
    }

    fn enabled(&self, level: LoggerLevel) -> bool {
        self.level <= level
    }

    fn write_to_file(&mut self, message: &str, level: LoggerLevel) {
        if let Some(ref mut writer) = self.writer {
            if writeln!(writer, "{}", message).is_ok() {
                self.unflushed_count += 1;

                let should_flush_immediately =
                    matches!(level, LoggerLevel::Error | LoggerLevel::Internal);

                let should_flush_threshold = self.unflushed_count >= self.flush_threshold;

                if should_flush_immediately || should_flush_threshold {
                    writer.flush().ok();
                    self.unflushed_count = 0;
                }
            }
        }
    }
}

fn ctx_log(ctx: &mut Context, level: LoggerLevel, text: impl AsRef<str>) {
    if ctx.logger.enabled(level) {
        let message = format!("[{}] [{}] {}", ctx.clock, level, text.as_ref());

        match ctx.logger.writer {
            Some(_) => ctx.logger.write_to_file(&message, level),
            None => println!("{}", message),
        };
    }
}

fn global_log(level: LoggerLevel, text: impl AsRef<str>) {
    println!("[GLOBAL] [{}] {}", level, text.as_ref());
}

macro_rules! define_log_fn {
    ($name:ident, $level:expr) => {
        paste! {
            pub fn $name(ctx: &mut Context, text: impl AsRef<str>) {
                ctx_log(ctx, $level, text);
            }

            pub fn [<global_ $name>](text: impl AsRef<str>) {
                global_log($level, text);
            }
        }
    };
}

define_log_fn!(debug, LoggerLevel::Debug);
define_log_fn!(info, LoggerLevel::Info);
define_log_fn!(warn, LoggerLevel::Warn);
define_log_fn!(error, LoggerLevel::Error);
define_log_fn!(internal, LoggerLevel::Internal);
