mod algorithms;
mod callbacks;
mod hooks;
mod message;
mod peer;
mod timer;

use ordered_float::OrderedFloat;
use peer::FlowUpdatingPairwisePeer;
use rand::{Rng, distr::Uniform};
use timer::TickTimer;

use crate::internal::core::{
    Context,
    events::TimerEvent,
    options::{ArrivalTimeRegistry, ExperimentOptions, TopologyRegistry},
    simulation::Simulation,
};

pub struct FlowUpdatingPairwise {}

impl Simulation for FlowUpdatingPairwise {
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

    fn start(
        ctx: &mut Context,
        topology_registry: &TopologyRegistry,
        arrival_time_registry: &ArrivalTimeRegistry,
        opts: ExperimentOptions,
    ) {
        let n_peers = opts.n_peers;

        for _ in 0..n_peers {
            let rval = ctx.rng.sample(Uniform::new(0, 80).unwrap());
            let (rx, ry) = (
                ctx.rng.sample(Uniform::new(-100.0, 100.0).unwrap()),
                ctx.rng.sample(Uniform::new(-100.0, 100.0).unwrap()),
            );
            let _ = ctx.add_peer(Box::new(FlowUpdatingPairwisePeer::new(rx, ry, 0.0, rval)));
        }

        topology_registry.connect_peers(opts.topology, ctx, n_peers);
        ctx.message_delay_cb = arrival_time_registry.get_callback(opts.arrival_time);
        ctx.on_simulation_finish_hook = Some(Box::new(hooks::on_simulation_finish_hook));

        ctx.add_event(TimerEvent::create(
            ctx.clock,
            Box::new(TickTimer { interval: 0.1 }),
        ));

        // TODO: This should be an argument
        ctx.run_for(OrderedFloat(17.1));
    }
}
