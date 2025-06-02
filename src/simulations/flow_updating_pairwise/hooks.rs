use ordered_float::OrderedFloat;

use crate::internal::core::Context;
use super::FlowUpdatingPairwisePeer;

pub fn on_simulation_finish_hook(ctx: &Context) {
    let avgs: Vec<OrderedFloat<f64>> = ctx
        .peers
        .iter()
        .filter_map(|p| {
            p.downcast_ref::<FlowUpdatingPairwisePeer>()
                .map(|peer| OrderedFloat(peer.last_avg))
        })
        .collect();

    let real_total: Vec<f64> = ctx
        .peers
        .iter()
        .filter_map(|p| {
            p.downcast_ref::<FlowUpdatingPairwisePeer>()
                .map(|peer| peer.value as f64)
        })
        .collect();

    let total: OrderedFloat<f64> = avgs.iter().sum();

    println!("{:?}", real_total);
    println!(
        "The resulting averages are the following {:?}\nshould_be: {:?}\navg: {:?}\nmax: {:?}\nmin: {:?}",
        avgs,
        real_total.iter().sum::<f64>() / real_total.len() as f64,
        total / avgs.len() as f64,
        avgs.iter().max().unwrap(),
        avgs.iter().min().unwrap(),
    )
}
