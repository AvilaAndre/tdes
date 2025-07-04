use indexmap::IndexMap;

use crate::internal::core::{Context, peer::CustomPeer};

pub fn add_peer(ctx: &mut Context, mut custom_peer: impl CustomPeer + 'static) -> usize {
    let new_id = custom_peer.instantiate(ctx.peers.len());

    ctx.peers.push(Box::new(custom_peer));
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

pub fn get_neighbors_alive(ctx: &mut Context, peer_id: usize) -> Option<Vec<usize>> {
    Some(
        ctx.links
            .get(peer_id)?
            .keys()
            .copied()
            .filter(|id| ctx.peers.get(*id).is_some_and(|p| p.is_alive()))
            .collect::<Vec<usize>>(),
    )
}
