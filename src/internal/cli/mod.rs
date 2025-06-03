pub mod utils;

use std::fs;

use clap::{ArgGroup, Parser};
use std::error::Error;

use crate::internal::core::log;

use super::core::{
    config::{Experiment, SimulationConfig},
    simulation::SimulationRegistry,
};

#[derive(Parser, Debug, Clone)]
#[command(
    version,
    about = "Discrete Event Simulator",
    long_about = None,
    group(
        ArgGroup::new("run_mode")
            .required(true)
            .args(["config", "simulation", "list_simulations"])
    )
)]
pub struct Args {
    /// Location of the configuration file
    #[arg(short, long)]
    pub config: Option<String>,

    /// Which simulation should be run
    #[arg(short, long)]
    pub simulation: Option<String>,

    /// The simulation seed - can only be used if 'simulation' is set
    #[arg(long, requires = "simulation")]
    pub seed: Option<String>,

    /// Which simulations can be run
    #[arg(long)]
    pub list_simulations: bool,

    /// Where to output the configuration file used (prints to console if not specified)
    #[arg(short, long)]
    pub out: Option<String>,
}

/*
 * Retrieves configuration from command line arguments.
 * If no config is created, it returns None meaning that a simulation
 * should not be run.
 */
pub fn get_config_from_args(
    args: Args,
    registry: &SimulationRegistry,
) -> Result<Option<SimulationConfig>, Box<dyn Error>> {
    // Add a new line every execution
    println!();
    if args.list_simulations {
        println!("Available simulations:");
        for sim in registry.list_simulations().iter() {
            println!("> {} - {}", sim.0, sim.1);
        }
        println!();
        return Ok(None);
    }

    if let Some(config_file) = args.config {
        let toml_str = fs::read_to_string(config_file)
            .map_err(|e| format!("Configuration file not found: {e}"))?;

        let config: SimulationConfig = toml::from_str(&toml_str)
            .map_err(|e| format!("Failed to load configuration file: {e}"))?;

        return Ok(Some(config));
    } else if let Some(simulation_name) = args.simulation {
        let seed: Option<u64> = args.seed.clone().and_then(|s| s.parse().ok());
        if args.seed.is_some() && seed.is_none() {
            log::global_warn(format!(
                "Failed to parse provided seed: {}",
                args.seed.unwrap()
            ));
        }

        let config = SimulationConfig {
            experiments: vec![Experiment {
                name: "experiment_0".to_string(),
                simulation: simulation_name,
                seed,
            }],
        };

        return Ok(Some(config));
    }

    Ok(None)
}
