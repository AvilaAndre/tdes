use ordered_float::OrderedFloat;

use crate::internal::{
    context::Context,
    events::{Event, types::EventType},
};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
pub struct MessageDeliveryEvent {
    timestamp: OrderedFloat<f64>,
    sender: usize,
    receiver: usize,
}

impl MessageDeliveryEvent {
    pub fn new(timestamp: OrderedFloat<f64>, sender: usize, recepient: usize) -> Self {
        Self {
            timestamp,
            sender,
            receiver: recepient,
        }
    }

    pub fn create(timestamp: OrderedFloat<f64>, sender: usize, recepient: usize) -> EventType {
        EventType::MessageDeliveryEvent(MessageDeliveryEvent::new(timestamp, sender, recepient))
    }
}

impl Event for MessageDeliveryEvent {
    fn timestamp(&self) -> OrderedFloat<f64> {
        self.timestamp
    }

    fn trigger(&self, ctx: &mut Context) {
        println!(
            "[{}] MessageDeliveryEvent from {} to {} triggered!",
            ctx.clock, self.sender, self.receiver
        );
    }
}
