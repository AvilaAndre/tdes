use std::{cmp::Ordering, fmt::Debug};

use downcast_rs::{Downcast, impl_downcast};
use ordered_float::OrderedFloat;

use crate::internal::{
    context::Context,
    events::{Event, types::EventType},
};

pub trait Timer: Debug + Downcast {
    fn fire(&self, ctx: &mut Context);
}
impl_downcast!(Timer);

#[derive(Debug)]
pub struct TimerEvent {
    timestamp: OrderedFloat<f64>,
    timer: Box<dyn Timer>,
}

// This compares only the timestamps
impl PartialOrd for TimerEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TimerEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        self.timestamp.total_cmp(&other.timestamp)
    }
}

impl PartialEq for TimerEvent {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
    }
}

impl Eq for TimerEvent {}

impl TimerEvent {
    pub fn new(timestamp: OrderedFloat<f64>, timer: Box<dyn Timer>) -> Self {
        Self { timestamp, timer }
    }

    pub fn create(timestamp: OrderedFloat<f64>, timer: Box<dyn Timer>) -> EventType {
        EventType::TimerEvent(TimerEvent::new(timestamp, timer))
    }
}

impl Event for TimerEvent {
    fn timestamp(&self) -> OrderedFloat<f64> {
        self.timestamp
    }

    fn process(&mut self, ctx: &mut Context) {
        self.timer.fire(ctx);
    }
}
