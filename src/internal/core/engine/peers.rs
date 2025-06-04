use indexmap::IndexMap;

use crate::internal::core::{Context, peer::CustomPeer};

pub fn add_peer(ctx: &mut Context, mut custom_peer: Box<dyn CustomPeer>) -> usize {
    let new_id = ctx.peers.len();
    custom_peer.as_mut().get_peer_mut().id = Some(new_id);

    ctx.peers.push(custom_peer);
    ctx.links.push(IndexMap::new());
    new_id
}

pub fn get_neighbors(ctx: &mut Context, peer_id: usize) -> Option<Vec<usize>> {
    Some(
        ctx.links
            .get(peer_id)?
            .keys()
            .copied()
            .collect::<Vec<usize>>(),
    )
}
