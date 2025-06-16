use ordered_float::OrderedFloat;
use rand::Rng;
use rand_distr::num_traits::Zero;

use crate::internal::core::{
    Context, Message, engine, events::MessageDeliveryEvent, experiment::LinkKind, log,
};

/// Verifies if peer 'from' can send a message to peer 'to' and
/// calculates the time it takes for the message to arrive.
/// If they message cannot be sent or the latency fails to be
/// calculated, this function returns None. If it suceeds, it
/// returns Some(latency), with latency being the latency value
/// calculated.
pub fn send_message_to(
    ctx: &mut Context,
    from: usize,
    to: usize,
    msg: impl Message + 'static,
) -> Option<OrderedFloat<f64>> {
    // Gets link, will be None if no link exists between peers
    let link_info = ctx.links.get(from).and_then(|map| map.get(&to)).copied();

    let drop_rate = ctx.get_drop_rate();
    // only generate random number if not zero
    if link_info.is_some() && !drop_rate.is_zero() && drop_rate >= ctx.rng.random_range(0.0..1.0) {
        log::trace(
            ctx,
            format!("Message from {from} to {to} dropped due to drop_rate"),
        );
        return None;
    }

    let mut latency = match link_info {
        // if has latency defined
        Some(Some(LinkKind::Latency(latency))) => OrderedFloat(latency),
        Some(bandwith_opt) => {
            let Some(mut delay) = (ctx.message_delay_cb)(ctx, from, to) else {
                log::warn(
                    ctx,
                    format!(
                        "Failed to send message from peer {from} to {to} because latency couldn't be calculated"
                    ),
                );
                return None;
            };

            // if has bandwidth defined
            if let Some(LinkKind::Bandwidth(bandwidth)) = &bandwith_opt {
                delay += (msg.size_bits() as f64) / bandwidth;
            }

            delay
        }
        None => {
            log::warn(
                ctx,
                format!(
                    "Failed to send message from peer {from} to {to} because they are not connected"
                ),
            );
            return None;
        }
    };

    // TODO: log jitter so that it can be visualized
    /*
    log::trace(ctx, format!("jitter {jitter}"));
    log::metrics(
        ctx,
        "jitter",
        &json!({
            "jitter": *jitter,
        }),
    );
    */
    latency += ctx.get_jitter_value();

    // ensure delay isn't negative
    if latency < OrderedFloat(0.0) {
        latency = OrderedFloat(0.0);
        log::global_warn(
            "Delay was set to 0 because after applying jitter the message delay was negative.",
        );
    }

    let duplicate_rate = ctx.get_duplicate_rate();
    // only generate random number if not zero
    if link_info.is_some()
        && !duplicate_rate.is_zero()
        && duplicate_rate >= ctx.rng.random_range(0.0..1.0)
    {
        engine::add_event(
            ctx,
            MessageDeliveryEvent::create_boxed(ctx.clock + latency, from, to, msg.clone_box()),
        );
        log::trace(
            ctx,
            format!("Message from {from} to {to} duplicated due to duplicate_rate"),
        );
    }

    engine::add_event(
        ctx,
        MessageDeliveryEvent::create(ctx.clock + latency, from, to, msg),
    );

    Some(latency)
}
