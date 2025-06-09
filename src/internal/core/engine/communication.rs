use ordered_float::OrderedFloat;
use rand::Rng;
use rand_distr::num_traits::Zero;

use crate::internal::core::{
    Context, Message, config::LinkKind, engine, events::MessageDeliveryEvent, log,
};

pub fn send_message_to(
    ctx: &mut Context,
    from: usize,
    to: usize,
    msg: impl Message + 'static,
) -> bool {
    let drop_rate = ctx.get_drop_rate();
    // only generate random number if not zero
    let r = ctx.rng.random_range(0.0..1.0);

    println!("{drop_rate} <= {r}");
    if !drop_rate.is_zero() && drop_rate >= r {
        log::trace(
            ctx,
            format!("Message from {from} to {to} dropped due to drop_rate"),
        );
        return false;
    }

    let link_info = ctx.links.get(from).and_then(|map| map.get(&to)).cloned();

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
