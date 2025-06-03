use super::Context;

pub mod topology_registry;

pub use topology_registry::TopologyRegistry;

pub struct SimulationOptions {
    pub n_peers: usize,
    pub topology: Option<String>,
}

pub trait Topology {
    fn name() -> &'static str
    where
        Self: Sized;

    fn connect(ctx: &mut Context, n_peers: usize);
}
