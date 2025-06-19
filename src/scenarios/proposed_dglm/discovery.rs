use super::{message::DiscoveryMessage, peer::PGlmPeer};
use crate::{
    internal::core::{Context, engine, log, macros::get_peer_of_type},
    scenarios::proposed_dglm::improve::check_missing_sum_rows,
};

pub fn broadcast_ids(ctx: &mut Context, peer_id: usize) {
    log::trace(ctx, format!("peer {peer_id} broadcasting ids"));

    let peer: &mut PGlmPeer = get_peer_of_type!(ctx, peer_id, PGlmPeer).expect("peer should exist");

    for neigh_id in peer.state.neighbors.clone() {
        send_discovery_msg(ctx, peer_id, neigh_id);
    }
}

pub fn send_discovery_msg(ctx: &mut Context, sender_id: usize, receiver_id: usize) {
    let sender_peer: &mut PGlmPeer =
        get_peer_of_type!(ctx, sender_id, PGlmPeer).expect("peer should exist");

    let ids: Vec<usize> = sender_peer.state.nodes.iter().copied().collect();

    let msg = DiscoveryMessage {
        origin: sender_id,
        nodes: ids.clone(),
    };

    log::trace(
        ctx,
        format!("peer {sender_id} Sent DiscoveryMessage to peer {receiver_id} with ids {ids:?}"),
    );
    engine::send_message_to(ctx, sender_id, receiver_id, msg);
}

pub fn receive_discovery_msg(ctx: &mut Context, peer_id: usize, msg: DiscoveryMessage) {
    log::trace(
        ctx,
        format!(
            "peer {peer_id} received a DiscoveryMessage from {sender} with ids {ids:?}",
            sender = msg.origin,
            ids = msg.nodes
        ),
    );

    let peer: &mut PGlmPeer = get_peer_of_type!(ctx, peer_id, PGlmPeer).expect("peer should exist");

    let old_peers_size = peer.state.nodes.len();
    peer.state.nodes.extend(msg.nodes);

    // Check if discovery updated
    if old_peers_size != peer.state.nodes.len() {
        peer.discovery_reset();
        log::trace(ctx, format!("peer {peer_id} called discovery_reset"));

        // before broadcasting, check if a neighbor was added
        let peer_ids_filtered: Vec<usize> = engine::get_neighbors_alive(ctx, peer_id)
            .map(|neighbors| {
                neighbors
                    .into_iter()
                    .filter(|&neigh_id| ctx.peers.get(neigh_id).is_some_and(|p| p.is::<PGlmPeer>()))
                    .collect()
            })
            .unwrap_or_default();

        let peer: &mut PGlmPeer =
            get_peer_of_type!(ctx, peer_id, PGlmPeer).expect("peer should exist");

        peer.state.neighbors = peer_ids_filtered.clone();

        broadcast_ids(ctx, peer_id);

        check_missing_sum_rows(ctx, peer_id);
    }
}
