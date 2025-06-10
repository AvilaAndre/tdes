use crate::internal::core::{
    Context,
    experiment::{LinkInfo, LinkKind},
    log,
};

// Adds a link to another peer.
// If latency is provided, that value will always be used,
// if not, the simulator will calculate it using "message_delay_cb".
pub fn add_oneway_link(ctx: &mut Context, from: usize, to: usize, info: LinkInfo) {
    validate_link_info(info);
    if from < ctx.links.len() && to < ctx.links.len() {
        ctx.links[from].insert(to, info);
    } else {
        log::global_warn(format!(
            "Failed to create a one way link between peers {from} and {to} because at least one of them does not exist."
        ));
    }
}

// Adds two links between two neighbors with the same latency.
// If latency is provided, that value will always be used,
// if not, the simulator will calculate it using "message_delay_cb".
pub fn add_twoway_link(ctx: &mut Context, from: usize, to: usize, info: LinkInfo) {
    validate_link_info(info);

    if from < ctx.links.len() && to < ctx.links.len() {
        ctx.links[from].insert(to, info);
        ctx.links[to].insert(from, info);
    } else {
        log::global_warn(format!(
            "Failed to create a two way link between peers {from} and {to} because at least one of them does not exist."
        ));
    }
}

fn validate_link_info(info_opt: LinkInfo) {
    if let Some(info) = info_opt {
        let (val, type_name) = match info {
            LinkKind::Bandwidth(b) => (b, "bandwith"),
            LinkKind::Latency(l) => (l, "latency"),
        };

        if val.is_sign_negative() {
            let reason = format!("A link was provided with negative {type_name}.");
            log::global_error(&reason);
            panic!("{reason}");
        } else if !val.is_normal() {
            let reason = format!("A link was provided with an invalid {type_name}.");
            log::global_error(&reason);
            panic!("{reason}");
        }
    }
}
