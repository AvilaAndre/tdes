use serde_yaml::Value;

use super::super::experiment::TopologyInfo;

#[derive(Debug)]
pub struct ExperimentOptions {
    pub topology: TopologyInfo,
    pub arrival_time: Option<String>,
    pub deadline: Option<f64>,
    pub extra_args: Option<Value>,
}
