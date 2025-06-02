use serde::{Deserialize, Serialize};
mod experiment;

pub use experiment::Experiment;

#[derive(Serialize, Deserialize)]
pub struct SimulationConfig {
    pub experiments: Vec<Experiment>,
}
