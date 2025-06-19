use enum_dispatch::enum_dispatch;
use ordered_float::OrderedFloat;
use std::cmp::Ordering;

use crate::internal::core::{
    Context,
    events::{Event, MessageDeliveryEvent, TimerEvent},
};

#[enum_dispatch(EventType)]
#[derive(Debug, Eq)]
pub enum EventType {
    TimerEvent,
    MessageDeliveryEvent,
}

impl Ord for EventType {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.timestamp().total_cmp(&other.timestamp()) {
            Ordering::Equal => self.id().cmp(&other.id()),
            other_ordering => other_ordering,
        }
    }
}
impl PartialOrd for EventType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for EventType {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp() == other.timestamp() && self.id() == other.id()
    }
}

impl Event for EventType {
    fn id(&self) -> u64 {
        let event: &dyn Event = match self {
            EventType::TimerEvent(event) => event,
            EventType::MessageDeliveryEvent(event) => event,
        };
        event.id()
    }

    fn set_id(&mut self, id: u64) {
        let event: &mut dyn Event = match self {
            EventType::TimerEvent(event) => event,
            EventType::MessageDeliveryEvent(event) => event,
        };
        event.set_id(id)
    }

    fn timestamp(&self) -> OrderedFloat<f64> {
        let event: &dyn Event = match self {
            EventType::TimerEvent(event) => event,
            EventType::MessageDeliveryEvent(event) => event,
        };
        event.timestamp()
    }

    fn process(&mut self, ctx: &mut Context) {
        let event: &mut dyn Event = match self {
            EventType::TimerEvent(event) => event,
            EventType::MessageDeliveryEvent(event) => event,
        };
        event.process(ctx);
    }
}
