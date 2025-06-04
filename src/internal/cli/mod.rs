pub mod args;
pub mod utils;

use std::{fs, path::Path};

pub use args::Args;
use std::error::Error;

use crate::internal::core::log;

use super::{
    core::config::{Experiment, SimulationConfig},
    simulator::Simulator,
};

/*
 * Retrieves configuration from command line arguments.
 * If no config is created, it returns None meaning
 * that a simulation should not be run.
 */
pub fn get_config_from_args(
    args: Args,
    simulator: &Simulator,
) -> Result<Option<SimulationConfig>, Box<dyn Error>> {
    // Add a new line every execution
    println!();
    if args.list_simulations {
        println!("Available simulations:");
        for sim in &simulator.simulation_registry.list() {
            let (name, description) = sim;
            println!("> {name} - {description}");
        }
        println!();
        return Ok(None);
    } else if args.list_topologies {
        println!("Available topologies:");
        for topology in &simulator.topology_registry.list() {
            println!("> {topology}");
        }
        println!();
        return Ok(None);
    } else if args.list_arrival_times {
        println!("Available arrival time callbacks:");
        for name in &simulator.arrival_time_registry.list() {
            println!("> {name}");
        }
        println!();
        return Ok(None);
    }

    if let Some(config_file) = args.config {
        let path = Path::new(&config_file);

        let mut config: SimulationConfig = toml::from_str(&fs::read_to_string(&config_file)?)?;

        match path.canonicalize() {
            Ok(canonical_path) => {
                if let Some(parent_dir) = canonical_path.parent() {
                    let config_dir = parent_dir.to_string_lossy().to_string();
                    log::global_internal(format!("Reading configuration file from '{config_dir}'"));
                    config.dir = Some(config_dir);
                } else {
                    log::global_error("Config file has no parent directory. Aborting execution.");
                    return Ok(None);
                }
            }
            Err(e) => {
                log::global_error(format!(
                    "Failed to resolve config file path: {e}. Aborting execution."
                ));
                return Ok(None);
            }
        }

        config.should_write_config = args.write_config;

        return Ok(Some(config));
    } else if let Some(simulation_name) = args.simulation {
        let seed: Option<u64> = args.seed.clone().and_then(|s| s.parse().ok());
        // TODO: refactor this to avoid unwrap
        if args.seed.is_some() && seed.is_none() {
            log::global_warn(format!(
                "Failed to parse provided seed: {}",
                args.seed.unwrap()
            ));
        }

        let config = SimulationConfig {
            experiments: vec![Experiment {
                name: args.name.unwrap_or("unnamed_experiment".to_string()),
                simulation: simulation_name,
                seed,
                n_peers: args.n_peers,
                topology: args.topology,
                arrival_time: args.arrival_time,
            }],
            dir: args.dir,
            should_write_config: true,
        };

        return Ok(Some(config));
    }

    Ok(None)
}
