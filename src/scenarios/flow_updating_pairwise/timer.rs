use serde_json::json;

use crate::{
    internal::core::{Context, engine, events::Timer, log, macros::get_peer_of_type},
    scenarios::flow_updating_pairwise::message::FlowUpdatingPairwiseMessage,
};

use super::{algorithms, peer::FlowUpdatingPairwisePeer};

#[derive(Debug, Clone)]
pub struct TickTimer {
    pub interval: f64,
}

impl Timer for TickTimer {
    fn fire(&self, ctx: &mut Context) {
        log::trace(ctx, "TickTimer fired");
        engine::add_timer(ctx, ctx.clock + self.interval, self.clone());

        // call tick for every peer
        for peer_id in 0..ctx.peers.len() {
            algorithms::tick(ctx, peer_id);
        }
    }
}

#[derive(Debug, Clone)]
pub struct StartTimer {
    pub peer_id: usize,
}

impl Timer for StartTimer {
    fn fire(&self, ctx: &mut Context) {
        if let Some(neighbors) = engine::get_neighbors_alive(ctx, self.peer_id) {
            for neigh_id in neighbors {
                log::info(
                    ctx,
                    format!("Peer_{} sending to Peer_{neigh_id}", self.peer_id),
                );

                let peer = get_peer_of_type!(ctx, self.peer_id, FlowUpdatingPairwisePeer)
                    .expect("peer should exist");

                let payload = FlowUpdatingPairwiseMessage {
                    sender: self.peer_id,
                    flow: 0.0,
                    estimate: f64::from(peer.value),
                };

                engine::send_message_to(ctx, self.peer_id, neigh_id, payload);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct MetricsTimer {
    pub interval: f64,
}

impl Timer for MetricsTimer {
    fn fire(&self, ctx: &mut Context) {
        engine::add_timer(ctx, ctx.clock + self.interval, self.clone());

        let mut avgs_metric = json!({});

        // call tick for every peer
        for peer_id in 0..ctx.peers.len() {
            if let Ok(peer) = get_peer_of_type!(ctx, peer_id, FlowUpdatingPairwisePeer) {
                avgs_metric[peer_id.to_string()] = peer.last_avg.into();
            }
        }

        log::metrics(ctx, "last_averages", &avgs_metric);
    }
}
