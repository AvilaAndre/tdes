use ordered_float::OrderedFloat;
use rand::Rng;
use rand_distr::num_traits::Zero;

use crate::internal::core::{
    Context, Message,
    config::LinkKind,
    delay_modifiers::{self, DelayModifiers},
    engine,
    events::MessageDeliveryEvent,
    log,
};

pub fn send_message_to(
    ctx: &mut Context,
    from: usize,
    to: usize,
    msg: impl Message + 'static,
) -> bool {
    let drop_rate = ctx.get_drop_rate();
    // only generate random number if not zero
    if !drop_rate.is_zero() && drop_rate >= ctx.rng.random_range(0.0..1.0) {
        log::trace(
            ctx,
            format!("Message from {from} to {to} dropped due to drop_rate"),
        );
        return false;
    }

    // Gets link, will be None if no link exists between peers
    let link_info = ctx.links.get(from).and_then(|map| map.get(&to)).cloned();

    let event = match link_info {
        // if has latency defined
        Some(Some(LinkKind::Latency(latency))) => {
            MessageDeliveryEvent::create(ctx.clock + OrderedFloat(latency), to, msg)
        }
        Some(bandwith_opt) => {
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
            if let Some(LinkKind::Bandwidth(bandwidth)) = &bandwith_opt {
                delay += (msg.size_bits() as f64) / bandwidth;
            }

            // TODO: Add jitter
            // delay += ctx.jitter
            delay += delay_modifiers::get_value(ctx, DelayModifiers::Weibull(1.0, 1.0));

            // ensure delay isn't negative
            delay = delay.max(OrderedFloat(0.0));

            MessageDeliveryEvent::create(ctx.clock + delay, to, msg)
        }
        None => {
            log::warn(
                ctx,
                format!(
                    "Failed to send message from peer {from} to {to} because they are not connected"
                ),
            );
            return false;
        }
    };

    engine::add_event(ctx, event);

    true
}
