use serde::Serialize;
mod experiment;

pub use experiment::Experiment;

#[derive(Serialize)]
pub struct SimulationConfig {
    pub experiments: Vec<Experiment>,
}
