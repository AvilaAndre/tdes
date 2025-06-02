pub mod message_delivery;
pub mod sample;
pub mod timer;

use enum_dispatch::enum_dispatch;
use ordered_float::OrderedFloat;
use std::cmp::Ordering;

use message_delivery::MessageDeliveryEvent;
use sample::SampleEvent;
use timer::TimerEvent;

use super::{Context, Event};

#[enum_dispatch(EventType)]
#[derive(Debug, Eq)]
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

    fn process(&mut self, ctx: &mut Context) {
        let event: &mut dyn Event = match self {
            EventType::SampleEvent(event) => event,
            EventType::TimerEvent(event) => event,
            EventType::MessageDeliveryEvent(event) => event,
        };
        event.process(ctx)
    }
}
