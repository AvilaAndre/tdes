use crate::internal::core::{Context, Message, engine, log, macros::get_peer_of_type};

use super::peer::ExamplePeer;

#[derive(Debug, Clone)]
pub struct ExampleMessage {
    pub sender: usize,
}
impl Message for ExampleMessage {}

pub fn example_on_message_receive(
    ctx: &mut Context,
    receiver_id: usize,
    msg: Option<Box<dyn Message>>,
) {
    let peer: &mut ExamplePeer =
        get_peer_of_type!(ctx, receiver_id, ExamplePeer).expect("peer should exist");

    if let Some(boxed_msg) = msg {
        if let Some(example_msg) = boxed_msg.downcast_ref::<ExampleMessage>() {
            let new_msg = Box::new(ExampleMessage {
                sender: receiver_id,
            });

            peer.value += 1;
            let val = peer.value;

            let log_msg = format!("peer with id {} has value {}", receiver_id, peer.value);
            log::info(ctx, log_msg);

            if val < 5 {
                engine::send_message_to(ctx, receiver_id, example_msg.sender, Some(new_msg));
            }
        }
    }
}
