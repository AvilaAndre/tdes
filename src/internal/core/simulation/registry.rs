use std::collections::HashMap;

use crate::internal::core::log;
use crate::internal::simulator::Simulator;

use super::super::options::ExperimentOptions;
use super::{Context, Simulation};

// Type alias for simulation functions
type ScenarioFn = fn(&mut Context, &Simulator, ExperimentOptions);

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
        simulator: &Simulator,
        opts: ExperimentOptions,
    ) -> Result<(), String> {
        match &opts.topology {
            Some(t) => log::global_info(format!("Topology selected from configuration: {}", t)),
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
