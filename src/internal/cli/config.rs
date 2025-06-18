use serde::{Deserialize, Serialize};

use crate::internal::core::{distributions::DistributionWrapper, experiment::Jitter};

use super::{
    super::{
        core::{
            experiment::{Experiment, TopologyInfo},
            log,
        },
        simulator::Simulator,
    },
    Args,
};

use std::{error::Error, fs, path::Path};

#[derive(Debug, Serialize, Deserialize)]
pub struct SimulationConfig {
    pub experiments: Vec<Experiment>,

    #[serde(skip)]
    pub dir: Option<String>,

    // Is true if the config was obtained by parsing an file
    #[serde(skip)]
    pub should_write_config: bool,
}

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
    if args.list_scenarios {
        println!("Available scenarios:");
        for sim in &simulator.scenario_registry.list() {
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

        let mut config: SimulationConfig =
            serde_yaml::from_str(&fs::read_to_string(&config_file)?)?;

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
    } else if let Some(scenario_name) = args.scenario {
        let seed: Option<u64> = args.seed.clone().and_then(|s| s.parse().ok());
        if args.seed.is_some() && seed.is_none() {
            log::global_warn(format!("Failed to parse provided seed: {:?}", args.seed));
        }

        let jitter = if args.use_jitter {
            Some(Jitter {
                distribution: DistributionWrapper::Weibull(1.064, 2.872),
                multiplier: 0.001,
            })
        } else {
            None
        };

        let config = SimulationConfig {
            experiments: vec![Experiment {
                name: args.name.unwrap_or("unnamed_experiment".to_string()),
                scenario: scenario_name,
                seed,
                arrival_time: args.arrival_time,
                topology: TopologyInfo::from_args(args.n_peers, args.topology),
                drop_rate: args.drop_rate,
                duplicate_rate: args.duplicate_rate,
                jitter,
                deadline: args.deadline,
                extra_args: None,
            }],
            dir: args.dir,
            should_write_config: true,
        };

        return Ok(Some(config));
    }

    Ok(None)
}
