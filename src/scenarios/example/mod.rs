mod message;
mod peer;

use message::ExampleMessage;
use peer::ExamplePeer;

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
            if let Some(positions) = opts.topology.positions.as_ref() {
                let (x, y, z) = positions.get(i).unwrap_or(&(0.0, 0.0, Some(0.0)));
                engine::add_peer(ctx, ExamplePeer::new(*x, *y, z.unwrap_or(0.0)));
            } else {
                engine::add_peer(ctx, ExamplePeer::new(0.0, 0.0, 0.0));
            }
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
