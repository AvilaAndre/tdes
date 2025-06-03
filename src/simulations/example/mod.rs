mod message;
mod peer;

use message::ExampleMessage;
use peer::ExamplePeer;

use crate::internal::core::{
    Context,
    communication::send_message_to,
    options::{ExperimentOptions, TopologyRegistry},
    simulation::Simulation,
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

    fn start(ctx: &mut Context, topology_registry: &TopologyRegistry, opts: ExperimentOptions) {
        let peer1_idx = ctx.add_peer(Box::new(ExamplePeer::new(1.0, 1.0, 0.0)));
        let peer2_idx = ctx.add_peer(Box::new(ExamplePeer::new(-1.0, 1.0, 0.0)));

        topology_registry.connect_peers(opts.topology, ctx, opts.n_peers);

        send_message_to(
            ctx,
            peer1_idx,
            peer2_idx,
            Some(Box::new(ExampleMessage { sender: peer1_idx })),
       );

        ctx.run();
    }
}
