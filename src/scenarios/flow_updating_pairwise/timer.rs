use serde_json::json;

use crate::internal::core::{
    Context, engine,
    events::{Timer, TimerEvent},
    log,
    macros::get_peer_of_type,
};

use super::{algorithms, peer::FlowUpdatingPairwisePeer};

#[derive(Debug, Clone)]
pub struct TickTimer {
    pub interval: f64,
}

impl Timer for TickTimer {
    fn fire(&self, ctx: &mut Context) {
        engine::add_event(
            ctx,
            TimerEvent::create(ctx.clock + self.interval, Box::new(self.clone())),
        );

        let mut avgs_metric = json!({});

        // call tick for every peer
        for peer_id in 0..ctx.peers.len() {
            algorithms::tick(ctx, peer_id);

            let peer = get_peer_of_type!(ctx, peer_id, FlowUpdatingPairwisePeer)
                .expect("peer should exist");

            avgs_metric[peer_id.to_string()] = peer.last_avg.into();
        }

        log::metrics(ctx, "on_tick_avgs", &avgs_metric);
    }
}
