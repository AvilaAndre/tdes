use crate::internal::core::peer::{CustomPeer, Peer};

use super::message::example_on_message_receive;

pub struct ExamplePeer {
    pub peer: Peer,
    pub value: u64,
}

impl ExamplePeer {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            peer: {
                let mut this = Peer::new(x, y, z);
                this.on_message_receive = example_on_message_receive;
                this
            },
            value: 0,
        }
    }
}

impl CustomPeer for ExamplePeer {
    fn get_peer(&self) -> &Peer {
        &self.peer
    }

    fn get_peer_mut(&mut self) -> &mut Peer {
        &mut self.peer
    }
}
