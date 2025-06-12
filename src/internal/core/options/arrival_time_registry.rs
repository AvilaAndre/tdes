use indexmap::IndexMap;

use crate::internal::core::{
    builtins::{
        self,
        arrival_times::{ConstantArrivalTime, DistanceBasedArrivalTime},
    },
    context::MessageDelayCallback,
    log,
};

use super::ArrivalTimeCallback;

pub struct ArrivalTimeRegistry {
    callbacks: IndexMap<String, MessageDelayCallback>,
}

impl ArrivalTimeRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self {
            callbacks: IndexMap::new(),
        }
    }

    pub fn register<A: ArrivalTimeCallback>(&mut self) -> &mut Self {
        let name = A::name().to_string();
        if !self.callbacks.contains_key(&name) {
            self.callbacks.insert(name, A::callback);
        } else {
            log::global_warn(format!(
                "A arrival time callback named {name} alreay exists"
            ));
        }
        self
    }

    #[must_use]
    pub fn list(&self) -> Vec<&str> {
        self.callbacks
            .keys()
            .map(String::as_str)
            .collect::<Vec<&str>>()
    }

    #[must_use]
    pub fn get_callback(&self, arrival_time_opt: Option<String>) -> MessageDelayCallback {
        if let Some(name) = arrival_time_opt {
            if let Some(callback_fn) = self.callbacks.get(&name) {
                log::global_info(format!("Arrival time callback '{name}' selected."));
                *callback_fn
            } else {
                log::global_warn(format!("Arrival time callback '{name}' not found"));
                builtins::arrival_times::ConstantArrivalTime::callback
            }
        } else {
            builtins::arrival_times::ConstantArrivalTime::callback
        }
    }
}

impl Default for ArrivalTimeRegistry {
    fn default() -> Self {
        let mut registry = Self::new();
        registry
            .register::<ConstantArrivalTime>()
            .register::<DistanceBasedArrivalTime>();
        registry
    }
}
