pub mod internal;
pub mod simulations;

use internal::Simulator;
use simulations::{DistributedGeneralizedLinearModel, Example, FlowUpdatingPairwise};

fn main() {
    Simulator::default()
        .add_simulation::<Example>()
        .add_simulation::<FlowUpdatingPairwise>()
        .add_simulation::<DistributedGeneralizedLinearModel>()
        .start();
}
