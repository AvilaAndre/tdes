use faer::Mat;

use crate::{
    internal::context::{Context, CustomHook},
    simulations::distributed_generalized_linear_model::peer::GlmPeer,
};

pub fn on_simulation_finish_hook(central: Mat<f64>) -> CustomHook {
    Box::new(move |ctx| inner_func(ctx, &central))
}

fn inner_func(ctx: &Context, central: &Mat<f64>) {
    let coefficients: Vec<Mat<f64>> = ctx
        .peers
        .iter()
        .filter_map(|p| {
            p.downcast_ref::<GlmPeer>()
                .map(|peer| peer.state.model.coefficients.clone())
        })
        .collect();

    check(central.clone(), coefficients);

    //println!(
    //    "The resulting averages are the following {:?}\nshould_be: {:?}\navg: {:?}\nmax: {:?}\nmin: {:?}",
    //)
}

fn check(central: Mat<f64>, coefficients: Vec<Mat<f64>>) {
    println!("{:?}", coefficients);
    println!("central {:?}", central);
    /*
    res = all(
        np.allclose(msg.coefficients, central.coefficients) for msg in coefficients_msgs
    )

    this_actor.info(f"Are the coefficients from every peer equal to central's? {res}")
    */
}
