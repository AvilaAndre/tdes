use faer::Mat;

use crate::internal::core::{Context, context::CustomHook, log};

use super::{peer::GlmPeer, utils::mat_allclose_default};

pub fn on_simulation_finish_hook(central: Mat<f64>) -> CustomHook {
    Box::new(move |ctx| {
        let coefficients: Vec<Mat<f64>> = ctx
            .peers
            .iter()
            .filter_map(|p| {
                p.downcast_ref::<GlmPeer>()
                    .map(|peer| peer.state.model.coefficients.clone())
            })
            .collect();

        check(ctx, &central, &coefficients);
    })
}

fn check(ctx: &mut Context, central: &Mat<f64>, coefficients: &[Mat<f64>]) {
    let res = coefficients
        .iter()
        .all(|coef| mat_allclose_default(coef, central));

    log::info(
        ctx,
        format!("Are the coefficients from every peer equal to central's? {res}"),
    );
}
