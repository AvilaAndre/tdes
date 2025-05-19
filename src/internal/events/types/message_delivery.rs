use std::cmp::Ordering;

use ordered_float::OrderedFloat;

use crate::internal::{
    context::Context,
    events::{Event, types::EventType},
    message::Message,
};

#[derive(Debug)]
pub struct MessageDeliveryEvent {
    timestamp: OrderedFloat<f64>,
    sender: usize,
    receiver: usize,
    message: Option<Box<dyn Message>>,
}

// This compares only the timestamps
impl PartialOrd for MessageDeliveryEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.timestamp.partial_cmp(&other.timestamp)
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
        sender: usize,
        receiver: usize,
        message: Option<Box<dyn Message>>,
    ) -> Self {
        Self {
            timestamp,
            sender,
            receiver,
            message,
        }
    }

    pub fn create(
        timestamp: OrderedFloat<f64>,
        sender: usize,
        recipient: usize,
        message: Option<Box<dyn Message>>,
    ) -> EventType {
        EventType::MessageDeliveryEvent(MessageDeliveryEvent::new(
            timestamp, sender, recipient, message,
        ))
    }
}

impl Event for MessageDeliveryEvent {
    fn timestamp(&self) -> OrderedFloat<f64> {
        self.timestamp
    }

    fn process(&mut self, ctx: &mut Context) {
        println!(
            "[{}] MessageDeliveryEvent from {} to {} triggered!",
            ctx.clock, self.sender, self.receiver
        );

        let receiver = ctx.peers.get(self.receiver);

        if receiver.is_some() {
            (receiver.unwrap().get_peer().on_message_receive)(ctx, self.receiver, self.message.take())
        } else {
            // TODO: Log that the receiver does not exist
        }
    }
}
