use crate::internal::{
    context::Context,
    message::Message,
    message_passing::send_message_to,
    peer::{CustomPeer, Peer},
};

pub struct ExamplePeer {
    pub peer: Peer,
}

fn example_on_message_receive(
    ctx: &mut Context,
    receiver_id: usize,
    msg: Option<Box<dyn Message>>,
) {
    if let Some(boxed_msg) = msg {
        if let Some(example_msg) = boxed_msg.downcast_ref::<ExampleMessage>() {
            let new_msg = Box::new(ExampleMessage {
                receiver: example_msg.sender,
                sender: receiver_id,
            });

            send_message_to(ctx, receiver_id, example_msg.sender, Some(new_msg));
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
