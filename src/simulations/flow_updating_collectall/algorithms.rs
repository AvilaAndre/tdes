use crate::{
    get_peer_of_type,
    internal::{context::Context, message_passing::send_message_to},
};

use super::peer::{FlowUpdatingPairwiseMessage, FlowUpdatingPairwisePeer};

pub fn avg_and_send(ctx: &mut Context, peer_id: usize, neigh_id: usize) {
    let peer: &mut FlowUpdatingPairwisePeer =
        get_peer_of_type!(ctx, peer_id, FlowUpdatingPairwisePeer).expect("peer should exist");

    // TODO: Delete this debug line
    println!(
        "[{}] peer with id {} has value {}, last_avg is {}",
        ctx.clock, peer_id, peer.value, peer.last_avg
    );

    // FIXME: only values from neighbors
    let flows_sum: f64 = peer.flows.values().sum();
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

// TODO: Implement tick() function with neighbors
pub fn tick(ctx: &mut Context, peer_id: usize) {
    //for neigh in self.neighbors.keys():
    //    if self.ticks_since_last_avg[neigh] < threshold:
    //        self.avg_and_send(neigh)

    let size = ctx.peers.len();

    for neigh_id in 0..size {
        if neigh_id == peer_id {
            continue;
        }

        let peer: &mut FlowUpdatingPairwisePeer =
            get_peer_of_type!(ctx, peer_id, FlowUpdatingPairwisePeer).expect("peer should exist");

        let neigh_ticks = peer
            .ticks_since_last_avg
            .get(&neigh_id)
            .copied()
            .unwrap_or(0);

        // TODO: make number of ticks a const
        if neigh_ticks > 50 {
            avg_and_send(ctx, peer_id, neigh_id);
        } else {
            peer.ticks_since_last_avg.insert(neigh_id, neigh_ticks + 1);
        }
    }
}
