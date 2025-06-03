use std::collections::HashMap;

use crate::internal::core::{Context, builtins, log};

use super::Topology;

// Type alias for topology functions
type TopologyFn = fn(&mut Context, usize);

pub struct TopologyRegistry {
    topologies: HashMap<String, TopologyFn>,
}

impl TopologyRegistry {
    pub fn new() -> Self {
        Self {
            topologies: HashMap::new(),
        }
    }

    pub fn register<T: Topology>(&mut self) -> &mut Self {
        self.topologies.insert(T::name().to_string(), T::connect);

        self
    }

    pub fn list_topologies(&self) -> Vec<&str> {
        self.topologies
            .keys()
            .map(|val| val.as_str())
            .collect::<Vec<&str>>()
    }

    pub fn connect_peers(&self, topology_opt: Option<String>, ctx: &mut Context, n_peers: usize) {
        if let Some(name) = topology_opt {
            match self.topologies.get(&name) {
                Some(connect_fn) => {
                    log::global_info(format!("Connecting peers using the {name} topology."));
                    connect_fn(ctx, n_peers.min(ctx.peers.len()));
                }
                None => {
                    log::global_warn(format!("Topology '{}' not found", name));
                }
            }
        }
    }
}

impl Default for TopologyRegistry {
    fn default() -> Self {
        let mut a = Self::new();
        a.register::<builtins::topologies::FullTopology>();
        a.register::<builtins::topologies::StarTopology>();
        a.register::<builtins::topologies::RingTopology>();
        a
    }
}
