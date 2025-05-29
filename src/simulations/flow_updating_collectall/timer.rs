use crate::internal::{
    context::Context,
    events::types::timer::{Timer, TimerEvent},
};

use super::algorithms;

#[derive(Debug, Clone)]
pub struct TickTimer {
    pub interval: f64,
}

impl Timer for TickTimer {
    fn fire(&self, ctx: &mut Context) {
        ctx.add_event(TimerEvent::create(
            ctx.clock + self.interval,
            Box::new(self.clone()),
        ));

        // call tick for every peer
        for peer_id in 0..ctx.peers.len() {
            algorithms::tick(ctx, peer_id);
        }
    }
}
