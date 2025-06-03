pub mod internal;
pub mod simulations;

use clap::Parser;
use internal::{
    cli::{Args, get_config_from_args, utils::write_file_with_dirs},
    core::{Context, config::SimulationConfig, log, simulation::SimulationRegistry},
};
use simulations::{DistributedGeneralizedLinearModel, Example, FlowUpdatingPairwise};

fn main() {
    let args = Args::parse();

    let mut registry = SimulationRegistry::default();
    registry
        .register::<DistributedGeneralizedLinearModel>()
        .register::<FlowUpdatingPairwise>()
        .register::<Example>();

    let config: SimulationConfig = match get_config_from_args(args.clone(), &registry) {
        Ok(c_option) => match c_option {
            Some(c) => c,
            None => return,
        },
        Err(e) => {
            log::global_error(format!("Failed to load configuration file: {e}"));
            return;
        }
    };

    for experiment in config.experiments.iter() {
        let mut exp_ctx = Context::new(experiment.seed);

        if let Err(err) = registry.run_simulation(&experiment.simulation, &mut exp_ctx) {
            log::global_warn(format!("Simulation not run: {:?}", err));
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
