use ordered_float::OrderedFloat;

use crate::{
    internal::context::Context, simulations::flow_updating_pairwise::peer::FlowUpdatingPairwisePeer,
};

pub fn on_simulation_finish_hook(ctx: &Context) {
    let avgs: Vec<OrderedFloat<f64>> = ctx
        .peers
        .iter()
        .filter_map(|p| {
            p.downcast_ref::<FlowUpdatingPairwisePeer>()
                .map(|peer| OrderedFloat(peer.last_avg))
        })
        .collect();

    let total: OrderedFloat<f64> = avgs.iter().sum();

    println!(
        "The resulting averages are the following {:?}\navg: {:?}\nmax: {:?}\nmin: {:?}",
        avgs,
        total / avgs.len() as f64,
        avgs.iter().max().unwrap(),
        avgs.iter().min().unwrap(),
    )
}
