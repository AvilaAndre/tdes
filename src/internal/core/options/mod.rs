use super::Context;

pub mod arrival_time_registry;
pub mod topology_registry;

pub use arrival_time_registry::ArrivalTimeRegistry;
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
pub use topology_registry::TopologyRegistry;

#[derive(Debug)]
pub struct ExperimentOptions {
    pub topology: TopologyInfo,
    pub arrival_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TopologyInfo {
    pub n_peers: usize,
    pub name: Option<String>,
    pub list: Option<Vec<(usize, usize, Option<f64>)>>,
}

impl TopologyInfo {
    pub fn from_args(n_peers: Option<usize>, name: Option<String>) -> Self {
        Self {
            // if n_peers isn't specified it will be the default value of 5
            n_peers: n_peers.unwrap_or(5),
            name,
            list: None,
        }
    }
}

pub trait Topology {
    fn name() -> &'static str
    where
        Self: Sized;

    fn connect(
        ctx: &mut Context,
        n_peers: usize,
        custom_list: Option<Vec<(usize, usize, Option<f64>)>>,
    );
}

pub trait ArrivalTimeCallback {
    fn name() -> &'static str
    where
        Self: Sized;

    fn callback(ctx: &mut Context, from: usize, to: usize) -> Option<OrderedFloat<f64>>;
}
