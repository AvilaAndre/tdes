pub mod internal;
pub mod simulations;

use internal::Simulator;
use simulations::{DistributedGeneralizedLinearModel, Example, FlowUpdatingPairwise};

fn main() {
    Simulator::default()
        .add_simulation::<DistributedGeneralizedLinearModel>()
        .add_simulation::<FlowUpdatingPairwise>()
        .add_simulation::<Example>()
        .start();
}
