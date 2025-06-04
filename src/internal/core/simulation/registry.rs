use indexmap::IndexMap;

use crate::internal::core::log;
use crate::internal::simulator::Simulator;

use super::super::options::ExperimentOptions;
use super::{Context, Simulation};

// Type alias for simulation functions
type ScenarioFn = fn(&mut Context, &Simulator, ExperimentOptions);

pub struct SimulationRegistry {
    simulations: IndexMap<String, (ScenarioFn, &'static str)>,
}

impl SimulationRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self {
            simulations: IndexMap::new(),
        }
    }

    pub fn register<T: Simulation>(&mut self) -> &mut Self {
        let name = T::name().to_string();
        if !self.simulations.contains_key(&name) {
            self.simulations.insert(name, (T::start, T::description()));
        } else {
            log::global_warn(format!("A simulation named {name} alreay exists"));
        }
        self
    }

    pub fn run_simulation(
        &self,
        name: &str,
        ctx: &mut Context,
        simulator: &Simulator,
        opts: ExperimentOptions,
    ) -> Result<(), String> {
        match &opts.topology.name {
            Some(name) => log::global_info(format!("Topology selected from configuration: {name}")),
            None => log::global_warn("No topology selected from configuration."),
        }
        match &opts.arrival_time {
            Some(a) => log::global_info(format!(
                "Arrival time callback selected from configuration: {a}",
            )),
            None => log::global_warn("No arrival time callback selected from configuration."),
        }

        match self.simulations.get(name) {
            Some((simulation_fn, _)) => {
                simulation_fn(ctx, simulator, opts);
                Ok(())
            }
            None => Err(format!("Simulation '{name}' not found")),
        }
    }

    #[must_use]
    pub fn list(&self) -> Vec<(&str, &str)> {
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
