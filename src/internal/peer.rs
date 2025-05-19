use super::{context::Context, message::Message};
use downcast_rs::{Downcast, impl_downcast};

type OnMessageReceiveCallback =
    fn(ctx: &mut Context, receiver_id: usize, msg: Option<Box<dyn Message>>) -> ();

#[derive(Clone)]
pub struct Peer {
    pub id: Option<usize>,
    pub position: (f64, f64, f64),
    pub on_message_receive: OnMessageReceiveCallback,
}

fn default_on_message_receive(
    _ctx: &mut Context,
    _receiver_id: usize,
    _msg: Option<Box<dyn Message>>,
) {
}

impl Peer {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            id: None,
            position: (x, y, z),
            on_message_receive: default_on_message_receive,
        }
    }

    pub fn with_on_message_receive(mut self, on_message_receive: OnMessageReceiveCallback) -> Self {
        self.on_message_receive = on_message_receive;
        self
    }
}

pub trait CustomPeer: Downcast {
    fn get_peer(&self) -> &Peer;
    fn get_peer_mut(&mut self) -> &mut Peer;
}
impl_downcast!(CustomPeer);

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
