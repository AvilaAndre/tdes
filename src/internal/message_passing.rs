use crate::internal::{
    events::types::message_delivery::MessageDeliveryEvent, utils::distance_between_points,
};

use super::{context::Context, message::Message};

pub fn send_message_to(
    ctx: &mut Context,
    from: usize,
    to: usize,
    msg: Option<Box<dyn Message>>,
) -> bool {
    println!(
        "[{}] send_message_to from {:?} to {:?} with content",
        ctx.clock, from, to
    );

    let from_peer = ctx.peers.get(from);
    let to_peer = ctx.peers.get(to);

    if from_peer.is_none() || to_peer.is_none() {
        // TODO: Debug reason why
        return false;
    }

    // calculate time between peers
    let arrival_time = ctx.clock
        + distance_between_points(
            from_peer.unwrap().get_peer().position,
            to_peer.unwrap().get_peer().position,
        );

    ctx.add_event(MessageDeliveryEvent::create(arrival_time, from, to, msg));

    true
}
