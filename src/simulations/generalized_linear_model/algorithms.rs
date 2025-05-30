use crate::{
    get_peer_of_type,
    internal::{context::Context, message_passing::send_message_to},
};

use super::{message::GlmSumRowsMessage, peer::GlmPeer};

pub fn peer_start(ctx: &mut Context, peer_id: usize) {
    broadcast_sum_rows(ctx, peer_id)
}

fn broadcast_sum_rows(ctx: &mut Context, peer_id: usize) {
    if let Some(neighbors) = ctx.get_neighbors(peer_id) {
        let mut nodes_filtered: Vec<usize> = Vec::new();

        for neigh_id in neighbors {
            if ctx.peers.get(neigh_id).is_some_and(|p| p.is::<GlmPeer>()) {
                send_sum_rows(ctx, peer_id, neigh_id);
                nodes_filtered.push(neigh_id);
            }
        }
    }
}

fn send_sum_rows(ctx: &mut Context, peer_id: usize, target_id: usize) {
    let peer: &mut GlmPeer = get_peer_of_type!(ctx, peer_id, GlmPeer).expect("peer should exist");

    let msg = GlmSumRowsMessage {
        origin: peer_id,
        nrows: peer.state.total_nrow,
    };

    send_message_to(ctx, peer_id, target_id, Some(Box::new(msg)));
}
