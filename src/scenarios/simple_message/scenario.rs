use ordered_float::OrderedFloat;

use crate::{
    internal::{
        Simulator,
        core::{
            Context, engine,
            hooks::SimulationHooks,
            options::{ExperimentOptions, Scenario},
        },
    },
    scenarios::simple_message::starts::StartTimer,
};

use super::peers::SimplePeer;

pub struct SimpleMessageScenario {}

impl Scenario for SimpleMessageScenario {
    fn name() -> &'static str
    where
        Self: Sized,
    {
        "simple_message"
    }

    fn description() -> &'static str
    where
        Self: Sized,
    {
        "An implementation used to evaluate the simulator."
    }

    fn start(ctx: &mut Context, simulator: &Simulator, opts: ExperimentOptions) {
        for i in 0..opts.topology.n_peers {
            let (pos_x, pos_y) = match opts.topology.positions.as_ref().and_then(|v| v.get(i)) {
                Some(&(px, py, _)) => (px, py),
                None => (0.0, 0.0),
            };

            engine::add_peer(ctx, SimplePeer::new(pos_x, pos_y));
        }

        simulator
            .topology_registry
            .connect_peers(ctx, opts.topology);
        ctx.message_delay_cb = simulator
            .arrival_time_registry
            .get_callback(opts.arrival_time);

        let message_size = opts
            .extra_args
            .as_ref()
            .and_then(|custom| custom.get("msg_size"))
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        // start event
        engine::add_timer(ctx, OrderedFloat(0.0), StartTimer { message_size });

        engine::run(ctx, &SimulationHooks::default(), opts.deadline);
    }
}
