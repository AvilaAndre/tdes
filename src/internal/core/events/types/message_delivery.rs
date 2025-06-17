use std::cmp::Ordering;

use ordered_float::OrderedFloat;

use crate::internal::core::{
    Context, Message,
    events::{Event, types::EventType},
    log,
};

#[derive(Debug)]
pub struct MessageDeliveryEvent {
    timestamp: OrderedFloat<f64>,
    sender: usize,
    receiver: usize,
    message: Box<dyn Message>,
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
    #[must_use]
    pub fn new(
        timestamp: OrderedFloat<f64>,
        sender: usize,
        receiver: usize,
        message: Box<dyn Message>,
    ) -> Self {
        Self {
            timestamp,
            sender,
            receiver,
            message,
        }
    }

    #[must_use]
    pub fn create(
        timestamp: OrderedFloat<f64>,
        sender: usize,
        receiver: usize,
        message: impl Message + 'static,
    ) -> EventType {
        EventType::MessageDeliveryEvent(MessageDeliveryEvent::new(
            timestamp,
            sender,
            receiver,
            Box::new(message),
        ))
    }

    #[must_use]
    pub fn create_boxed(
        timestamp: OrderedFloat<f64>,
        sender: usize,
        receiver: usize,
        message: Box<dyn Message>,
    ) -> EventType {
        EventType::MessageDeliveryEvent(MessageDeliveryEvent::new(
            timestamp, sender, receiver, message,
        ))
    }
}

impl Event for MessageDeliveryEvent {
    fn timestamp(&self) -> OrderedFloat<f64> {
        self.timestamp
    }

    fn process(&mut self, ctx: &mut Context) {
        if let Some(receiver) = ctx.peers.get(self.receiver) {
            if receiver.is_alive() {
                (receiver.get_peer().on_message_receive)(
                    ctx,
                    self.sender,
                    self.receiver,
                    self.message.as_ref(),
                );
            } else {
                log::warn(
                    ctx,
                    format!(
                        "MessageDeliveryEvent not processed because receiver {} is dead",
                        self.receiver
                    ),
                );
            }
        } else {
            log::warn(
                ctx,
                format!(
                    "MessageDeliveryEvent receiver {} does not exist",
                    self.receiver
                ),
            );
        }
    }
}
