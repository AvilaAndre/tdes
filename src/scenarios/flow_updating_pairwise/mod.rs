mod algorithms;
mod callbacks;
mod hooks;
mod message;
mod peer;
mod timer;

use peer::FlowUpdatingPairwisePeer;
use rand::{Rng, distr::Uniform};
use timer::TickTimer;

use crate::internal::{
    core::{
        engine, hooks::SimulationHooks, options::{ExperimentOptions, Scenario}, Context
    }, Simulator
};

pub struct FlowUpdatingPairwise {}

impl Scenario for FlowUpdatingPairwise {
    fn name() -> &'static str
    where
        Self: Sized,
    {
        "flow_updating_pairwise"
    }

    fn description() -> &'static str
    where
        Self: Sized,
    {
        "An implementation of the flow updating pairwise algorithm."
    }

    fn start(ctx: &mut Context, simulator: &Simulator, opts: ExperimentOptions) {
        let n_peers = opts.topology.n_peers;

        for _ in 0..n_peers {
            let rval = ctx.rng.sample(Uniform::new(0, 80).unwrap());
            let (rx, ry) = (
                ctx.rng.sample(Uniform::new(-100.0, 100.0).unwrap()),
                ctx.rng.sample(Uniform::new(-100.0, 100.0).unwrap()),
            );
            engine::add_peer(ctx, FlowUpdatingPairwisePeer::new(rx, ry, 0.0, rval));
        }

        engine::add_timer(ctx, ctx.clock, TickTimer { interval: 0.1 });

        simulator
            .topology_registry
            .connect_peers(ctx, opts.topology);
        ctx.message_delay_cb = simulator
            .arrival_time_registry
            .get_callback(opts.arrival_time);

        let mut hooks = SimulationHooks::default();
        hooks.set_on_simulation_finish_hook(Box::new(hooks::on_simulation_finish_hook));
        hooks.set_finish_condition(Box::new(hooks::finish_condition_hook));

        engine::run(ctx, &hooks, opts.deadline);
    }
}
