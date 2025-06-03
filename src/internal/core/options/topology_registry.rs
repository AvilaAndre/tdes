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
        // FIXME: use keys
        self.topologies
            .iter()
            .map(|(name, _)| name.as_str())
            .collect()
    }

    pub fn connect_peers(&self, topology_opt: Option<String>, ctx: &mut Context, n_peers: usize) {
        if let Some(name) = topology_opt {
            match self.topologies.get(&name) {
                Some(connect_fn) => {
                    log::global_info(format!("Connecting peers using the {name} topology."));
                    // FIXME: n_peers must be the same or less than the number of peers available.
                    connect_fn(ctx, n_peers);
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
