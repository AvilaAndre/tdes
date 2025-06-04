use serde::{Deserialize, Serialize};

use crate::internal::core::options::TopologyInfo;

#[derive(Debug, Serialize, Deserialize)]
pub struct Experiment {
    pub name: String,
    pub scenario: String,
    pub seed: Option<u64>,
    pub arrival_time: Option<String>,
    pub topology: TopologyInfo,
    pub deadline: Option<f64>,
}
