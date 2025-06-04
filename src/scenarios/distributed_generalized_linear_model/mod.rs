mod algorithms;
mod callbacks;
mod hooks;
mod message;
mod peer;
mod timer;

mod data;
mod family;
mod generalized_linear_model;
mod utils;

use algorithms::peer_start;
use data::{ModelData, chunk_nx, model_beta, model_data};
use faer::Mat;
use peer::GlmPeer;
use rand::{Rng, distr::Uniform};

use crate::internal::{
    Simulator,
    core::{
        Context, engine,
        options::{ExperimentOptions, Scenario},
    },
};

pub struct DistributedGeneralizedLinearModel;

impl Scenario for DistributedGeneralizedLinearModel {
    fn name() -> &'static str {
        "dglm"
    }

    fn description() -> &'static str {
        "A distributed implementation of the generalized linear model."
    }

    fn start(ctx: &mut Context, simulator: &Simulator, opts: ExperimentOptions) {
        let n_peers: usize = opts.topology.n_peers;

        let data: ModelData = match model_data("glm") {
            Ok(d) => d,
            Err(e) => panic!("Failed to load model_data: {e}"),
        };
        let beta = match model_beta("glm") {
            Ok(b) => b,
            Err(e) => panic!("Failed to load model_beta: {e}"),
        };
        let beta_mat = Mat::from_fn(
            beta.len(),
            beta.first().map_or(0, std::vec::Vec::len),
            |i, j| *beta.get(i).unwrap().get(j).unwrap(),
        );

        ctx.on_simulation_finish_hook = Some(hooks::on_simulation_finish_hook(beta_mat));

        let y_len = data.y.nrows();
        let ncols = data.x.ncols();

        assert!(data.x.nrows() == y_len, "x.nrows() != y.nrows()");
        assert!(n_peers * (ncols + 1) < y_len, "split > ncols");

        let x_chunks = chunk_nx(data.x, n_peers);
        let y_chunks = chunk_nx(data.y, n_peers);

        for (x, y) in x_chunks.into_iter().zip(y_chunks.into_iter()) {
            let (rx, ry) = (
                ctx.rng.sample(Uniform::new(-100.0, 100.0).unwrap()),
                ctx.rng.sample(Uniform::new(-100.0, 100.0).unwrap()),
            );

            engine::add_peer(ctx, Box::new(GlmPeer::new(rx, ry, x, y)));
        }

        simulator
            .topology_registry
            .connect_peers(ctx, opts.topology);
        ctx.message_delay_cb = simulator
            .arrival_time_registry
            .get_callback(opts.arrival_time);

        for i in 0..n_peers {
            peer_start(ctx, i);
        }

        engine::run(ctx, opts.deadline);
    }
}
