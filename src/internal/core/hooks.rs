use super::Context;

pub type CustomOnFinishHook = Box<dyn Fn(&mut Context)>;
pub type CustomFinishConditionHook = Box<dyn Fn(&mut Context) -> bool>;

pub struct SimulationHooks {
    /// It is called when the simulation finishes.
    pub on_simulation_finish: CustomOnFinishHook,
    /// If it returns true, the simulation loop finishes.
    pub finish_condition: CustomFinishConditionHook,
}

impl Default for SimulationHooks {
    fn default() -> Self {
        Self {
            on_simulation_finish: Box::new(|_ctx| {}),
            finish_condition: Box::new(|_ctx| false),
        }
    }
}

impl SimulationHooks {
    pub fn set_on_simulation_finish_hook(&mut self, hook: CustomOnFinishHook) -> &mut Self {
        self.on_simulation_finish = hook;
        self
    }

    pub fn set_finish_condition(&mut self, hook: CustomFinishConditionHook) -> &mut Self {
        self.finish_condition = hook;
        self
    }
}
