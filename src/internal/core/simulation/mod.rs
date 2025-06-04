mod registry;

use crate::internal::Simulator;

use super::{Context, options::ExperimentOptions};

pub use registry::SimulationRegistry;

pub trait Simulation {
    fn name() -> &'static str
    where
        Self: Sized;

    fn description() -> &'static str
    where
        Self: Sized;

    fn start(ctx: &mut Context, simulator: &Simulator, opts: ExperimentOptions);
}
