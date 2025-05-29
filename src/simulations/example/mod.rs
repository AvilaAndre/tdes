mod message;
mod peer;

use message::ExampleMessage;
use peer::ExamplePeer;

use crate::internal::{context::Context, message_passing::send_message_to};

pub fn start(ctx: &mut Context) {
    let peer1_idx = ctx.add_peer(Box::new(ExamplePeer::new(1.0, 1.0, 0.0)));
    let peer2_idx = ctx.add_peer(Box::new(ExamplePeer::new(-1.0, 1.0, 0.0)));

    send_message_to(
        ctx,
        peer1_idx,
        peer2_idx,
        Some(Box::new(ExampleMessage { sender: peer1_idx })),
    );

    ctx.run();
}
