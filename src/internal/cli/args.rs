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
    #[arg(long, requires = "simulation", value_enum, default_value = "info")]
    pub logger_level: Option<LoggerLevel>,

    /// Which simulation should be run
    #[arg(short, long)]
    pub simulation: Option<String>,

    /// Which simulations can be run
    #[arg(long)]
    pub list_simulations: bool,

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

    /// The path for the log file which the experiment's logs will be output to
    #[arg(long, requires = "simulation")]
    pub log_file: Option<String>,

    // TODO: Change so that this is a folder
    /// Where to output the configuration file used (prints to console if not specified)
    #[arg(short, long)]
    pub out: Option<String>,
}
