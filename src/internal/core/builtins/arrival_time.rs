use ordered_float::OrderedFloat;

use crate::internal::core::Context;

pub fn distance_based_arrival_time(ctx: &mut Context, from: usize, to: usize) -> OrderedFloat<f64> {
    let (from_peer, to_peer) = match (ctx.peers.get(from), ctx.peers.get(to)) {
        (Some(from), Some(to)) => (from, to),
        _ => return OrderedFloat(0.0),
    };

    let dist = distance_between_points(from_peer.get_peer().position, to_peer.get_peer().position);

    OrderedFloat(dist / 10000.0)
}

fn distance_between_points(a: (f64, f64, f64), b: (f64, f64, f64)) -> f64 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2) + (a.2 - b.2).powi(2)).sqrt()
}
