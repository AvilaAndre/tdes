mod algorithms;
mod callbacks;
mod hooks;
mod message;
mod peer;
mod timers;

mod data;
mod family;
mod generalized_linear_model;
mod utils;

use data::{ModelData, chunk_nx, model_beta, model_data};
use faer::Mat;
use ordered_float::OrderedFloat;
use peer::GlmPeer;
use rand::Rng;
use serde_yaml::Value;

use crate::{
    internal::{
        Simulator,
        core::{
            Context, engine,
            hooks::SimulationHooks,
            options::{ExperimentOptions, Scenario},
        },
    },
    scenarios::distributed_generalized_linear_model::timers::{KillTimer, StartTimer},
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

        let y_len = data.y.nrows();
        let ncols = data.x.ncols();

        assert!(data.x.nrows() == y_len, "x.nrows() != y.nrows()");
        assert!(n_peers * (ncols + 1) < y_len, "split > ncols");

        let x_chunks = chunk_nx(data.x, n_peers);
        let y_chunks = chunk_nx(data.y, n_peers);

        for (i, (x, y)) in x_chunks.into_iter().zip(y_chunks.into_iter()).enumerate() {
            let (pos_x, pos_y) = match opts.topology.positions.as_ref().and_then(|v| v.get(i)) {
                Some(&(px, py, _)) => (px, py),
                None => (
                    ctx.rng.random_range(-100.0..=100.0) * 1000.0,
                    ctx.rng.random_range(-100.0..=100.0) * 1000.0,
                ),
            };

            engine::add_peer(ctx, GlmPeer::new(pos_x, pos_y, x, y));
        }

        simulator
            .topology_registry
            .connect_peers(ctx, opts.topology);
        ctx.message_delay_cb = simulator
            .arrival_time_registry
            .get_callback(opts.arrival_time);

        // init
        for peer_id in 0..ctx.peers.len() {
            engine::add_timer(ctx, OrderedFloat(0.0), StartTimer { peer_id });
        }

        if let Some(custom) = opts.extra_args {
            if let Some(Value::Bool(true)) = custom.get("kill_peer") {
                engine::add_timer(ctx, OrderedFloat(0.1), KillTimer::new(0));
            }
        }

        let mut hooks = SimulationHooks::default();
        hooks.set_on_simulation_finish_hook(hooks::on_simulation_finish_hook(beta_mat));

        engine::run(ctx, &hooks, opts.deadline);
    }
}
