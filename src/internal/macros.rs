#[macro_export]
macro_rules! get_peer_of_type {
    ($ctx:expr, $peer_id:expr, $peer_type:ty) => {{

        if $peer_id >= $ctx.peers.len() {
            Err(format!("Invalid peer ID: {}", $peer_id))
        } else {
            if let Some(peer) = $ctx.peers[$peer_id].downcast_mut::<$peer_type>() {
                Ok(peer)
            } else {
                Err(format!(
                    "Peer {} is not of required type {}",
                    $peer_id,
                    stringify!($peer_type)
                ))
            }
        }
    }};
}
