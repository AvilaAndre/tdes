use clap::{ArgGroup, Parser};

use crate::internal::core::log::LoggerLevel;

#[derive(Parser, Debug, Clone)]
#[command(
    version,
    about = "Discrete Event Simulator",
    long_about = None,
    group(
        ArgGroup::new("run_mode")
            .required(true)
            .args(["config", "scenario", "list_scenarios", "list_topologies", "list_arrival_times"])
    )
)]
pub struct Args {
    /// Location of the configuration file to use
    #[arg(short, long)]
    pub config: Option<String>,

    /// Log verbosity level
    #[arg(long, value_enum, default_value = "info")]
    pub logger_level: Option<LoggerLevel>,

    /// Which scenario should be run
    #[arg(short, long)]
    pub scenario: Option<String>,

    /// Which scenarios can be run
    #[arg(long)]
    pub list_scenarios: bool,

    /// Names a experiment - can only be used if 'scenario' is set
    #[arg(long, requires = "scenario")]
    pub name: Option<String>,

    /// The scenario seed - can only be used if 'scenario' is set
    #[arg(long, requires = "scenario")]
    pub seed: Option<String>,

    /// The amount of peers to instantiate - can only be used if 'scenario' is set
    #[arg(long, requires = "scenario", default_value = "5")]
    pub n_peers: Option<usize>,

    /// Sets the message drop rate (A float in [0.0, 1.0]) - can only be used if 'scenario' is set
    #[arg(long, requires = "scenario")]
    pub drop_rate: Option<f64>,

    /// The topology to use, must be registered in the simulator - can only be used if 'scenario' is set
    #[arg(long, requires = "scenario")]
    pub topology: Option<String>,

    /// Which topologies can be selected
    #[arg(long)]
    pub list_topologies: bool,

    /// The arrival time callback to use, must be registered in the simulator - can only be used if 'scenario' is set
    #[arg(long, requires = "scenario")]
    pub arrival_time: Option<String>,

    /// Which arrival time callbacks can be selected
    #[arg(long)]
    pub list_arrival_times: bool,

    /// Optional value which tells the simulator when to stop (it may stop earlier if there are no events to process)
    #[arg(long)]
    pub deadline: Option<f64>,

    /// The amount of logs needed to flush to file
    #[arg(long, default_value = "200")]
    pub flush_threshold: usize,

    /// Where the configuration and logs should be stored (prints to console if not specified)
    #[arg(short, long, requires = "scenario")]
    pub dir: Option<String>,

    /// Overwrites the provided a configuration file
    #[arg(short, long, requires = "config")]
    pub write_config: bool,
}
