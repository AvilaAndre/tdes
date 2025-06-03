use std::collections::HashMap;

use crate::internal::core::options::ArrivalTimeRegistry;

use super::super::options::ExperimentOptions;
use super::{Context, Simulation, TopologyRegistry};

// Type alias for simulation functions
type ScenarioFn = fn(&mut Context, &TopologyRegistry, &ArrivalTimeRegistry, ExperimentOptions);

pub struct SimulationRegistry {
    simulations: HashMap<String, (ScenarioFn, &'static str)>,
}

impl SimulationRegistry {
    pub fn new() -> Self {
        Self {
            simulations: HashMap::new(),
        }
    }

    pub fn register<T: Simulation>(&mut self) -> &mut Self {
        self.simulations
            .insert(T::name().to_string(), (T::start, T::description()));

        self
    }

    pub fn run_simulation(
        &self,
        name: &str,
        ctx: &mut Context,
        topology_registry: &TopologyRegistry,
        arrival_time_registry: &ArrivalTimeRegistry,
        opts: ExperimentOptions,
    ) -> Result<(), String> {
        match self.simulations.get(name) {
            Some((simulation_fn, _)) => {
                simulation_fn(ctx, topology_registry, arrival_time_registry, opts);
                Ok(())
            }
            None => Err(format!("Simulation '{}' not found", name)),
        }
    }

    pub fn list_simulations(&self) -> Vec<(&str, &str)> {
        self.simulations
            .iter()
            .map(|(name, (_, desc))| (name.as_str(), *desc))
            .collect()
    }
}

impl Default for SimulationRegistry {
    fn default() -> Self {
        Self::new()
    }
}
