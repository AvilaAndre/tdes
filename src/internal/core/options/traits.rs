use ordered_float::OrderedFloat;

use crate::internal::{
    Simulator,
    core::{Context, config::ConnectionInfo, options::ExperimentOptions},
};

pub trait Scenario {
    fn name() -> &'static str
    where
        Self: Sized;

    fn description() -> &'static str
    where
        Self: Sized;

    fn start(ctx: &mut Context, simulator: &Simulator, opts: ExperimentOptions);
}

pub trait Topology {
    fn name() -> &'static str
    where
        Self: Sized;

    fn connect(ctx: &mut Context, n_peers: usize, custom_list: Option<Vec<ConnectionInfo>>);
}

pub trait ArrivalTimeCallback {
    fn name() -> &'static str
    where
        Self: Sized;

    fn callback(ctx: &mut Context, from: usize, to: usize) -> Option<OrderedFloat<f64>>;
}
