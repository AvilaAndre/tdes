mod algorithms;
mod callbacks;
mod data;
mod discovery;
mod family;
mod generalized_linear_model;
mod hooks;
mod message;
mod peer;
mod timers;
mod utils;

use data::{ModelData, chunk_nx, model_beta, model_data};
use faer::Mat;
use ordered_float::OrderedFloat;
use peer::PGlmPeer;
use rand::Rng;
use serde_yaml::Value;

use crate::{
    internal::{
        Simulator,
        core::{
            Context, engine,
            hooks::SimulationHooks,
            macros::get_peer_of_type,
            options::{ExperimentOptions, Scenario},
            peer::CustomPeer,
        },
    },
    scenarios::proposed_dglm::timers::{KillTimer, ReviveTimer, StartTimer, TimeoutTimer},
};

pub struct ProposedDglm;

impl Scenario for ProposedDglm {
    fn name() -> &'static str {
        "proposed_dglm"
    }

    fn description() -> &'static str {
        "A proposed implementation of the distributed generalized linear model algorithm."
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

            engine::add_peer(ctx, PGlmPeer::new(pos_x, pos_y, x, y));
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

        // tick
        engine::add_timer(ctx, OrderedFloat(0.01), TimeoutTimer { interval: 0.5 });

        if let Some(custom) = opts.extra_args {
            if let Some(Value::Bool(true)) = custom.get("kill_peer") {
                engine::add_timer(ctx, OrderedFloat(0.1), KillTimer::new(0));
            }
            if let Some(Value::Bool(true)) = custom.get("revive_peer") {
                let target = 0;

                let peer: &mut PGlmPeer =
                    get_peer_of_type!(ctx, target, PGlmPeer).expect("peer should exist");
                peer.kill();
                engine::add_timer(ctx, OrderedFloat(1.0), ReviveTimer { target });
            }
        }

        let mut hooks = SimulationHooks::default();
        hooks.set_on_simulation_finish_hook(hooks::on_simulation_finish_hook(beta_mat));

        engine::run(ctx, &hooks, opts.deadline);
    }
}
