use serde::{Deserialize, Serialize};

use crate::internal::core::experiment::Experiment;

#[derive(Debug, Serialize, Deserialize)]
pub struct SimulationConfig {
    pub experiments: Vec<Experiment>,

    #[serde(skip)]
    pub dir: Option<String>,

    // Is true if the config was obtained by parsing an file
    #[serde(skip)]
    pub should_write_config: bool,
}
