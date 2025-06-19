use faer::Mat;
use serde_json::json;

use crate::internal::core::{Context, hooks::CustomOnFinishHook, log};

use super::{peer::PGlmPeer, utils::mat_allclose_default};

pub fn on_simulation_finish_hook(central: Mat<f64>) -> CustomOnFinishHook {
    Box::new(move |ctx| {
        let coefficients: Vec<Mat<f64>> = ctx
            .peers
            .iter()
            .filter_map(|p| {
                p.downcast_ref::<PGlmPeer>()
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

    let data = json!({
        "coefficients": format!("{coefficients:?}"),
        "central": format!("{central:?}")
    });
    log::metrics(ctx, "central", &data);

    log::info(
        ctx,
        format!("Are the coefficients from every peer equal to central's? {res}"),
    );
}
