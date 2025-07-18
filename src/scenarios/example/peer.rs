use crate::internal::core::{
    macros::define_custom_peer,
    peer::{CustomPeer, PeerInfo},
};

use super::message::example_on_message_receive;

#[derive(Default)]
pub struct ExamplePeer {
    pub peer_info: PeerInfo,
    pub value: u64,
}

impl ExamplePeer {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            peer_info: PeerInfo::new(x, y, z).with_on_message_receive(example_on_message_receive),
            value: 0,
        }
    }
}

define_custom_peer!(ExamplePeer);
