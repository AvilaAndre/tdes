mod types;

use enum_dispatch::enum_dispatch;
use ordered_float::OrderedFloat;

use super::Context;

#[enum_dispatch]
pub trait Event {
    fn timestamp(&self) -> OrderedFloat<f64>;
    fn process(&mut self, ctx: &mut Context);
}

pub use types::{
    EventType,
    message_delivery::MessageDeliveryEvent,
    timer::{Timer, TimerEvent},
};
