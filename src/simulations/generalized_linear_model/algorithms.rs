use crate::{get_peer_of_type, internal::context::Context};

use super::peer::GlmPeer;

pub fn start(ctx: &mut Context, peer_id: usize) {
    let peer: &mut GlmPeer = get_peer_of_type!(ctx, peer_id, GlmPeer).expect("peer should exist");

    broadcast_sum_rows(ctx, peer_id)
}

fn broadcast_sum_rows(ctx: &mut Context, peer_id: usize) {
    // TODO: I am here
    if let Some(neighbors) = ctx.get_neighbors(peer_id) {
        let mut nodes_filtered: Vec<usize> = Vec::new();

        for neigh_id in neighbors {
            // FIXME: Get all neighbors that are of same type
            let peer: &mut GlmPeer =
                get_peer_of_type!(ctx, peer_id, GlmPeer).expect("peer should exist");

            nodes_filtered.push(peer.peer.id.unwrap());

            // self.send_sum_rows(actor_name)
            // nodes_filtered.append(actor_name)

            // self.state.nodes = nodes_filtered
        }
    }
}

fn send_sum_rows() {

}
