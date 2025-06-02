use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Experiment {
    pub name: String,
    pub simulation: String,
    pub seed: Option<u64>,
}
