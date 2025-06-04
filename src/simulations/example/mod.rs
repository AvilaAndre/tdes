mod message;
mod peer;

use message::ExampleMessage;
use peer::ExamplePeer;

use crate::internal::{
    Simulator,
    core::{
        Context, communication::send_message_to, options::ExperimentOptions, simulation::Simulation,
    },
};

pub struct Example {}

impl Simulation for Example {
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
            if i < opts.topology.positions.len() {
                let (x, y, z) = opts.topology.positions[i];
                ctx.add_peer(Box::new(ExamplePeer::new(x, y, z.unwrap_or(0.0))));
            } else {
                ctx.add_peer(Box::new(ExamplePeer::new(0.0, 0.0, 0.0)));
            }
        }

        simulator
            .topology_registry
            .connect_peers(ctx, opts.topology);

        ctx.message_delay_cb = simulator
            .arrival_time_registry
            .get_callback(opts.arrival_time);

        send_message_to(ctx, 0, 1, Some(Box::new(ExampleMessage { sender: 0 })));

        ctx.run();
    }
}
