use std::collections::HashMap;

use crate::internal::peer::{CustomPeer, Peer};

use super::callbacks;

pub struct FlowUpdatingPairwisePeer {
    pub peer: Peer,
    pub value: i32,
    pub flows: HashMap<usize, f64>,
    pub estimates: HashMap<usize, f64>,
    pub ticks_since_last_avg: HashMap<usize, u32>,
    pub last_avg: f64,
}

impl FlowUpdatingPairwisePeer {
    pub fn new(x: f64, y: f64, z: f64, value: i32) -> Self {
        Self {
            peer: {
                let mut this = Peer::new(x, y, z);
                this.on_message_receive = callbacks::example_on_message_receive;
                this
            },
            value,
            flows: HashMap::new(),
            estimates: HashMap::new(),
            ticks_since_last_avg: HashMap::new(),
            last_avg: 0.0,
        }
    }
}

impl CustomPeer for FlowUpdatingPairwisePeer {
    fn get_peer(&self) -> &Peer {
        &self.peer
    }

    fn get_peer_mut(&mut self) -> &mut Peer {
        &mut self.peer
    }
}
