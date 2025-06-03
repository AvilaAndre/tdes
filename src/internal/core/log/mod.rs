use clap::ValueEnum;
use paste::paste;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

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
    fn fmt(&self, f: &mut Formatter) -> Result {
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
}

impl Logger {
    pub fn new(level: LoggerLevel) -> Self {
        Self { level }
    }
}

impl Logger {
    fn enabled(&self, level: LoggerLevel) -> bool {
        self.level <= level
    }
}

fn ctx_log(ctx: &Context, level: LoggerLevel, text: impl AsRef<str>) {
    if ctx.logger.enabled(level) {
        println!("[{}] [{}] {}", ctx.clock, level, text.as_ref());
    }
}

fn global_log(level: LoggerLevel, text: impl AsRef<str>) {
    println!("[GLOBAL] [{}] {}", level, text.as_ref());
}

macro_rules! define_log_fn {
    ($name:ident, $level:expr) => {
        paste! {
            pub fn $name(ctx: &Context, text: impl AsRef<str>) {
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
