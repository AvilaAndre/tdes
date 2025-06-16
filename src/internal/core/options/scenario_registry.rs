use indexmap::IndexMap;

use super::super::{
    super::Simulator, Context, log, options::ExperimentOptions, options::traits::Scenario,
};

// Type alias for scenario functions
type ScenarioFn = fn(&mut Context, &Simulator, ExperimentOptions);

pub struct ScenarioRegistry {
    scenarios: IndexMap<String, (ScenarioFn, &'static str)>,
}

impl ScenarioRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self {
            scenarios: IndexMap::new(),
        }
    }

    pub fn register<S: Scenario>(&mut self) -> &mut Self {
        let name = S::name().to_string();
        if self.scenarios.contains_key(&name) {
            log::global_warn(format!("A scenario named {name} already exists"));
        } else {
            self.scenarios.insert(name, (S::start, S::description()));
        }
        self
    }

    pub fn run_scenario(
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

        match self.scenarios.get(name) {
            Some((scenario_fn, _)) => {
                scenario_fn(ctx, simulator, opts);
                Ok(())
            }
            None => Err(format!("Scenario '{name}' not found")),
        }
    }

    #[must_use]
    pub fn list(&self) -> Vec<(&str, &str)> {
        self.scenarios
            .iter()
            .map(|(name, (_, desc))| (name.as_str(), *desc))
            .collect()
    }
}

impl Default for ScenarioRegistry {
    fn default() -> Self {
        Self::new()
    }
}
