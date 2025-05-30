use faer::Mat;

use crate::{
    internal::context::Context, simulations::distributed_generalized_linear_model::peer::GlmPeer,
};

pub fn on_simulation_finish_hook(ctx: &Context) {
    let coefficients: Vec<Mat<f64>> = ctx
        .peers
        .iter()
        .filter_map(|p| {
            p.downcast_ref::<GlmPeer>()
                .map(|peer| peer.state.model.coefficients.clone())
        })
        .collect();

    println!("{:?}", coefficients);
    //println!(
    //    "The resulting averages are the following {:?}\nshould_be: {:?}\navg: {:?}\nmax: {:?}\nmin: {:?}",
    //)
}
