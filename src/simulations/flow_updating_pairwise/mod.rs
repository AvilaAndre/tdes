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

use crate::internal::{
    context::Context, events::types::timer::TimerEvent,
    message_passing::distance_based_arrival_time,
};

pub fn start(ctx: &mut Context) {
    ctx.message_delay_cb = distance_based_arrival_time;
    ctx.on_simulation_finish_hook = Some(hooks::on_simulation_finish_hook);

    for _ in 0..20 {
        let rval = ctx.rng.sample(Uniform::new(0, 80).unwrap());
        let (rx, ry) = (
            ctx.rng.sample(Uniform::new(-100.0, 100.0).unwrap()),
            ctx.rng.sample(Uniform::new(-100.0, 100.0).unwrap()),
        );
        let _ = ctx.add_peer(Box::new(FlowUpdatingPairwisePeer::new(rx, ry, 0.0, rval)));
    }

    ctx.add_event(TimerEvent::create(
        ctx.clock,
        Box::new(TickTimer { interval: 0.1 }),
    ));

    ctx.run_for(OrderedFloat(7.1));
}
