use crate::internal::core::{log, Context};

// Adds a link to another peer.
// If latency is provided, that value will always be used,
// if not, the simulator will calculate it using "message_delay_cb".
pub fn add_oneway_link(ctx: &mut Context, from: usize, to: usize, latency: Option<f64>) {
    if from < ctx.links.len() && to < ctx.links.len() {
        ctx.links[from].insert(to, latency);
    } else {
        log::global_warn(format!(
            "Failed to create a one way link between peers {from} and {to} because at least one of them does not exist."
        ));
    }
}

// Adds two links between two neighbors with the same latency.
// If latency is provided, that value will always be used,
// if not, the simulator will calculate it using "message_delay_cb".
pub fn add_twoway_link(ctx: &mut Context, from: usize, to: usize, latency: Option<f64>) {
    if from < ctx.links.len() && to < ctx.links.len() {
        ctx.links[from].insert(to, latency);
        ctx.links[to].insert(from, latency);
    } else {
        log::global_warn(format!(
            "Failed to create a two way link between peers {from} and {to} because at least one of them does not exist."
        ));
    }
}
