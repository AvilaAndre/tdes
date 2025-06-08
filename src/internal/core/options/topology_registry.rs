use indexmap::IndexMap;

use super::{
    super::{
        Context,
        builtins::{
            self,
            topologies::{OneWayCustomTopology, TwoWayCustomTopology},
        },
        config::{ConnectionInfo, TopologyInfo},
        log,
    },
    Topology,
};

// Type alias for topology functions
type TopologyFn = fn(&mut Context, usize, Option<Vec<ConnectionInfo>>);

pub struct TopologyRegistry {
    topologies: IndexMap<String, TopologyFn>,
}

impl TopologyRegistry {
    #[must_use]
    pub fn new() -> Self {
        let mut registry = Self {
            topologies: IndexMap::new(),
        };
        registry.register::<OneWayCustomTopology>();
        registry.register::<TwoWayCustomTopology>();
        registry
    }

    pub fn register<T: Topology>(&mut self) -> &mut Self {
        let name = T::name().to_string();
        if !self.topologies.contains_key(&name) {
            self.topologies.insert(name, T::connect);
        } else {
            log::global_warn(format!("A topology named {name} alreay exists"));
        }
        self
    }

    #[must_use]
    pub fn list(&self) -> Vec<&str> {
        self.topologies
            .keys()
            .map(String::as_str)
            .collect::<Vec<&str>>()
    }

    pub fn connect_peers(&self, ctx: &mut Context, topology: TopologyInfo) {
        if let Some(name) = topology.name {
            match self.topologies.get(&name) {
                Some(connect_fn) => {
                    log::global_info(format!("Connecting peers using the {name} topology."));
                    connect_fn(
                        ctx,
                        topology.n_peers.min(ctx.peers.len()),
                        topology.connections,
                    );
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
