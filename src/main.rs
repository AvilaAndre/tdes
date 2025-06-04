pub mod internal;
pub mod simulations;

use clap::Parser;
use internal::{
    cli::Args,
    core::{
        options::{ArrivalTimeRegistry, TopologyRegistry},
        simulation::SimulationRegistry,
    },
};
use simulations::{DistributedGeneralizedLinearModel, Example, FlowUpdatingPairwise};

fn main() {
    let args = Args::parse();

    let mut simulation_registry = SimulationRegistry::default();
    let topology_registry = TopologyRegistry::default();
    let arrival_time_registry = ArrivalTimeRegistry::default();

    simulation_registry
        .register::<DistributedGeneralizedLinearModel>()
        .register::<FlowUpdatingPairwise>()
        .register::<Example>();

    internal::start(
        args,
        &simulation_registry,
        &topology_registry,
        &arrival_time_registry,
    );
}
