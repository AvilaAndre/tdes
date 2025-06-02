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
    context::Context, events::types::timer::TimerEvent,
    message_passing::distance_based_arrival_time, simulation::Simulation,
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

    fn start(ctx: &mut Context) {
        ctx.message_delay_cb = distance_based_arrival_time;
        ctx.on_simulation_finish_hook = Some(Box::new(hooks::on_simulation_finish_hook));

        let n_peers = 50;

        for _ in 0..n_peers {
            let rval = ctx.rng.sample(Uniform::new(0, 80).unwrap());
            let (rx, ry) = (
                ctx.rng.sample(Uniform::new(-100.0, 100.0).unwrap()),
                ctx.rng.sample(Uniform::new(-100.0, 100.0).unwrap()),
            );
            let _ = ctx.add_peer(Box::new(FlowUpdatingPairwisePeer::new(rx, ry, 0.0, rval)));
        }

        // full connection
        for i in 0..n_peers {
            for j in i + 1..n_peers {
                ctx.add_twoway_link(i, j, None);
            }
        }

        ctx.add_event(TimerEvent::create(
            ctx.clock,
            Box::new(TickTimer { interval: 0.1 }),
        ));

        ctx.run_for(OrderedFloat(17.1));
    }
}
