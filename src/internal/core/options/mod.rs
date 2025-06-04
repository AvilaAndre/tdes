use super::Context;

pub mod arrival_time_registry;
pub mod topology_registry;

use ordered_float::OrderedFloat;
pub use topology_registry::TopologyRegistry;
pub use arrival_time_registry::ArrivalTimeRegistry;

#[derive(Debug)]
pub struct ExperimentOptions {
    pub n_peers: usize,
    pub topology: Option<String>,
    pub arrival_time: Option<String>,
}

pub trait Topology {
    fn name() -> &'static str
    where
        Self: Sized;

    fn connect(ctx: &mut Context, n_peers: usize);
}

pub trait ArrivalTimeCallback {
    fn name() -> &'static str
    where
        Self: Sized;

    fn callback(ctx: &mut Context, from: usize, to: usize) -> OrderedFloat<f64>;
}
