use crate::internal::{
    context::Context,
    message_passing::send_message_to,
    peer::{CustomPeer, Peer},
};

pub struct ExamplePeer {
    pub peer: Peer,
}

fn example_on_message_receive(ctx: &mut Context, receiver_id: usize) {
    send_message_to(ctx, receiver_id, receiver_id);
}

impl ExamplePeer {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            peer: Peer::new(x, y, z).with_on_message_receive(example_on_message_receive),
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
