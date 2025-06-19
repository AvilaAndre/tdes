mod algorithms;
mod callbacks;
mod hooks;
mod message;
mod peer;
mod timer;

use ordered_float::OrderedFloat;
use peer::FlowUpdatingPairwisePeer;
use rand::Rng;
use timer::TickTimer;

use crate::{
    internal::{
        Simulator,
        core::{
            Context, engine,
            hooks::SimulationHooks,
            options::{ExperimentOptions, Scenario},
        },
    },
    scenarios::flow_updating_pairwise::timer::{MetricsTimer, StartTimer},
};

pub struct FlowUpdatingPairwise {}

impl Scenario for FlowUpdatingPairwise {
    fn name() -> &'static str
    where
        Self: Sized,
    {
        "flow_updating_pairwise"
    }

    fn description() -> &'static str
    where
        Self: Sized,
    {
        "An implementation of the flow updating pairwise algorithm."
    }

    fn start(ctx: &mut Context, simulator: &Simulator, opts: ExperimentOptions) {
        let n_peers = opts.topology.n_peers;

        let peer_values: Vec<i32> = opts
            .extra_args
            .as_ref()
            .and_then(|custom| custom.get("values"))
            .and_then(|v| v.as_sequence())
            .map(|seq| {
                seq.iter()
                    .filter_map(|val| val.as_f64().map(|f| f as i32))
                    .collect()
            })
            .unwrap_or_default();

        for i in 0..n_peers {
            let (rx, ry) = (
                ctx.rng.random_range(-100000.0..=100000.0),
                ctx.rng.random_range(-100000.0..=100000.0),
            );

            let val: i32 = *peer_values.get(i).unwrap_or(&ctx.rng.random_range(0..=80));

            engine::add_peer(ctx, FlowUpdatingPairwisePeer::new(rx, ry, val));
        }

        simulator
            .topology_registry
            .connect_peers(ctx, opts.topology);
        ctx.message_delay_cb = simulator
            .arrival_time_registry
            .get_callback(opts.arrival_time);

        let mut hooks = SimulationHooks::default();
        hooks.set_on_simulation_finish_hook(Box::new(hooks::on_simulation_finish_hook));

        let tick_interval = 0.001;
        let metrics_interval = 0.001;

        // init
        for peer_id in 0..ctx.peers.len() {
            engine::add_timer(ctx, OrderedFloat(0.0), StartTimer { peer_id });
        }

        engine::add_timer(
            ctx,
            OrderedFloat(tick_interval),
            TickTimer {
                interval: tick_interval,
            },
        );

        engine::add_timer(
            ctx,
            OrderedFloat(0.0),
            MetricsTimer {
                interval: metrics_interval,
            },
        );

        engine::run(ctx, &hooks, opts.deadline);
    }
}
