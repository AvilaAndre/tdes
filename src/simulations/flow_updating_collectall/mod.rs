mod algorithms;
mod message;
mod peer;
mod timer;

use ordered_float::OrderedFloat;
use peer::FlowUpdatingPairwisePeer;
use rand::{Rng, distr::Uniform};
use timer::TickTimer;

use crate::internal::{context::Context, events::types::timer::TimerEvent};

pub fn start(ctx: &mut Context) {
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
