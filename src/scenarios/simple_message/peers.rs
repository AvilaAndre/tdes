use crate::{
    internal::core::{
        Context, Message, log,
        macros::{define_custom_peer, get_peer_of_type},
        peer::{CustomPeer, PeerInfo},
    },
    scenarios::simple_message::messages::EmptyMessage,
};

pub struct SimplePeer {
    pub peer_info: PeerInfo,
}

define_custom_peer!(SimplePeer);

impl SimplePeer {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            peer_info: PeerInfo::new(x, y, 0.0)
                .with_on_message_receive(simple_peer_on_message_receive),
        }
    }
}

pub fn simple_peer_on_message_receive(
    ctx: &mut Context,
    _sender_id: usize,
    receiver_id: usize,
    msg: &dyn Message,
) {
    let _peer: &mut SimplePeer =
        get_peer_of_type!(ctx, receiver_id, SimplePeer).expect("peer should exist");

    if let Some(_example_msg) = msg.downcast_ref::<EmptyMessage>() {
        log::info(ctx, format!("Peer {receiver_id} received a message"));
    }
}
