use clap::ValueEnum;
use paste::paste;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    fmt::{self, Display, Formatter},
    fs::{self, File, OpenOptions},
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
    log_writer: Option<BufWriter<File>>,
    metrics_writer: Option<BufWriter<File>>,
    flush_threshold: usize,
    log_unflushed_count: usize,
}

impl Logger {
    pub fn new(level: LoggerLevel) -> Self {
        Self {
            level,
            log_writer: None,
            metrics_writer: None,
            flush_threshold: 200,
            log_unflushed_count: 0,
        }
    }

    pub fn set_flush_threshold(&mut self, new_threshold: usize) {
        self.flush_threshold = new_threshold
    }

    fn open_file<P: AsRef<Path>>(&mut self, file_path: P) -> io::Result<File> {
        // Create parent directories if they don't exist
        if let Some(parent) = file_path.as_ref().parent() {
            fs::create_dir_all(parent)?;
        }

        OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(file_path)
    }

    pub fn set_log_file<P: AsRef<Path>>(&mut self, file_path: P) -> io::Result<()> {
        let file = self.open_file(file_path)?;

        self.log_writer = Some(BufWriter::new(file));
        Ok(())
    }

    pub fn set_metrics_file<P: AsRef<Path>>(&mut self, file_path: P) -> io::Result<()> {
        let file = self.open_file(file_path)?;

        self.metrics_writer = Some(BufWriter::new(file));
        Ok(())
    }

    pub fn close_log_file(&mut self) {
        if let Some(mut writer) = self.log_writer.take() {
            writer.flush().ok();
        }
        self.log_unflushed_count = 0;
    }

    pub fn close_metrics_file(&mut self) {
        if let Some(mut writer) = self.metrics_writer.take() {
            writer.flush().ok();
        }
        self.log_unflushed_count = 0;
    }

    fn enabled(&self, level: LoggerLevel) -> bool {
        self.level <= level
    }

    fn write_to_log_file(&mut self, message: &str, level: LoggerLevel) {
        if let Some(ref mut writer) = self.log_writer {
            if writeln!(writer, "{}", message).is_ok() {
                self.log_unflushed_count += 1;

                let should_flush_immediately =
                    matches!(level, LoggerLevel::Error | LoggerLevel::Internal);

                let should_flush_threshold = self.log_unflushed_count >= self.flush_threshold;

                if should_flush_immediately || should_flush_threshold {
                    writer.flush().ok();
                    self.log_unflushed_count = 0;
                }
            }
        }
    }

    fn write_to_metrics_file(&mut self, metrics: &Value) {
        if let Some(ref mut metrics_writer) = self.metrics_writer {
            if writeln!(metrics_writer, "{}", metrics).is_ok() {
                metrics_writer.flush().ok();
            }
        }
    }
}

fn ctx_log(ctx: &mut Context, level: LoggerLevel, text: impl AsRef<str>) {
    if ctx.logger.enabled(level) {
        let message = format!("[{}] [{}] {}", ctx.clock, level, text.as_ref());

        match ctx.logger.log_writer {
            Some(_) => ctx.logger.write_to_log_file(&message, level),
            None => println!("{}", message),
        };
    }
}

fn global_log(level: LoggerLevel, text: impl AsRef<str>) {
    println!("[GLOBAL] [{}] {}", level, text.as_ref());
}

pub fn metrics(ctx: &mut Context, title: impl AsRef<str>, metrics: Value) {
    let metrics_with_timestamp = json!({
        "title": title.as_ref(),
        "timestamp": *ctx.clock,
        "metrics": metrics
    });

    ctx.logger.write_to_metrics_file(&metrics_with_timestamp);
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
