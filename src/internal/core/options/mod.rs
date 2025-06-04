use super::Context;

pub mod arrival_time_registry;
pub mod scenario_registry;
pub mod topology_registry;
mod traits;

pub use arrival_time_registry::ArrivalTimeRegistry;
pub use scenario_registry::ScenarioRegistry;
pub use topology_registry::TopologyRegistry;
pub use traits::{ArrivalTimeCallback, Scenario, Topology};

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct ExperimentOptions {
    pub topology: TopologyInfo,
    pub arrival_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TopologyInfo {
    pub n_peers: usize,
    pub name: Option<String>,
    #[serde(default)]
    pub connections: Option<Vec<(usize, usize, Option<f64>)>>,
    #[serde(default)]
    pub positions: Vec<(f64, f64, Option<f64>)>,
}

impl TopologyInfo {
    pub fn from_args(n_peers: Option<usize>, name: Option<String>) -> Self {
        Self {
            // if n_peers isn't specified it will be the default value of 5
            n_peers: n_peers.unwrap_or(5),
            name,
            connections: None,
            positions: Vec::new(),
        }
    }
}
