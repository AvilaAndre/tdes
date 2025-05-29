use ordered_float::OrderedFloat;

use crate::internal::{
    events::types::message_delivery::MessageDeliveryEvent, utils::distance_between_points,
};

use super::{context::Context, message::Message, peer::CustomPeer};

pub fn send_message_to(
    ctx: &mut Context,
    from: usize,
    to: usize,
    msg: Option<Box<dyn Message>>,
) -> bool {
    // TODO: this could be a custom method in ctx
    // calculate time between peers
    let arrival_time = ctx.clock + (ctx.message_delay)(ctx, from, to);

    ctx.add_event(MessageDeliveryEvent::create(arrival_time, to, msg));

    true
}

pub fn distance_based_arrival_time(ctx: &mut Context, from: usize, to: usize) -> OrderedFloat<f64> {
    let (from_peer, to_peer) = match (ctx.peers.get(from), ctx.peers.get(to)) {
        (Some(from), Some(to)) => (from, to),
        _ => return OrderedFloat(0.0),
    };

    return OrderedFloat(distance_between_points(
        from_peer.get_peer().position,
        to_peer.get_peer().position,
    )) / 10000.0;
}
