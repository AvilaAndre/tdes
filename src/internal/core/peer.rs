use super::{Context, Message};
use downcast_rs::{Downcast, impl_downcast};

type OnMessageReceiveCallback = fn(&mut Context, usize, Box<dyn Message>) -> ();

#[derive(Clone)]
pub struct PeerInfo {
    id: usize,
    instantiated: bool,
    alive: bool,
    pub position: (f64, f64, f64),
    pub on_message_receive: OnMessageReceiveCallback,
}

fn default_on_message_receive(_ctx: &mut Context, _receiver_id: usize, _msg: Box<dyn Message>) {
    // Does nothing.
}

impl PeerInfo {
    #[must_use]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            id: 0,
            instantiated: false,
            alive: true,
            position: (x, y, z),
            on_message_receive: default_on_message_receive,
        }
    }

    #[must_use]
    pub fn with_on_message_receive(mut self, on_message_receive: OnMessageReceiveCallback) -> Self {
        self.on_message_receive = on_message_receive;
        self
    }
}

impl Default for PeerInfo {
    fn default() -> Self {
        Self {
            id: 0,
            instantiated: false,
            alive: true,
            position: (0.0, 0.0, 0.0),
            on_message_receive: default_on_message_receive,
        }
    }
}

pub trait CustomPeer: Downcast {
    fn get_peer(&self) -> &PeerInfo;
    fn get_peer_mut(&mut self) -> &mut PeerInfo;
    fn instantiate(&mut self, id: usize) -> usize {
        self.get_peer_mut().id = id;
        self.get_peer_mut().instantiated = true;
        id
    }
    fn get_id(&self) -> usize {
        let p = self.get_peer();
        if !p.instantiated {
            panic!(
                "Attempted to get the id of a non-instantiated peer. Make sure the peer exists in the simulation context."
            )
        }
        p.id
    }
    fn revive(&mut self) {
        self.get_peer_mut().alive = true;
    }
    fn kill(&mut self) {
        self.get_peer_mut().alive = false;
    }
    fn is_alive(&self) -> bool {
        self.get_peer().alive
    }
}
impl_downcast!(CustomPeer);
