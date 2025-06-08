use clap::ValueEnum;
use ordered_float::OrderedFloat;
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
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
    Internal = 5,
}

impl Display for LoggerLevel {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let text = match self {
            LoggerLevel::Trace => "TRACE",
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
    #[must_use]
    pub fn new(level: Option<LoggerLevel>) -> Self {
        Self {
            level: level.unwrap_or(LoggerLevel::Info),
            log_writer: None,
            metrics_writer: None,
            flush_threshold: 200,
            log_unflushed_count: 0,
        }
    }

    pub fn set_flush_threshold(&mut self, new_threshold: usize) {
        self.flush_threshold = new_threshold;
    }

    // TODO: Move this elsewhere
    fn open_file<P: AsRef<Path>>(file_path: P) -> io::Result<File> {
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
        let file = Self::open_file(file_path)?;

        self.log_writer = Some(BufWriter::new(file));
        Ok(())
    }

    pub fn set_metrics_file<P: AsRef<Path>>(&mut self, file_path: P) -> io::Result<()> {
        let file = Self::open_file(file_path)?;

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
            if writeln!(writer, "{message}").is_ok() {
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
            if writeln!(metrics_writer, "{metrics}").is_ok() {
                metrics_writer.flush().ok();
            }
        }
    }
}

fn log_format(clock: OrderedFloat<f64>, level: LoggerLevel, text: impl AsRef<str>) -> String {
    format!("[{}] [{}] {}", clock, level, text.as_ref())
}

fn ctx_log(ctx: &mut Context, level: LoggerLevel, text: impl AsRef<str>) {
    // outputs logs to file independently of the logger_level

    if ctx.logger.log_writer.is_some() {
        ctx.logger
            .write_to_log_file(&log_format(ctx.clock, level, &text), level);
    } else {
        let msg = log_format(ctx.clock, level, &text);
        if ctx.logger.enabled(level) {
            match level {
                LoggerLevel::Warn | LoggerLevel::Error => eprintln!("{}", &msg),
                _ => println!("{}", &msg),
            }
        }
    }
}

fn global_log(level: LoggerLevel, text: impl AsRef<str>) {
    println!("[GLOBAL] [{}] {}", level, text.as_ref());
}

pub fn metrics(ctx: &mut Context, title: impl AsRef<str>, metrics: &Value) {
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

define_log_fn!(trace, LoggerLevel::Trace);
define_log_fn!(debug, LoggerLevel::Debug);
define_log_fn!(info, LoggerLevel::Info);
define_log_fn!(warn, LoggerLevel::Warn);
define_log_fn!(error, LoggerLevel::Error);
define_log_fn!(internal, LoggerLevel::Internal);
