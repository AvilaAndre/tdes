use crate::{
    get_peer_of_type,
    internal::{context::Context, message_passing::send_message_to},
};

use super::{message::FlowUpdatingPairwiseMessage, peer::FlowUpdatingPairwisePeer};

const TICKS: u32 = 50;

pub fn avg_and_send(ctx: &mut Context, peer_id: usize, neigh_id: usize) {
    if let Some(neighbors) = ctx.get_neighbors(peer_id) {
        let peer: &mut FlowUpdatingPairwisePeer =
            get_peer_of_type!(ctx, peer_id, FlowUpdatingPairwisePeer).expect("peer should exist");

        let flows_sum: f64 = neighbors
            .into_iter()
            .map(|idx| *peer.flows.get(&idx).unwrap_or(&0.0))
            .sum();
        let estimate = (peer.value as f64) - flows_sum;
        let avg = (peer.estimates.get(&neigh_id).copied().unwrap_or(0.0) + estimate) / 2.0;

        peer.last_avg = avg;
        peer.flows.insert(
            neigh_id,
            peer.flows.get(&neigh_id).copied().unwrap_or(0.0) + avg
                - peer.estimates.get(&neigh_id).copied().unwrap_or(0.0),
        );
        peer.estimates.insert(neigh_id, avg);

        peer.ticks_since_last_avg.insert(neigh_id, 0);

        let payload = Box::new(FlowUpdatingPairwiseMessage {
            sender: peer_id,
            flow: peer.flows.get(&neigh_id).copied().unwrap_or(0.0),
            estimate: avg,
        });

        send_message_to(ctx, peer_id, neigh_id, Some(payload));
    }
}

pub fn tick(ctx: &mut Context, peer_id: usize) {
    if let Some(neighbors) = ctx.get_neighbors(peer_id) {
        for neigh_id in neighbors {
            let peer: &mut FlowUpdatingPairwisePeer =
                get_peer_of_type!(ctx, peer_id, FlowUpdatingPairwisePeer)
                    .expect("peer should exist");

            let neigh_ticks = peer
                .ticks_since_last_avg
                .get(&neigh_id)
                .copied()
                .unwrap_or(0);

            if neigh_ticks > TICKS {
                avg_and_send(ctx, peer_id, neigh_id);
            } else {
                peer.ticks_since_last_avg.insert(neigh_id, neigh_ticks + 1);
            }
        }
    }
}
