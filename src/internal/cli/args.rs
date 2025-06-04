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
            .args(["config", "simulation", "list_simulations", "list_topologies", "list_arrival_times"])
    )
)]
pub struct Args {
    /// Location of the configuration file to use
    #[arg(short, long)]
    pub config: Option<String>,

    /// Log verbosity level
    #[arg(long, value_enum, default_value = "info")]
    pub logger_level: Option<LoggerLevel>,

    /// Which simulation should be run
    #[arg(short, long)]
    pub simulation: Option<String>,

    /// Which simulations can be run
    #[arg(long)]
    pub list_simulations: bool,

    /// Names a experiment - can only be used if 'simulation' is set
    #[arg(long, requires = "simulation")]
    pub name: Option<String>,

    /// The simulation seed - can only be used if 'simulation' is set
    #[arg(long, requires = "simulation")]
    pub seed: Option<String>,

    /// The amount of peers to instantiate - can only be used if 'simulation' is set
    #[arg(long, requires = "simulation", default_value = "5")]
    pub n_peers: Option<usize>,

    /// The topology to use, must be registered in the simulator - can only be used if 'simulation' is set
    #[arg(long, requires = "simulation")]
    pub topology: Option<String>,

    /// Which topologies can be selected
    #[arg(long)]
    pub list_topologies: bool,

    /// The arrival time callback to use, must be registered in the simulator - can only be used if 'simulation' is set
    #[arg(long, requires = "simulation")]
    pub arrival_time: Option<String>,

    /// Which arrival time callbacks can be selected
    #[arg(long)]
    pub list_arrival_times: bool,

    /// The amount of logs needed to flush to file
    #[arg(long, default_value = "200")]
    pub flush_threshold: usize,

    /// Where the configuration and logs should be stored (prints to console if not specified)
    #[arg(short, long, requires = "simulation")]
    pub dir: Option<String>,

    /// Overwrites the provided a configuration file
    #[arg(short, long, requires = "config")]
    pub write_config: bool,
}
