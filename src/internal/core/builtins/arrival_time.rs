use ordered_float::OrderedFloat;

use crate::internal::core::{Context, options::ArrivalTimeCallback};

macro_rules! define_custom_arrival_time_callback {
    ($name:ident, $topology_name:expr, |$ctx:ident, $from:ident, $to:ident| $connect_fn:block) => {
        pub struct $name;

        impl ArrivalTimeCallback for $name {
            fn name() -> &'static str
            where
                Self: Sized,
            {
                $topology_name
            }

            fn callback($ctx: &mut Context, $from: usize, $to: usize) -> OrderedFloat<f64> $connect_fn
        }
    };
}

define_custom_arrival_time_callback!(ConstantArrivalTime, "constant", |_ctx, _from, _to| {
    OrderedFloat(1.0)
});

define_custom_arrival_time_callback!(DistanceBasedArrivalTime, "distance", |ctx, from, to| {
    let (from_peer, to_peer) = match (ctx.peers.get(from), ctx.peers.get(to)) {
        (Some(from), Some(to)) => (from, to),
        _ => return OrderedFloat(0.0),
    };

    let dist = distance_between_points(from_peer.get_peer().position, to_peer.get_peer().position);

    OrderedFloat(dist / 1000.0)
});

fn distance_between_points(a: (f64, f64, f64), b: (f64, f64, f64)) -> f64 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2) + (a.2 - b.2).powi(2)).sqrt()
}
