pub mod internal;
mod scenarios;

use internal::Simulator;
use scenarios::{
    DistributedGeneralizedLinearModel, Example, FlowUpdatingPairwise, SimpleMessageScenario,
};

use crate::scenarios::ProposedDglm;

fn main() {
    Simulator::default()
        .add_scenario::<Example>()
        .add_scenario::<FlowUpdatingPairwise>()
        .add_scenario::<DistributedGeneralizedLinearModel>()
        .add_scenario::<SimpleMessageScenario>()
        .add_scenario::<ProposedDglm>()
        .start();
}
