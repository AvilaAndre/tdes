use ordered_float::OrderedFloat;

use crate::internal::core::{
    Context, macros::define_custom_arrival_time_callback, options::ArrivalTimeCallback,
};

define_custom_arrival_time_callback!(ConstantArrivalTime, "constant", |_ctx, _from, _to| {
    Some(OrderedFloat(1.0))
});

define_custom_arrival_time_callback!(
    DistanceBasedArrivalTime,
    "distance",
    |ctx, from, to| { distance(ctx, from, to) }
);

// optical fiber latency per kilometer in seconds
const OPTICAL_FIBER: f64 = 0.350e-6;

fn distance(ctx: &mut Context, from: usize, to: usize) -> Option<OrderedFloat<f64>> {
    let (from_peer, to_peer) = (ctx.peers.get(from)?, ctx.peers.get(to)?);

    let dist = distance_between_points(from_peer.get_peer().position, to_peer.get_peer().position);

    Some(OrderedFloat(dist * OPTICAL_FIBER))
}

// In kilometers
fn distance_between_points(a: (f64, f64, f64), b: (f64, f64, f64)) -> f64 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2) + (a.2 - b.2).powi(2)).sqrt()
}
