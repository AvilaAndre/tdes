use std::collections::HashMap;

use crate::internal::core::{
    Context,
    builtins::{
        self,
        topologies::{OneWayCustomTopology, TwoWayCustomTopology},
    },
    log,
};

use super::{Topology, TopologyInfo};

// Type alias for topology functions
type TopologyFn = fn(&mut Context, usize, Option<Vec<(usize, usize, Option<f64>)>>);

pub struct TopologyRegistry {
    topologies: HashMap<String, TopologyFn>,
}

impl TopologyRegistry {
    #[must_use]
    pub fn new() -> Self {
        let mut registry = Self {
            topologies: HashMap::new(),
        };
        registry.register::<OneWayCustomTopology>();
        registry.register::<TwoWayCustomTopology>();
        registry
    }

    pub fn register<T: Topology>(&mut self) -> &mut Self {
        // FIXME: Avoid having more than one topology of the same name
        self.topologies.insert(T::name().to_string(), T::connect);

        self
    }

    #[must_use]
    pub fn list(&self) -> Vec<&str> {
        self.topologies
            .keys()
            .map(String::as_str)
            .collect::<Vec<&str>>()
    }

    pub fn connect_peers(
        &self,
        ctx: &mut Context,
        topology_opt: Option<TopologyInfo>,
        n_peers: usize,
    ) {
        if let Some(topology) = topology_opt {
            let name = topology.name;

            match self.topologies.get(&name) {
                Some(connect_fn) => {
                    log::global_info(format!("Connecting peers using the {name} topology."));
                    connect_fn(ctx, n_peers.min(ctx.peers.len()), topology.list);
                }
                None => {
                    log::global_warn(format!("Topology '{name}' not found"));
                }
            }
        }
    }
}

impl Default for TopologyRegistry {
    fn default() -> Self {
        let mut registry = Self::new();
        registry
            .register::<builtins::topologies::FullTopology>()
            .register::<builtins::topologies::StarTopology>()
            .register::<builtins::topologies::RingTopology>();
        registry
    }
}
