use ordered_float::OrderedFloat;

use crate::internal::{context::Context, events::Event};

use super::EventType;

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

    fn process(&self, ctx: &mut Context) {
        println!(
            "[{}]: SampleEvent triggered with value {}!",
            ctx.clock, self.value
        );
    }
}
