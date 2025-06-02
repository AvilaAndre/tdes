pub mod internal;
pub mod simulations;

use internal::core::{
    Context,
    config::{Experiment, SimulationConfig},
    simulation::SimulationRegistry,
};
use simulations::{DistributedGeneralizedLinearModel, Example, FlowUpdatingPairwise};

fn main() {
    let mut registry = SimulationRegistry::default();
    registry
        .register::<DistributedGeneralizedLinearModel>()
        .register::<FlowUpdatingPairwise>()
        .register::<Example>();

    let config = SimulationConfig {
        experiments: vec![
            Experiment {
                name: "experiment1".to_string(),
                simulation: "distributed_generalized_linear_model".to_string(),
                seed: Some(559464190120120835),
            },
            Experiment {
                name: "experiment2".to_string(),
                simulation: "distributed_generalized_linear_model".to_string(),
                seed: None,
            },
        ],
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
