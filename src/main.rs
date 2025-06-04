pub mod internal;
mod scenarios;

use internal::Simulator;
use scenarios::{DistributedGeneralizedLinearModel, Example, FlowUpdatingPairwise};

fn main() {
    Simulator::default()
        .add_scenario::<Example>()
        .add_scenario::<FlowUpdatingPairwise>()
        .add_scenario::<DistributedGeneralizedLinearModel>()
        .start();
}
