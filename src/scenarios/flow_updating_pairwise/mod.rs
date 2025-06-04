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
    Simulator,
    core::{
        Context,
        events::TimerEvent,
        options::{ExperimentOptions, Scenario},
    },
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
            let _ = ctx.add_peer(Box::new(FlowUpdatingPairwisePeer::new(rx, ry, 0.0, rval)));
        }

        simulator
            .topology_registry
            .connect_peers(ctx, opts.topology);
        ctx.message_delay_cb = simulator
            .arrival_time_registry
            .get_callback(opts.arrival_time);
        ctx.on_simulation_finish_hook = Some(Box::new(hooks::on_simulation_finish_hook));

        ctx.add_event(TimerEvent::create(
            ctx.clock,
            Box::new(TickTimer { interval: 0.1 }),
        ));

        ctx.run(opts.deadline);
    }
}
