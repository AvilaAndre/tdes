use serde::Serialize;

#[derive(Serialize)]
pub struct Experiment {
    pub name: String,
    pub simulation: String,
    pub seed: Option<u64>,
}
