use ordered_float::OrderedFloat;

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
        } else {
            let latency = (ctx.message_delay_cb)(ctx, from, to);
            ctx.add_event(MessageDeliveryEvent::create(ctx.clock + latency, to, msg));
        }
        true
    } else {
        false
    }
}
