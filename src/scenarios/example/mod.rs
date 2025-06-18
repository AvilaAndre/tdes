mod message;
mod peer;

use message::ExampleMessage;
use peer::ExamplePeer;
use rand::Rng;

use crate::internal::{
    Simulator,
    core::{
        Context, engine,
        hooks::SimulationHooks,
        options::{ExperimentOptions, Scenario},
    },
};

pub struct Example {}

impl Scenario for Example {
    fn name() -> &'static str
    where
        Self: Sized,
    {
        "example"
    }

    fn description() -> &'static str
    where
        Self: Sized,
    {
        "An example simulation."
    }

    fn start(ctx: &mut Context, simulator: &Simulator, opts: ExperimentOptions) {
        for i in 0..opts.topology.n_peers {
            let (pos_x, pos_y, pos_z) =
                match opts.topology.positions.as_ref().and_then(|v| v.get(i)) {
                    Some(&(px, py, pz)) => (px, py, pz.unwrap_or(0.0)),
                    None => (
                        ctx.rng.random_range(-10.0..=10.0),
                        ctx.rng.random_range(-10.0..=10.0),
                        0.0,
                    ),
                };

            engine::add_peer(ctx, ExamplePeer::new(pos_x, pos_y, pos_z));
        }

        simulator
            .topology_registry
            .connect_peers(ctx, opts.topology);

        ctx.message_delay_cb = simulator
            .arrival_time_registry
            .get_callback(opts.arrival_time);

        engine::send_message_to(ctx, 0, 1, ExampleMessage { sender: 0 });

        engine::run(ctx, &SimulationHooks::default(), opts.deadline);
    }
}
