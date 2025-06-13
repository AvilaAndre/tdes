use crate::internal::core::{Context, Message, engine, log, macros::get_peer_of_type};

use super::peer::ExamplePeer;

#[derive(Debug, Clone)]
pub struct ExampleMessage {
    pub sender: usize,
}
impl Message for ExampleMessage {
    fn size_bytes(&self) -> u64 {
        1
    }
}

pub fn example_on_message_receive(ctx: &mut Context, receiver_id: usize, msg: &dyn Message) {
    let peer: &mut ExamplePeer =
        get_peer_of_type!(ctx, receiver_id, ExamplePeer).expect("peer should exist");

    if let Some(example_msg) = msg.downcast_ref::<ExampleMessage>() {
        let new_msg = ExampleMessage {
            sender: receiver_id,
        };

        peer.value += 1;
        let val = peer.value;

        let log_msg = format!("peer with id {} has value {}", receiver_id, peer.value);
        log::info(ctx, log_msg);

        if val < 5 {
            engine::send_message_to(ctx, receiver_id, example_msg.sender, new_msg);
        }
    }
}
