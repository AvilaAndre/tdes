use ordered_float::OrderedFloat;

use crate::internal::{
    context::Context,
    events::{
        Event,
        types::{EventType, sample::SampleEvent},
    },
};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
pub struct TimerEvent {
    timestamp: OrderedFloat<f64>,
}

impl TimerEvent {
    pub fn new(timestamp: OrderedFloat<f64>) -> Self {
        Self { timestamp }
    }
}

impl Event for TimerEvent {
    fn timestamp(&self) -> OrderedFloat<f64> {
        self.timestamp
    }

    fn trigger(&self, ctx: &mut Context) {
        println!("[{}] TimerEvent triggered!", ctx.clock);

        ctx.add_event(EventType::SampleEvent(SampleEvent::new(
            ctx.clock + 1.0,
            10,
        )));
    }
}
