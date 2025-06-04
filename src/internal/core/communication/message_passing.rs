use ordered_float::OrderedFloat;

use crate::internal::core::log;

use super::{
    super::{Context, events::MessageDeliveryEvent},
    message::Message,
};

pub fn send_message_to(
    ctx: &mut Context,
    from: usize,
    to: usize,
    msg: Option<Box<dyn Message>>,
) -> bool {
    // TODO: Add communication failures

    if let Some(latency_opt) = ctx.links.get(from).and_then(|map| map.get(&to)) {
        if let Some(latency) = latency_opt {
            ctx.add_event(MessageDeliveryEvent::create(
                ctx.clock + OrderedFloat(*latency),
                to,
                msg,
            ));
        } else if let Some(latency) = (ctx.message_delay_cb)(ctx, from, to) {
            ctx.add_event(MessageDeliveryEvent::create(ctx.clock + latency, to, msg));
        } else {
            log::warn(
                ctx,
                format!(
                    "Failed to send message from peer {from} to {to} because latency couldn't be calculated"
                ),
            );
            return false;
        }
        true
    } else {
        log::warn(
            ctx,
            format!(
                "Failed to send message from peer {from} to {to} because they are not connected"
            ),
        );
        false
    }
}
