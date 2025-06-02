mod registry;

use super::Context;
pub use registry::SimulationRegistry;

pub trait Simulation {
    fn name() -> &'static str
    where
        Self: Sized;

    fn description() -> &'static str
    where
        Self: Sized;

    fn start(ctx: &mut Context);
}
