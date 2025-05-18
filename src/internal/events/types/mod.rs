pub mod message_delivery;
pub mod sample;
pub mod timer;

use enum_dispatch::enum_dispatch;
use std::cmp::Ordering;

use message_delivery::MessageDeliveryEvent;
use ordered_float::OrderedFloat;
use sample::SampleEvent;
use timer::TimerEvent;

use crate::internal::context::Context;

use super::Event;

#[enum_dispatch(EventType)]
#[derive(Debug, Eq, Clone, Copy)]
pub enum EventType {
    SampleEvent,
    TimerEvent,
    MessageDeliveryEvent,
}

impl Ord for EventType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.timestamp().cmp(&other.timestamp())
    }
}
impl PartialOrd for EventType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for EventType {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp() == other.timestamp()
    }
}

impl Event for EventType {
    fn timestamp(&self) -> OrderedFloat<f64> {
        let event: &dyn Event = match self {
            EventType::SampleEvent(event) => event,
            EventType::TimerEvent(event) => event,
            EventType::MessageDeliveryEvent(event) => event,
        };
        event.timestamp()
    }

    fn trigger(&self, ctx: &mut Context) {
        let event: &dyn Event = match self {
            EventType::SampleEvent(event) => event,
            EventType::TimerEvent(event) => event,
            EventType::MessageDeliveryEvent(event) => event,
        };
        event.trigger(ctx)
    }
}
