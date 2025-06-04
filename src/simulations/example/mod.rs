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
        let peer1_idx = ctx.add_peer(Box::new(ExamplePeer::new(1.0, 1.0, 0.0)));
        let peer2_idx = ctx.add_peer(Box::new(ExamplePeer::new(-1.0, 1.0, 0.0)));

        simulator
            .topology_registry
            .connect_peers(ctx, opts.topology);

        ctx.message_delay_cb = simulator
            .arrival_time_registry
            .get_callback(opts.arrival_time);

        send_message_to(
            ctx,
            peer1_idx,
            peer2_idx,
            Some(Box::new(ExampleMessage { sender: peer1_idx })),
        );

        ctx.run();
    }
}
