use std::collections::HashMap;

use crate::internal::core::{
    builtins::{
        self,
        arrival_time::{ConstantArrivalTime, DistanceBasedArrivalTime},
    },
    context::MessageDelayCallback,
    log,
};

use super::ArrivalTimeCallback;

pub struct ArrivalTimeRegistry {
    callbacks: HashMap<String, MessageDelayCallback>,
}

impl ArrivalTimeRegistry {
    pub fn new() -> Self {
        Self {
            callbacks: HashMap::new(),
        }
    }

    pub fn register<A: ArrivalTimeCallback>(&mut self) -> &mut Self {
        self.callbacks.insert(A::name().to_string(), A::callback);

        self
    }

    pub fn list(&self) -> Vec<&str> {
        self.callbacks
            .keys()
            .map(|val| val.as_str())
            .collect::<Vec<&str>>()
    }

    pub fn get_callback(&self, arrival_time_opt: Option<String>) -> MessageDelayCallback {
        if let Some(name) = arrival_time_opt {
            match self.callbacks.get(&name) {
                Some(callback_fn) => {
                    log::global_info(format!("Arrival time callback '{name}' selected."));
                    *callback_fn
                }
                None => {
                    log::global_warn(format!("Arrival time callback '{name}' not found"));
                    builtins::arrival_time::ConstantArrivalTime::callback
                }
            }
        } else {
            builtins::arrival_time::ConstantArrivalTime::callback
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
