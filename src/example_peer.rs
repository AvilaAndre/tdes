use crate::internal::{
    context::Context,
    message::Message,
    message_passing::send_message_to,
    peer::{CustomPeer, Peer},
};

pub struct ExamplePeer {
    pub peer: Peer,
    pub value: u64,
}

fn example_on_message_receive(
    ctx: &mut Context,
    receiver_id: usize,
    msg: Option<Box<dyn Message>>,
) {
    let peer: &mut ExamplePeer;

    // TODO: create macro to get peer
    if let Some(this_peer) = ctx.peers[receiver_id].downcast_mut::<ExamplePeer>() {
        peer = this_peer;
    } else {
        // TODO: Add error log that the receiving peer is of different type
        return;
    }

    if let Some(boxed_msg) = msg {
        if let Some(example_msg) = boxed_msg.downcast_ref::<ExampleMessage>() {
            let new_msg = Box::new(ExampleMessage {
                receiver: example_msg.sender,
                sender: receiver_id,
            });

            peer.value += 1;
            println!("peer with id {} has value {}", receiver_id, peer.value);

            if peer.value < 5 {
                send_message_to(ctx, receiver_id, example_msg.sender, Some(new_msg));
            }
        }
    }
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

#[derive(Debug, Clone)]
pub struct ExampleMessage {
    pub sender: usize,
    pub receiver: usize,
}
impl Message for ExampleMessage {}

#[derive(Debug, Clone)]
pub struct ExampleMessage2 {
    pub sender: usize,
    pub receiver: usize,
}
impl Message for ExampleMessage2 {}
