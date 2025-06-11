use super::Context;
use super::experiment::TopologyInfo;

pub mod arrival_time_registry;
pub mod scenario_registry;
pub mod topology_registry;
mod traits;

pub use arrival_time_registry::ArrivalTimeRegistry;
pub use scenario_registry::ScenarioRegistry;
use serde_yaml::Value;
pub use topology_registry::TopologyRegistry;
pub use traits::{ArrivalTimeCallback, Scenario, Topology};

#[derive(Debug)]
pub struct ExperimentOptions {
    pub topology: TopologyInfo,
    pub arrival_time: Option<String>,
    pub deadline: Option<f64>,
    pub extra_args: Option<Value>
}
