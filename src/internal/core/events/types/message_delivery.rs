use std::cmp::Ordering;

use ordered_float::OrderedFloat;

use crate::internal::core::{
    events::{types::EventType, Event}, log, Context, Message
};

#[derive(Debug)]
pub struct MessageDeliveryEvent {
    timestamp: OrderedFloat<f64>,
    receiver: usize,
    message: Option<Box<dyn Message>>,
}

// This compares only the timestamps
impl PartialOrd for MessageDeliveryEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MessageDeliveryEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        self.timestamp.total_cmp(&other.timestamp)
    }
}

impl PartialEq for MessageDeliveryEvent {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
    }
}

impl Eq for MessageDeliveryEvent {}

impl MessageDeliveryEvent {
    pub fn new(
        timestamp: OrderedFloat<f64>,
        receiver: usize,
        message: Option<Box<dyn Message>>,
    ) -> Self {
        Self {
            timestamp,
            receiver,
            message,
        }
    }

    pub fn create(
        timestamp: OrderedFloat<f64>,
        recipient: usize,
        message: Option<Box<dyn Message>>,
    ) -> EventType {
        EventType::MessageDeliveryEvent(MessageDeliveryEvent::new(timestamp, recipient, message))
    }
}

impl Event for MessageDeliveryEvent {
    fn timestamp(&self) -> OrderedFloat<f64> {
        self.timestamp
    }

    fn process(&mut self, ctx: &mut Context) {
        if let Some(receiver) = ctx.peers.get(self.receiver) {
            (receiver.get_peer().on_message_receive)(ctx, self.receiver, self.message.take())
        } else {
            log::warn(ctx, format!("MessageDeliveryEvent receiver {} does not exist", self.receiver));
        }
    }
}
