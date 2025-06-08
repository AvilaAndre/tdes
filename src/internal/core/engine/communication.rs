use ordered_float::OrderedFloat;

use crate::internal::core::{
    Context, Message, config::LinkKind, engine, events::MessageDeliveryEvent, log,
};

pub fn send_message_to(
    ctx: &mut Context,
    from: usize,
    to: usize,
    msg: impl Message + 'static,
) -> bool {
    // TODO: Add communication failures

    let link_info = ctx.links.get(from).and_then(|map| map.get(&to)).cloned();

    // TODO: Here
    if let Some(link_info) = link_info {
        // if has latency defined
        if let Some(LinkKind::Latency(latency)) = &link_info {
            engine::add_event(
                ctx,
                MessageDeliveryEvent::create(ctx.clock + OrderedFloat(*latency), to, msg),
            );
        } else {
            let mut delay = match (ctx.message_delay_cb)(ctx, from, to) {
                Some(val) => val,
                None => {
                    log::warn(
                        ctx,
                        format!(
                            "Failed to send message from peer {from} to {to} because latency couldn't be calculated"
                        ),
                    );
                    return false;
                }
            };

            // if has bandwidth defined
            if let Some(LinkKind::Bandwidth(bandwidth)) = &link_info {
                delay += (msg.size_bits() as f64) / bandwidth;
            }

            engine::add_event(
                ctx,
                MessageDeliveryEvent::create(ctx.clock + delay, to, msg),
            );
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
