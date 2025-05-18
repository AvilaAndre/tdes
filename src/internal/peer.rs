use ordered_float::OrderedFloat;

use super::{context::Context, events::types::message_delivery::MessageDeliveryEvent};

#[derive(Clone)]
pub struct Peer {
    pub id: Option<usize>,
    pub position: (f64, f64, f64),
    pub on_message_receive: fn(ctx: &mut Context, receiver_id: usize) -> (),
}

fn default_on_message_receive(_ctx: &mut Context, _receiver_id: usize) {
    // TODO: Remove this println
    println!("Default on_message_receive called");
}

impl Peer {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            id: None,
            position: (x, y, z),
            on_message_receive: default_on_message_receive,
        }
    }

    pub fn with_on_message_receive(
        mut self,
        on_message_receive: fn(ctx: &mut Context, receiver_id: usize) -> (),
    ) -> Self {
        self.on_message_receive = on_message_receive;
        self
    }

    // TODO: This should be default behaviour
    pub fn send_message_to(&self, ctx: &mut Context, target_id: usize) -> bool {
        let Some(sender_id) = self.id else {
            return false;
        };

        ctx.add_event(MessageDeliveryEvent::create(
            OrderedFloat(5.0),
            sender_id,
            target_id,
        ));

        true
    }
}

pub trait CustomPeer {
    fn get_peer(&self) -> &Peer;
    fn get_peer_mut(&mut self) -> &mut Peer;
}
