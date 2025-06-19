use std::{cmp::Ordering, fmt::Debug};

use downcast_rs::{Downcast, impl_downcast};
use ordered_float::OrderedFloat;

use crate::internal::core::{
    Context,
    events::{Event, event::impl_timestamp_id_ordering, types::EventType},
};

pub trait Timer: Debug + Downcast {
    fn fire(&self, ctx: &mut Context);
}
impl_downcast!(Timer);

#[derive(Debug)]
pub struct TimerEvent {
    id: u64,
    timestamp: OrderedFloat<f64>,
    timer: Box<dyn Timer>,
}

impl_timestamp_id_ordering!(TimerEvent);

impl TimerEvent {
    #[must_use]
    pub fn new(timestamp: OrderedFloat<f64>, timer: Box<dyn Timer>) -> Self {
        Self {
            id: 0,
            timestamp,
            timer,
        }
    }

    #[must_use]
    pub fn create(timestamp: OrderedFloat<f64>, timer: Box<dyn Timer>) -> EventType {
        EventType::TimerEvent(TimerEvent::new(timestamp, timer))
    }
}

impl Event for TimerEvent {
    fn id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id
    }

    fn timestamp(&self) -> OrderedFloat<f64> {
        self.timestamp
    }

    fn process(&mut self, ctx: &mut Context) {
        self.timer.fire(ctx);
    }
}
