pub mod cli;
pub mod core;

use core::{
    Context,
    config::SimulationConfig,
    log,
    options::{ArrivalTimeRegistry, ExperimentOptions, TopologyRegistry},
    simulation::SimulationRegistry,
};

use chrono::Local;
use clap::Parser;
use cli::{Args, get_config_from_args, utils::write_file_with_dirs};

pub fn start(
    _args: Args,
    simulation_registry: &SimulationRegistry,
    topology_registry: &TopologyRegistry,
    arrival_time_registry: &ArrivalTimeRegistry,
) {
    let args = Args::parse();

    let mut config: SimulationConfig = match get_config_from_args(
        args.clone(),
        simulation_registry,
        topology_registry,
        arrival_time_registry,
    ) {
        Ok(c_option) => match c_option {
            Some(c) => c,
            None => return,
        },
        Err(e) => {
            log::global_error(format!("Failed to load configuration file: {e}"));
            return;
        }
    };

    for experiment in config.experiments.iter_mut() {
        // Print new line before each experiment
        println!();
        log::global_internal(format!("EXPERIMENT '{}'", experiment.name));

        let mut exp_ctx = Context::new(experiment.seed, args.logger_level);

        if let Some(directory) = &config.dir {
            let log_file_path = format!(
                "{}/{}/{}/{}.log",
                directory,
                experiment.name,
                Local::now().timestamp(),
                experiment.name,
            );
            let metrics_file_path = format!(
                "{}/{}/{}/{}.jsonl",
                directory,
                experiment.name,
                Local::now().timestamp(),
                experiment.name,
            );
            if let Err(e) = exp_ctx.logger.set_log_file(&log_file_path) {
                log::global_error(format!("Failed to set log file to {log_file_path}: {e}"));
            }
            if let Err(e) = exp_ctx.logger.set_metrics_file(&metrics_file_path) {
                log::global_error(format!(
                    "Failed to set metrics file to {metrics_file_path}: {e}"
                ));
            }
        }

        exp_ctx.logger.set_flush_threshold(args.flush_threshold);

        // add generated seed to config
        experiment.seed = Some(exp_ctx.seed);
        // if n_peers isn't specified it will be the default value of 5
        experiment.n_peers = Some(experiment.n_peers.unwrap_or(5));

        let opts = ExperimentOptions {
            n_peers: experiment.n_peers.unwrap_or(5),
            topology: experiment.topology.clone(),
            arrival_time: experiment.arrival_time.clone(),
        };

        if let Err(err) = simulation_registry.run_simulation(
            &experiment.simulation,
            &mut exp_ctx,
            topology_registry,
            arrival_time_registry,
            opts,
        ) {
            log::global_error(format!("Simulation not run: {:?}", err));
        }
    }

    let toml_str = toml::to_string(&config).expect("Failed to serialized configuration");

    if config.should_write_config {
        if let Some(dir) = config.dir {
            let outfile = format!("{}/config.toml", dir);
            match write_file_with_dirs(&outfile, &toml_str) {
                Ok(_) => {
                    log::global_info(format!("Wrote configuration file to: {}", outfile));
                }
                Err(e) => {
                    log::global_warn(format!(
                        "Failed to write configuration file: {}\nto: {}",
                        e, outfile
                    ));
                }
            }
        } else {
            println!("\nConfiguration file:\n{toml_str}");
        }
    } else {
        println!("\nThe parsed configuration file is:\n{toml_str}");
    }
}
