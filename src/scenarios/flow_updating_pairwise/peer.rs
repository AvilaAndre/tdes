use std::collections::HashMap;

use crate::internal::core::{
    macros::define_custom_peer,
    peer::{CustomPeer, PeerInfo},
};

use super::callbacks;

pub struct FlowUpdatingPairwisePeer {
    pub peer_info: PeerInfo,
    pub value: i32,
    pub flows: HashMap<usize, f64>,
    pub estimates: HashMap<usize, f64>,
    pub ticks_since_last_avg: HashMap<usize, u32>,
    pub last_avg: f64,
}

define_custom_peer!(FlowUpdatingPairwisePeer);

impl FlowUpdatingPairwisePeer {
    pub fn new(x: f64, y: f64, z: f64, value: i32) -> Self {
        Self {
            peer_info: PeerInfo::new(x, y, z)
                .with_on_message_receive(callbacks::example_on_message_receive),
            value,
            flows: HashMap::new(),
            estimates: HashMap::new(),
            ticks_since_last_avg: HashMap::new(),
            last_avg: 0.0,
        }
    }
}
