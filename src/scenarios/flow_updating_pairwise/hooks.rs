use ordered_float::OrderedFloat;

use super::FlowUpdatingPairwisePeer;
use crate::internal::core::{Context, log};

pub fn on_simulation_finish_hook(ctx: &mut Context) {
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
                .map(|peer| f64::from(peer.value))
        })
        .collect();

    let total: OrderedFloat<f64> = avgs.iter().sum();

    log::info(ctx, format!("initial values {real_total:?}"));
    log::info(
        ctx,
        format!("The resulting averages are the following {avgs:?}"),
    );
    log::info(
        ctx,
        format!(
            "should_be: {:?}",
            real_total.iter().sum::<f64>() / real_total.len() as f64,
        ),
    );
    log::info(ctx, format!("avg: {:?}", total / avgs.len() as f64,));
    log::info(ctx, format!("max: {:?}", avgs.iter().max().unwrap(),));
    log::info(ctx, format!("min: {:?}", avgs.iter().min().unwrap(),));
}

pub fn finish_condition_hook(ctx: &mut Context) -> bool {
    log::info(ctx, format!("check finish_condition_hook"));

    return false;
}
