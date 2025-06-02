pub mod internal;
pub mod simulations;

use clap::Parser;
use internal::{
    cli::{Args, get_config_from_args},
    core::{Context, config::SimulationConfig, simulation::SimulationRegistry},
};
use simulations::{DistributedGeneralizedLinearModel, Example, FlowUpdatingPairwise};

fn main() {
    let args = Args::parse();

    println!("args {:?}", args);

    let mut registry = SimulationRegistry::default();
    registry
        .register::<DistributedGeneralizedLinearModel>()
        .register::<FlowUpdatingPairwise>()
        .register::<Example>();

    let config: SimulationConfig = match get_config_from_args(args, &registry) {
        Ok(c_option) => match c_option {
            Some(c) => c,
            None => return,
        },
        Err(e) => {
            println!("Failed to load configuration file: {e}");
            return;
        }
    };

    for experiment in config.experiments.iter() {
        let mut exp_ctx = Context::new(experiment.seed);

        if let Err(err) = registry.run_simulation(&experiment.simulation, &mut exp_ctx) {
            println!("Simulation not run: {:?}", err);
        }
    }

    let toml_str = toml::to_string(&config).expect("Failed to serialized configuration");
    println!("\nConfiguration file:\n{toml_str}");
}
