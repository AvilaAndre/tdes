use std::cmp::Ordering;

use ordered_float::OrderedFloat;

use crate::internal::core::{
    Context, Message,
    events::{Event, event::impl_timestamp_id_ordering, types::EventType},
    log,
};

#[derive(Debug)]
pub struct MessageDeliveryEvent {
    id: u64,
    timestamp: OrderedFloat<f64>,
    sender: usize,
    receiver: usize,
    message: Box<dyn Message>,
}

impl_timestamp_id_ordering!(MessageDeliveryEvent);

impl MessageDeliveryEvent {
    #[must_use]
    pub fn new(
        timestamp: OrderedFloat<f64>,
        sender: usize,
        receiver: usize,
        message: Box<dyn Message>,
    ) -> Self {
        Self {
            id: 0,
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
