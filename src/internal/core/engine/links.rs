use crate::internal::core::{
    Context,
    experiment::{LinkInfo, LinkKind},
    log,
};

/// Adds a link to another peer.
/// If latency is provided, that value will always be used,
/// if not, the simulator will calculate it using `message_delay_cb`.
pub fn add_oneway_link(ctx: &mut Context, from: usize, to: usize, info: LinkInfo) {
    if let Err(err) = validate_link_info(info) {
        log::global_warn(format!("Failed to create a two way link, reason: {err}"));
        return;
    }

    if from < ctx.links.len() && to < ctx.links.len() {
        ctx.links[from].insert(to, info);
    } else {
        log::global_warn(format!(
            "Failed to create a one way link between peers {from} and {to} because at least one of them does not exist."
        ));
    }
}

/// Adds two links between two neighbors with the same latency.
/// If latency is provided, that value will always be used,
/// if not, the simulator will calculate it using `message_delay_cb`.
pub fn add_twoway_link(ctx: &mut Context, from: usize, to: usize, info: LinkInfo) {
    if let Err(err) = validate_link_info(info) {
        log::global_warn(format!("Failed to create a two way link, reason: {err}"));
        return;
    }

    if from < ctx.links.len() && to < ctx.links.len() {
        ctx.links[from].insert(to, info);
        ctx.links[to].insert(from, info);
    } else {
        log::global_warn(format!(
            "Failed to create a two way link between peers {from} and {to} because at least one of them does not exist."
        ));
    }
}

fn validate_link_info(info_opt: Option<LinkKind>) -> Result<(), String> {
    let (bandwidth, latency) = match info_opt {
        Some(LinkKind::Bandwidth(b)) => (Some(b), None),
        Some(LinkKind::Latency(l)) => (None, Some(l)),
        Some(LinkKind::Full { bandwidth, latency }) => (Some(bandwidth), Some(latency)),
        None => (None, None),
    };

    for (value_opt, type_name) in vec![(bandwidth, "bandwith"), (latency, "latency")] {
        if let Some(val) = value_opt {
            if val.is_sign_negative() {
                return Err(format!("A link was provided with negative {type_name}."));
            } else if !val.is_normal() {
                return Err(format!("A link was provided with an invalid {type_name}."));
            }
        }
    }

    Ok(())
}
