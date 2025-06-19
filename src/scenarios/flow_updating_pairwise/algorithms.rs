use crate::internal::core::{Context, engine, log, macros::get_peer_of_type, peer::CustomPeer};

use super::{message::FlowUpdatingPairwiseMessage, peer::FlowUpdatingPairwisePeer};

pub fn avg_and_send(ctx: &mut Context, peer_id: usize, neigh_id: usize) {
    log::trace(
        ctx,
        format!("Peer_{peer_id} avg_and_send to Peer_{neigh_id}"),
    );

    if let Some(neighbors) = engine::get_neighbors_alive(ctx, peer_id) {
        let peer: &mut FlowUpdatingPairwisePeer =
            get_peer_of_type!(ctx, peer_id, FlowUpdatingPairwisePeer).expect("peer should exist");

        let flows_sum: f64 = neighbors
            .into_iter()
            .map(|idx| *peer.flows.get(&idx).unwrap_or(&0.0))
            .sum();
        let estimate = f64::from(peer.value) - flows_sum;
        let avg = (peer.estimates.get(&neigh_id).copied().unwrap_or(0.0) + estimate) / 2.0;

        peer.last_avg = avg;
        peer.flows.insert(
            neigh_id,
            peer.flows.get(&neigh_id).copied().unwrap_or(0.0) + avg
                - peer.estimates.get(&neigh_id).copied().unwrap_or(0.0),
        );
        peer.estimates.insert(neigh_id, avg);

        peer.ticks_since_last_avg.insert(neigh_id, 0);

        let payload = FlowUpdatingPairwiseMessage {
            sender: peer_id,
            flow: peer.flows.get(&neigh_id).copied().unwrap_or(0.0),
            estimate: avg,
        };

        log::trace(
            ctx,
            format!("Peer_{peer_id} flows_sum {flows_sum}"),
        );
        log::trace(
            ctx,
            format!("Peer_{peer_id} estimate {estimate}"),
        );
        log::trace(
            ctx,
            format!("Peer_{peer_id} new last_avg {avg}"),
        );

        engine::send_message_to(ctx, peer_id, neigh_id, payload);
    }
}

const TICKS: u32 = 50;

pub fn tick(ctx: &mut Context, peer_id: usize) {
    let peer: &mut FlowUpdatingPairwisePeer =
        get_peer_of_type!(ctx, peer_id, FlowUpdatingPairwisePeer).expect("peer should exist");

    if !peer.is_alive() {
        return;
    }

    if let Some(neighbors) = engine::get_neighbors(ctx, peer_id) {
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
