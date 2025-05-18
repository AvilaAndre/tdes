pub mod types;

use ordered_float::OrderedFloat;

use crate::Context;

pub trait Event {
    fn timestamp(&self) -> OrderedFloat<f64>;
    fn trigger(&self, ctx: &mut Context);
}
