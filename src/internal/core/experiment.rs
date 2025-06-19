use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use super::distributions::DistributionWrapper;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum LinkKind {
    Bandwidth(f64),
    Latency(f64),
    Full(f64, f64)
}

pub type LinkInfo = Option<LinkKind>;

#[derive(Debug, Serialize, Deserialize, Default, Copy, Clone)]
pub struct Jitter {
    pub distribution: DistributionWrapper,
    pub multiplier: f64,
}

// from, to, option(bandwidth or latency)
pub type ConnectionInfo = (usize, usize, LinkInfo);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TopologyInfo {
    pub n_peers: usize,
    pub name: Option<String>,
    #[serde(default)]
    pub connections: Option<Vec<ConnectionInfo>>,
    #[serde(default)]
    pub positions: Option<Vec<(f64, f64, Option<f64>)>>,
}

impl TopologyInfo {
    #[must_use]
    pub fn from_args(n_peers: Option<usize>, name: Option<String>) -> Self {
        Self {
            // if n_peers isn't specified it will be the default value of 5
            n_peers: n_peers.unwrap_or(5),
            name,
            connections: None,
            positions: Some(Vec::new()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Experiment {
    pub name: String,
    pub scenario: String,
    pub seed: Option<u64>,
    pub arrival_time: Option<String>,
    pub topology: TopologyInfo,
    pub drop_rate: Option<f64>,
    pub duplicate_rate: Option<f64>,
    pub jitter: Option<Jitter>,
    pub deadline: Option<f64>,
    pub extra_args: Option<Value>,
    pub repetitions: Option<u64>,
}
