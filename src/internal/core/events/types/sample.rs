use ordered_float::OrderedFloat;

use crate::internal::core::log;

use super::{Context, Event, EventType};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
pub struct SampleEvent {
    timestamp: OrderedFloat<f64>,
    value: u64,
}

impl SampleEvent {
    pub fn new(timestamp: OrderedFloat<f64>, value: u64) -> Self {
        Self { timestamp, value }
    }

    pub fn create(timestamp: OrderedFloat<f64>, value: u64) -> EventType {
        EventType::SampleEvent(SampleEvent::new(timestamp, value))
    }
}

impl Event for SampleEvent {
    fn timestamp(&self) -> OrderedFloat<f64> {
        self.timestamp
    }

    fn process(&mut self, ctx: &mut Context) {
        log::debug(
            ctx,
            format!("SampleEvent triggered with value {}!", self.value),
        );
    }
}
