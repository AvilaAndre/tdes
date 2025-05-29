#[macro_export]
macro_rules! get_peer_of_type {
    ($ctx:expr, $peer_id:expr, $peer_type:ty) => {{
        // FIXME: Make sure peer_id is in peers

        if let Some(peer) = $ctx.peers[$peer_id].downcast_mut::<$peer_type>() {
            Ok(peer)
        } else {
            Err(format!(
                "Peer {} is not of required type {}",
                $peer_id,
                stringify!($peer_type)
            ))
        }
    }};
}
