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

pub(crate) use get_peer_of_type;

macro_rules! define_custom_peer {
    ($structname: ident) => {
        impl CustomPeer for $structname {
            fn get_peer(&self) -> &PeerInfo {
                &self.peer_info
            }

            fn get_peer_mut(&mut self) -> &mut PeerInfo {
                &mut self.peer_info
            }
        }
    };
}

pub(crate) use define_custom_peer;

macro_rules! define_custom_arrival_time_callback {
    ($name:ident, $topology_name:expr, |$ctx:ident, $from:ident, $to:ident| $connect_fn:block) => {
        pub struct $name;

        impl ArrivalTimeCallback for $name {
            fn name() -> &'static str
            where
                Self: Sized,
            {
                $topology_name
            }

            fn callback($ctx: &mut Context, $from: usize, $to: usize) -> Option<OrderedFloat<f64>> $connect_fn
        }
    };
}

pub(crate) use define_custom_arrival_time_callback;

macro_rules! define_custom_topology {
    ($name:ident, $topology_name:expr, $connect_fn:path) => {
        pub struct $name;

        impl Topology for $name {
            fn name() -> &'static str {
                $topology_name
            }

            fn connect(
                ctx: &mut Context,
                n_peers: usize,
                custom_list: Option<Vec<(usize, usize, Option<f64>)>>,
            ) {
                $connect_fn(ctx, n_peers, custom_list);
            }
        }
    };
}

pub(crate) use define_custom_topology;
