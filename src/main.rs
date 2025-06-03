pub mod internal;
pub mod simulations;

use clap::Parser;
use internal::{
    cli::{Args, get_config_from_args, utils::write_file_with_dirs},
    core::{
        Context,
        config::SimulationConfig,
        log,
        options::{ArrivalTimeRegistry, ExperimentOptions, TopologyRegistry},
        simulation::SimulationRegistry,
    },
};
use simulations::{DistributedGeneralizedLinearModel, Example, FlowUpdatingPairwise};

fn main() {
    let args = Args::parse();

    let mut simulation_registry = SimulationRegistry::default();
    simulation_registry
        .register::<DistributedGeneralizedLinearModel>()
        .register::<FlowUpdatingPairwise>()
        .register::<Example>();

    let topology_registry = TopologyRegistry::default();
    let arrival_time_registry = ArrivalTimeRegistry::default();

    let mut config: SimulationConfig = match get_config_from_args(
        args.clone(),
        &simulation_registry,
        &topology_registry,
        &arrival_time_registry,
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
        let mut exp_ctx = Context::new(experiment.seed, experiment.logger_level);

        if let Some(filepath) = &experiment.log_file {
            if let Err(e) = exp_ctx.logger.set_file(filepath) {
                log::global_error(format!("Failed to set log file to {filepath}: {e}"));
                // Do not store log_file path as it was not used
                experiment.log_file = None;
            }
        }

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
            &topology_registry,
            &arrival_time_registry,
            opts,
        ) {
            log::global_error(format!("Simulation not run: {:?}", err));
        }
    }

    let toml_str = toml::to_string(&config).expect("Failed to serialized configuration");
    if let Some(outfile) = args.out {
        match write_file_with_dirs(&outfile, &toml_str) {
            Ok(_) => {
                log::global_info(format!("Wrote configuration file to: {}", outfile));
            }
            Err(e) => {
                log::global_warn(format!("Failed to write configuration file: {}", e));
            }
        }
    } else {
        println!("\nConfiguration file:\n{toml_str}");
    }
}
